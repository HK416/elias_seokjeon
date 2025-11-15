pub mod init;
pub mod matching;
pub mod prepare;
pub mod sync;
pub mod title;

use std::{
    collections::VecDeque,
    fmt, mem,
    net::SocketAddr,
    sync::atomic::{AtomicUsize, Ordering as MemOrdering},
};

use crossbeam_queue::SegQueue;
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use protocol::{DEF_SCORE, Hero, MAX_SCORE, Packet, Player, rand, serde_json, uuid::Uuid};
use tokio::{
    net::TcpStream,
    sync::mpsc::{UnboundedSender, unbounded_channel},
    task::JoinHandle,
    time::{self, Duration, Instant},
};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

use crate::stream::{StreamPollResult, poll_stream_nonblocking};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Title,
    Matching,
}

pub struct Session {
    uuid: Uuid,
    name: String,
    hero: Hero,
    score: u16,
    addr: SocketAddr,
    read: SplitStream<WebSocketStream<TcpStream>>,
    tx: UnboundedSender<Packet>,
    _write_task: JoinHandle<SplitSink<WebSocketStream<TcpStream>, Message>>,
}

impl fmt::Debug for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Player)).field(&self.addr).finish()
    }
}

pub fn next_state(state: State, session: Session) {
    match state {
        State::Title => tokio::spawn(title::update(session)),
        State::Matching => tokio::spawn(matching::regist(session)),
    };
}
