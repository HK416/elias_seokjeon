pub mod init;
pub mod matching;
pub mod title;

use std::{collections::VecDeque, mem, net::SocketAddr};

use crossbeam_queue::SegQueue;
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use protocol::{Packet, serde_json, uuid::Uuid};
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

pub fn next_state(
    uuid: Uuid,
    state: State,
    ws_stream: WebSocketStream<TcpStream>,
    addr: SocketAddr,
) {
    match state {
        State::Title => tokio::spawn(title::update(uuid, addr, ws_stream)),
        State::Matching => tokio::spawn(matching::regist(uuid, addr, ws_stream)),
    };
}
