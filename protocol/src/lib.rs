pub use serde;
pub use serde_json;
pub use uuid;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub enum Packet {
    Connection {
        uuid: Uuid,
        username: String,
    },
    EnterGame,
    CancelGame,
    MatchingStatus {
        millis: u16,
    },
    MatchingResult {
        username: String,
    }
}
