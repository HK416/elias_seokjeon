#![cfg(target_arch = "wasm32")]
pub use bevy::prelude::*;
pub use wasm_bindgen::JsValue;
pub use web_sys::{Storage, WebSocket, window};

pub fn get_local_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}

#[derive(Resource)]
pub struct WebSocketManager {
    socket: WebSocket,
}

impl WebSocketManager {
    pub fn connect(url: &str) -> Result<Self, JsValue> {
        let ws = WebSocket::new(url)?;
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

        Ok(Self { socket: ws })
    }
}

unsafe impl Send for WebSocketManager {}
unsafe impl Sync for WebSocketManager {}
