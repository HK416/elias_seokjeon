pub use serde;
pub use serde_json;
pub use uuid;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub enum Header {
    Connection,
    EnterGame,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub header: Header,
    pub json: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectionMessage {
    pub uuid: Uuid,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnterGameMessage {
    pub uuid: Uuid,
}
