#![cfg(target_arch = "wasm32")]
pub use bevy::prelude::*;
pub use wasm_bindgen::{JsCast, JsValue, closure::Closure};
pub use web_sys::{BinaryType, MessageEvent, Storage, WebSocket, window};

pub fn get_local_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}

#[derive(Resource)]
pub struct Network {
    pub socket: WebSocket,
    pub receiver: flume::Receiver<protocol::Message>,
}

impl Network {
    pub const fn new(socket: WebSocket, receiver: flume::Receiver<protocol::Message>) -> Self {
        Self { socket, receiver }
    }

    pub fn send(&self, message: &protocol::Message) -> Result<(), JsValue> {
        let text = serde_json::to_string(message).unwrap();
        self.socket.send_with_str(&text)
    }

    pub fn try_iter(&self) -> flume::TryIter<'_, protocol::Message> {
        self.receiver.try_iter()
    }
}

unsafe impl Send for Network {}
unsafe impl Sync for Network {}
