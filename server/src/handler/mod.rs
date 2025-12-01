pub mod in_game;
pub mod init;
pub mod matching;
pub mod prepare;
pub mod sync;
pub mod title;

use std::{
    collections::VecDeque,
    f32::consts::TAU,
    fmt, mem,
    net::SocketAddr,
    sync::atomic::{AtomicUsize, Ordering as MemOrdering},
};

use crossbeam_queue::SegQueue;
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use glam::Vec2;
use protocol::{
    DEF_SCORE, Hero, LEFT_END_ANGLE, LEFT_PLAYER_POS_Y, LEFT_START_ANGLE, LEFT_THROW_POS_X,
    LEFT_THROW_POS_Y, MAX_CTRL_TIME, MAX_HEALTH, MAX_PLAY_TIME, MAX_SCORE, Packet, Player,
    RIGHT_END_ANGLE, RIGHT_START_ANGLE, RIGHT_THROW_POS_X, RIGHT_THROW_POS_Y, THROW_END_TIME,
    THROW_POWER, WIND_POWER, WORLD_MAX_X, WORLD_MIN_X, rand, serde_json, uuid::Uuid,
};
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
