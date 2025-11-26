use std::fmt;

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

pub const DEF_SCORE: u16 = 100;
pub const MAX_SCORE: u16 = 9_999;
pub const MAX_PLAY_TIME: u32 = 180_000; // 180 seconds
pub const MAX_CTRL_TIME: u16 = 10_000; // 10 seconds
pub const MAX_HEALTH: u16 = 100;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Packet {
    // Server -> Client
    Connection {
        uuid: Uuid,
        name: String,
        hero: Hero,
        score: u16,
    },
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
        left: Player,
        right: Player,
    },
    // Client -> Server
    GameLoadSuccess, 
    // Server -> Client
    GameLoadTimeout, 
    // Server -> Client
    PrepareInGame, 
    // Client -> Server
    UpdateThrowParams {
        angle: f32,
        power: f32,
    },
    // Client -> Server
    ThrowProjectile,
    // Server -> Client
    InGameLeftTurn {
        total_remaining_millis: u32,
        remaining_millis: u16,
        wind_angle: u8,
        wind_power: u8,
        left_health: u16,
        right_health: u16,
        angle: Option<f32>,
        power: Option<f32>,
    },
    // Server -> Client
    InGameRightTurn {
        total_remaining_millis: u32,
        remaining_millis: u16,
        wind_angle: u8,
        wind_power: u8,
        left_health: u16,
        right_health: u16,
        angle: Option<f32>,
        power: Option<f32>,
    },
    // Server -> Client
    InGameProjectileThrown {
        total_remaining_millis: u32,
        wind_angle: u8,
        wind_power: u8,
        left_health: u16,
        right_health: u16,
        projectile_pos: (f32, f32),
        projectile_vel: (f32, f32),
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Player {
    pub uuid: Uuid,
    pub name: String,
    pub hero: Hero,
    pub score: u16,
}

pub const WORLD_MIN_X: f32 = -1440.0;
pub const WORLD_MAX_X: f32 = 1440.0;
pub const WORLD_MIN_Y: f32 = -540.0;
pub const WORLD_MAX_Y: f32 = 540.0;

pub const LEFT_CAM_POS_X: f32 = -480.0;
pub const RIGHT_CAM_POS_X: f32 = 480.0;

pub const LEFT_PLAYER_POS_X : f32 = -960.0;
pub const LEFT_PLAYER_POS_Y : f32 = 340.0;

pub const RIGHT_PLAYER_POS_X: f32 = 960.0;
pub const RIGHT_PLAYER_POS_Y: f32 = 340.0;
