use std::net::SocketAddr;

use futures_util::{SinkExt, StreamExt};
use protocol::{ConnectionMessage, Header, Message, serde_json, uuid::Uuid};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::unbounded_channel,
};
use tokio_tungstenite::{accept_async, tungstenite};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8889").await.unwrap();
    println!("WebSocket server listening on ws://0.0.0.0:8889");

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_initial_connection(stream, addr));
    }
}

pub async fn handle_initial_connection(stream: TcpStream, addr: SocketAddr) {
    let result = accept_async(stream).await;
    let mut ws_stream = match result {
        Ok(ws_stream) => {
            println!("New WebSocket connection (Address:{addr})");
            ws_stream
        }
        Err(e) => {
            eprintln!("Failed to accept WebSocket connection (Address:{addr}): {e}");
            return;
        }
    };

    let message = Message {
        header: Header::Connection,
        json: serde_json::to_string(&ConnectionMessage {
            uuid: Uuid::new_v4(),
            username: "Test".to_string(),
        })
        .unwrap(),
    };

    let result = ws_stream
        .send(tungstenite::Message::text(
            serde_json::to_string(&message).unwrap(),
        ))
        .await;
    if let Err(e) = result {
        eprintln!("Failed to send message to WebSocket (Address:{addr}): {e}");
        return;
    }

    // Test echo server.
    let (tx, mut rx) = unbounded_channel::<String>();
    let (mut write, mut read) = ws_stream.split();
    let write_task = tokio::spawn(async move {
        while let Some(s) = rx.recv().await {
            let result = write.send(tungstenite::Message::text(s)).await;
            if let Err(e) = result {
                eprintln!("Failed to send message to WebSocket (Address:{addr}): {e}");
                return;
            }
        }
    });

    while let Some(result) = read.next().await {
        let message = match result {
            Ok(message) => message,
            Err(e) => {
                eprintln!("Failed to read message from WebSocket (Address:{addr}): {e}");
                break;
            }
        };

        if let tungstenite::Message::Text(s) = message {
            tx.send(s.to_string()).unwrap();
        }
    }

    drop(tx);
    write_task.into_future().await.unwrap();
}
