pub mod in_game;
pub mod init;
pub mod matching;
pub mod prepare;
pub mod sync;
pub mod title;

use std::{
    any::Any,
    collections::VecDeque,
    f32::consts::{PI, TAU},
    fmt, mem,
    net::SocketAddr,
    ops::RangeInclusive,
    sync::atomic::{AtomicUsize, Ordering as MemOrdering},
};

use crossbeam_queue::SegQueue;
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use glam::{FloatExt, Vec2};
use protocol::{
    COLLIDER_DATA, GRAVITY, Hero, LEFT_END_ANGLE, LEFT_PLAYER_POS_X, LEFT_PLAYER_POS_Y,
    LEFT_START_ANGLE, LEFT_THROW_POS_X, LEFT_THROW_POS_Y, MAX_CTRL_TIME, MAX_HEALTH_COUNT,
    MAX_PLAY_TIME, MAX_POINT, PROJECTILE_SIZE, Packet, PlayData, RIGHT_END_ANGLE,
    RIGHT_PLAYER_POS_X, RIGHT_PLAYER_POS_Y, RIGHT_START_ANGLE, RIGHT_THROW_POS_X,
    RIGHT_THROW_POS_Y, RankItem, THROW_END_TIME, THROW_POWER, WIND_POWER, WORLD_MAX_X, WORLD_MIN_X,
    rand, serde_json, uuid::Uuid,
};
use rand::seq::IndexedRandom;
use redis::{AsyncTypedCommands, Script, aio::MultiplexedConnection};
use tokio::{
    net::TcpStream,
    sync::mpsc::{UnboundedSender, unbounded_channel},
    task::JoinHandle,
    time::{self, Duration, Instant},
};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

use crate::{
    DRAWS_KEY, EXPIRE, LEADER_BOARD_KEY, LOSSES_KEY, NAME_KEY, WINS_KEY, get_name_table,
    stream::{StreamPollResult, poll_stream_nonblocking},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Title,
    Matching,
}

pub trait Session: fmt::Debug + Send + Sync {
    fn uuid(&self) -> Option<Uuid>;
    fn name(&self) -> &str;
    fn hero(&self) -> Hero;
    fn win(&self) -> u16;
    fn lose(&self) -> u16;
    fn increase_win(&mut self);
    fn increase_lose(&mut self);
    fn increase_draw(&mut self);
    fn reader(&mut self) -> Option<&mut SplitStream<WebSocketStream<TcpStream>>>;
    fn sender(&self) -> Option<&UnboundedSender<Packet>>;
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send>;
}

pub struct Player {
    uuid: Uuid,
    name: String,
    hero: Hero,
    win: u16,
    lose: u16,
    draw: u16,
    addr: SocketAddr,
    read: SplitStream<WebSocketStream<TcpStream>>,
    tx: UnboundedSender<Packet>,
    write_task: JoinHandle<SplitSink<WebSocketStream<TcpStream>, Message>>,
}

impl Player {
    pub fn new(
        uuid: Uuid,
        hero: Hero,
        name: String,
        addr: SocketAddr,
        ws_stream: WebSocketStream<TcpStream>,
    ) -> Self {
        let (tx, mut rx) = unbounded_channel::<Packet>();
        let (mut write, read) = ws_stream.split();
        let write_task = tokio::spawn(async move {
            while let Some(s) = rx.recv().await {
                let s = serde_json::to_string(&s).unwrap();
                let result = write.send(Message::text(s)).await;
                if let Err(e) = result {
                    eprintln!("Failed to send message to WebSocket (Address:{addr}): {e}");
                    return write;
                }
            }
            write
        });

        Self {
            uuid,
            name,
            hero,
            win: 0,
            lose: 0,
            draw: 0,
            addr,
            read,
            tx,
            write_task,
        }
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        self.write_task.abort();
    }
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Player)).field(&self.addr).finish()
    }
}

impl Session for Player {
    fn uuid(&self) -> Option<Uuid> {
        Some(self.uuid)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn hero(&self) -> Hero {
        self.hero
    }

    fn win(&self) -> u16 {
        self.win
    }

    fn lose(&self) -> u16 {
        self.lose
    }

    fn increase_win(&mut self) {
        self.win = (self.win + 1).min(MAX_POINT);
    }

    fn increase_lose(&mut self) {
        self.lose = (self.lose + 1).min(MAX_POINT);
    }

    fn increase_draw(&mut self) {
        self.draw = (self.draw + 1).min(MAX_POINT);
    }

    fn reader(&mut self) -> Option<&mut SplitStream<WebSocketStream<TcpStream>>> {
        Some(&mut self.read)
    }

    fn sender(&self) -> Option<&UnboundedSender<Packet>> {
        Some(&self.tx)
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any + Send> {
        self
    }
}

pub struct Bot {
    name: String,
    hero: Hero,
    win: u16,
    lose: u16,
}

impl Bot {
    pub fn new() -> Self {
        let hero: Hero = rand::random();
        let prefix = get_name_table().choose(&mut rand::rng()).unwrap();

        Self {
            name: format!("{} {}", prefix, hero),
            hero,
            win: rand::random_range(0..=3),
            lose: rand::random_range(0..=3),
        }
    }
}

impl From<Box<dyn Session>> for Bot {
    fn from(value: Box<dyn Session>) -> Self {
        Self {
            name: value.name().to_string(),
            hero: value.hero(),
            win: value.win(),
            lose: value.lose(),
        }
    }
}

impl From<&dyn Session> for Bot {
    fn from(value: &dyn Session) -> Self {
        Self {
            name: value.name().to_string(),
            hero: value.hero(),
            win: value.win(),
            lose: value.lose(),
        }
    }
}

impl fmt::Debug for Bot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Bot)).field(&self.name).finish()
    }
}

impl Session for Bot {
    fn uuid(&self) -> Option<Uuid> {
        None
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn hero(&self) -> Hero {
        self.hero
    }

    fn win(&self) -> u16 {
        self.win
    }

    fn lose(&self) -> u16 {
        self.lose
    }

    fn increase_win(&mut self) {
        /* empty */
    }

    fn increase_lose(&mut self) {
        /* empty */
    }

    fn increase_draw(&mut self) {
        /* empty */
    }

    fn reader(&mut self) -> Option<&mut SplitStream<WebSocketStream<TcpStream>>> {
        None
    }

    fn sender(&self) -> Option<&UnboundedSender<Packet>> {
        None
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any + Send> {
        self
    }
}

fn next_state(state: State, player: Box<Player>, redis_conn: MultiplexedConnection) {
    match state {
        State::Title => tokio::spawn(title::update(player, redis_conn)),
        State::Matching => tokio::spawn(matching::regist(player)),
    };
}

fn send_message(
    session: Box<dyn Session>,
    message: &Packet,
    num_player: &mut usize,
) -> Box<dyn Session> {
    match session.sender() {
        Some(tx) => match tx.send(message.clone()) {
            Ok(_) => session,
            Err(_) => {
                println!("WebSocket disconnected ({:?})", session);
                *num_player -= 1;
                Box::new(Bot::from(session))
            }
        },
        None => session,
    }
}
