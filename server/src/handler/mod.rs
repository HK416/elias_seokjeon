pub mod init;
pub mod matching;
pub mod title;

use std::{collections::VecDeque, fmt, mem, net::SocketAddr};

use crossbeam_queue::SegQueue;
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use protocol::{Hero, Packet, rand, serde_json, uuid::Uuid};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tokio::task::JoinHandle;
use tokio::time::{Duration, Instant, interval};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

use crate::stream::{StreamPollResult, poll_stream_nonblocking};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Title,
    Matching,
}

pub struct Player {
    uuid: Uuid,
    name: String,
    hero: Hero,
    addr: SocketAddr,
    read: SplitStream<WebSocketStream<TcpStream>>,
    tx: UnboundedSender<Packet>,
    _write_task: JoinHandle<SplitSink<WebSocketStream<TcpStream>, Message>>,
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Player)).field(&self.addr).finish()
    }
}

pub fn next_state(state: State, player: Player) {
    match state {
        State::Title => tokio::spawn(title::update(player)),
        State::Matching => tokio::spawn(matching::regist(player)),
    };
}
