#![cfg(target_arch = "wasm32")]
pub use bevy::prelude::*;
pub use wasm_bindgen::{JsCast, closure::Closure};
pub use web_sys::{BinaryType, MessageEvent, Storage, WebSocket, window};

pub fn get_local_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}

#[derive(Resource)]
pub struct Network {
    pub socket: WebSocket,
    pub receiver: flume::Receiver<protocol::Message>,
}

unsafe impl Send for Network {}
unsafe impl Sync for Network {}
