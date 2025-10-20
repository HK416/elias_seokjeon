mod stream;

use std::{net::SocketAddr, time::Duration};

use futures_util::{SinkExt, StreamExt};
use protocol::{Packet, serde_json, uuid::Uuid};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::unbounded_channel,
    time::{Instant, interval_at},
};
use tokio_tungstenite::{
    WebSocketStream, accept_async,
    tungstenite::{self, Message},
};

use crate::stream::{StreamPollResult, poll_stream_nonblocking};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8889").await.unwrap();
    println!("WebSocket server listening on ws://0.0.0.0:8889");

    while let Ok((stream, addr)) = listener.accept().await {
        let result = accept_async(stream).await;
        let ws_stream = match result {
            Ok(ws_stream) => {
                println!("New WebSocket connection (Address:{addr})");
                ws_stream
            }
            Err(e) => {
                eprintln!("Failed to accept WebSocket connection (Address:{addr}): {e}");
                return;
            }
        };

        tokio::spawn(handle_initial_connection(ws_stream, addr));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    None,
    Title,
    Matching,
}

pub async fn handle_initial_connection(
    mut ws_stream: WebSocketStream<TcpStream>,
    addr: SocketAddr,
) {
    #[cfg(not(feature = "no-debuging-log"))]
    println!("Current State: Init");

    let packet = Packet::Connection {
        uuid: Uuid::new_v4(),
        username: "Test".into(),
    };

    let s = serde_json::to_string(&packet).unwrap();
    let item = Message::text(s);
    if let Err(e) = ws_stream.send(item).await {
        println!("WebSocket disconnected (Address:{addr}): {e}");
    }

    next_state(State::Title, ws_stream, addr);
}

pub async fn handle_title_connection(ws_stream: WebSocketStream<TcpStream>, addr: SocketAddr) {
    #[cfg(not(feature = "no-debuging-log"))]
    println!("Current State: Title");

    let mut state = State::Title;
    let (tx, mut rx) = unbounded_channel::<Packet>();
    let (mut write, mut read) = ws_stream.split();
    let write_task = tokio::spawn(async move {
        while let Some(packet) = rx.recv().await {
            let s = serde_json::to_string(&packet).unwrap();
            let result = write.send(tungstenite::Message::text(s)).await;
            if let Err(e) = result {
                eprintln!("Failed to send message to WebSocket (Address:{addr}): {e}");
                return write;
            }
        }
        write
    });

    while let Some(result) = read.next().await {
        let message = match result {
            Ok(message) => message,
            Err(e) => {
                println!("WebSocket disconnected (Address:{addr}): {e}");
                return;
            }
        };

        if let Message::Text(s) = message
            && let Ok(packet) = serde_json::from_str::<Packet>(&s)
        {
            match packet {
                Packet::EnterGame => {
                    state = State::Matching;
                    break;
                }
                _ => { /* empty */ }
            }
        }
    }

    drop(tx);
    let other = write_task.await.unwrap();
    let ws_stream = read.reunite(other).unwrap();

    match state {
        State::Matching => tokio::spawn(handle_matching_connection(ws_stream, addr)),
        _ => return,
    };
}

pub async fn handle_matching_connection(ws_stream: WebSocketStream<TcpStream>, addr: SocketAddr) {
    #[cfg(not(feature = "no-debuging-log"))]
    println!("Current State: Matching");

    let mut state = State::Matching;
    let (tx, mut rx) = unbounded_channel::<String>();
    let (mut write, mut read) = ws_stream.split();
    let write_task = tokio::spawn(async move {
        while let Some(s) = rx.recv().await {
            let result = write.send(tungstenite::Message::text(s)).await;
            if let Err(e) = result {
                eprintln!("Failed to send message to WebSocket (Address:{addr}): {e}");
                return write;
            }
        }
        write
    });

    const TICK: u64 = 1000 / 15;
    const PERIOD: Duration = Duration::from_millis(TICK);
    const MAX_MATCHING_TIME: u16 = 15000;
    let mut previous_instant = Instant::now();
    let mut interval = interval_at(previous_instant, PERIOD);
    let mut millis = MAX_MATCHING_TIME;
    'update: while millis > 0 {
        let instant = interval.tick().await;
        let elapsed = instant
            .saturating_duration_since(previous_instant)
            .as_millis();
        previous_instant = instant;
        millis = millis.saturating_sub(elapsed as u16);
        println!("millis: {}", millis);

        let packet = Packet::MatchingStatus { millis };
        tx.send(serde_json::to_string(&packet).unwrap()).unwrap();

        loop {
            match poll_stream_nonblocking(&mut read) {
                StreamPollResult::Pending => break,
                StreamPollResult::Item(message) => {
                    if let Message::Text(s) = message
                        && let Ok(packet) = serde_json::from_str::<Packet>(&s)
                    {
                        match packet {
                            Packet::CancelGame => {
                                state = State::Title;
                                break 'update;
                            }
                            _ => { /* empty */ }
                        }
                    }
                }
                StreamPollResult::Error(e) => {
                    println!("WebSocket disconnected (Address:{addr}): {e}");
                    state = State::None;
                    break 'update;
                }
                StreamPollResult::Closed => {
                    println!("WebSocket disconnected (Address:{addr})");
                    state = State::None;
                    break 'update;
                }
            }
        }
    }

    drop(tx);
    let other = write_task.await.unwrap();
    let ws_stream = read.reunite(other).unwrap();
    next_state(state, ws_stream, addr);
}

pub fn next_state(state: State, ws_stream: WebSocketStream<TcpStream>, addr: SocketAddr) {
    match state {
        State::None => return,
        State::Title => tokio::spawn(handle_title_connection(ws_stream, addr)),
        State::Matching => tokio::spawn(handle_matching_connection(ws_stream, addr)),
    };
}
