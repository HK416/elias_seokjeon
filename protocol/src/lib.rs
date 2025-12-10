use std::{collections::HashMap, fmt};

use lazy_static::lazy_static;
pub use rand;
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};
pub use serde;
pub use serde_json;
pub use uuid;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const NUM_HEROS: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Hero {
    Butter,
    Kommy,
}

impl fmt::Display for Hero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hero::Butter => write!(f, "butter"),
            Hero::Kommy => write!(f, "kommy"),
        }
    }
}

impl Distribution<Hero> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hero {
        match rng.random_range(0..NUM_HEROS) {
            0 => Hero::Butter,
            1 => Hero::Kommy,
            _ => unreachable!(),
        }
    }
}

pub const MAX_POINT: u16 = 9_999;
// pub const MAX_PLAY_TIME: i32 = 180_000; // 180 seconds
pub const MAX_PLAY_TIME: i32 = 10_000; // 10 seconds
pub const MAX_CTRL_TIME: u16 = 10_000; // 10 seconds
pub const MAX_HEALTH_COUNT: usize = 5;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Packet {
    // Server -> Client
    Connection(PlayData),
    // Client -> Server
    EnterGame,
    // Client -> Server
    TryCancelGame,
    // Server -> Client
    CancelSuccess,
    // Server -> Client
    MatchingStatus {
        millis: u16,
    },
    // Server -> Client
    MatchingSuccess {
        left: PlayData,
        right: PlayData,
    },
    // Client -> Server
    GameLoadSuccess,
    // Server -> Client
    GameLoadTimeout,
    // Server -> Client
    PrepareInGame,
    // Client -> Server
    UpdateThrowParams {
        angle: u8,
        power: u8,
    },
    // Client -> Server
    ThrowProjectile,
    // Server -> Client
    InGameLeftTurn {
        total_remaining_millis: i32,
        remaining_millis: u16,
        left_health_cnt: u8,
        right_health_cnt: u8,
        control: Option<(u8, u8)>,
    },
    // Server -> Client
    InGameRightTurn {
        total_remaining_millis: i32,
        remaining_millis: u16,
        left_health_cnt: u8,
        right_health_cnt: u8,
        control: Option<(u8, u8)>,
    },
    // Server -> Client
    InGameTurnSetup {
        wind_angle: u8,
        wind_power: u8,
    },
    // Server -> Client
    InGameProjectileThrown {
        total_remaining_millis: i32,
        remaining_millis: u16,
        left_health_cnt: u8,
        right_health_cnt: u8,
        projectile_pos: (f32, f32),
        projectile_vel: (f32, f32),
    },
    // Server -> Client
    GameResult {
        win: u16,
        lose: u16,
        victory: bool,
    },
    // Server -> Client
    GameResultDraw,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayData {
    pub uuid: Option<Uuid>,
    pub name: String,
    pub hero: Hero,
    pub win: u16,
    pub lose: u16,
}

pub const WORLD_MIN_X: f32 = -1440.0;
pub const WORLD_MAX_X: f32 = 1440.0;
pub const WORLD_MIN_Y: f32 = -540.0;
pub const WORLD_MAX_Y: f32 = 540.0;

pub const LEFT_CAM_POS_X: f32 = -480.0;
pub const RIGHT_CAM_POS_X: f32 = 480.0;

pub const LEFT_PLAYER_POS_X: f32 = -960.0;
pub const LEFT_PLAYER_POS_Y: f32 = 340.0;

pub const RIGHT_PLAYER_POS_X: f32 = -LEFT_PLAYER_POS_X;
pub const RIGHT_PLAYER_POS_Y: f32 = LEFT_PLAYER_POS_Y;

pub const LEFT_THROW_POS_X: f32 = LEFT_PLAYER_POS_X;
pub const LEFT_THROW_POS_Y: f32 = LEFT_PLAYER_POS_Y + 96.0;

pub const LEFT_START_ANGLE: f32 = 15f32.to_radians();
pub const LEFT_END_ANGLE: f32 = 75f32.to_radians();

pub const RIGHT_THROW_POS_X: f32 = RIGHT_PLAYER_POS_X;
pub const RIGHT_THROW_POS_Y: f32 = RIGHT_PLAYER_POS_Y + 96.0;

pub const RIGHT_START_ANGLE: f32 = 105f32.to_radians();
pub const RIGHT_END_ANGLE: f32 = 165f32.to_radians();

pub const THROW_POWER: f32 = 1500.0;
pub const THROW_END_TIME: u16 = 3_000; // 3 seconds

pub const WIND_POWER: f32 = THROW_POWER * 0.125;

pub const PROJECTILE_SIZE: f32 = 64.0;
pub const GRAVITY: f32 = -9.80665 * 50.0;

lazy_static! {
    pub static ref COLLIDER_DATA: HashMap<Hero, Circle> = HashMap::from_iter([
        (Hero::Butter, Circle::new(40.0, (0.0, 142.0))),
        (Hero::Kommy, Circle::new(40.0, (0.0, 152.0))),
    ]);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub radius: f32,
    pub center: (f32, f32),
}

impl Circle {
    pub fn new(radius: f32, center: (f32, f32)) -> Self {
        Self { radius, center }
    }
}
