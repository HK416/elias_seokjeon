pub use serde;
pub use serde_json;
pub use uuid;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Header {
    Connection,
    EnterGame,
    CancelGame,
    MatchingStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Packet {
    pub header: Header,
    pub json: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectionPacket {
    pub uuid: Uuid,
    pub username: String,
}

impl From<ConnectionPacket> for Packet {
    fn from(value: ConnectionPacket) -> Self {
        Packet { 
            header: Header::Connection, 
            json: serde_json::to_string(&value).unwrap(),}
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnterGamePacket {
    pub uuid: Uuid,
}

impl From<EnterGamePacket> for Packet {
    fn from(value: EnterGamePacket) -> Self {
        Packet { 
            header: Header::EnterGame, 
            json: serde_json::to_string(&value).unwrap(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CancelGamePacket {
    pub uuid: Uuid,
}

impl From<CancelGamePacket> for Packet {
    fn from(value: CancelGamePacket) -> Self {
        Packet { 
            header: Header::CancelGame, 
            json: serde_json::to_string(&value).unwrap(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MatchingStatusPacket {
    pub millis: u16,
}

impl MatchingStatusPacket {
    pub const fn new(millis: u16) -> Self {
        Self { millis }
    }
}

impl From<MatchingStatusPacket> for Packet {
    fn from(value: MatchingStatusPacket) -> Self {
        Packet { 
            header: Header::MatchingStatus, 
            json: serde_json::to_string(&value).unwrap(), 
        }
    }
}
