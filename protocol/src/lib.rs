use rand::{distr::{Distribution, StandardUniform}, Rng};
pub use serde;
pub use serde_json;
pub use uuid;
pub use rand;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

const NUM_HEROS: usize = 2;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub enum Packet {
    Connection {
        uuid: Uuid,
        name: String,
        hero: Hero,
    },
    EnterGame,
    TryCancelGame,
    CancelSuccess,
    MatchingStatus {
        millis: u16,
    },
    MatchingSuccess {
        other: String,
        hero: Hero,
    },
}
