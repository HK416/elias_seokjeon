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

impl Distribution<Hero> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hero {
        match rng.random_range(0..NUM_HEROS) {
            0 => Hero::Butter,
            1 => Hero::Kommy,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Packet {
    Connection {
        // Server -> Client
        uuid: Uuid,
        name: String,
        hero: Hero,
    },
    EnterGame,     // Client -> Server
    TryCancelGame, // Client -> Server
    CancelSuccess, // Server -> Client
    MatchingStatus {
        // Server -> Client
        millis: u16,
    },
    MatchingSuccess {
        // Server -> Client
        other: String,
        hero: Hero,
    },
    GameLoadSuccess, // Client -> Server
    GameLoadTimeout, // Server -> Client
}
