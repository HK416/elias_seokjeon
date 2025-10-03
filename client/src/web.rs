#![cfg(target_arch = "wasm32")]
use bevy::prelude::*;
use wasm_bindgen::{JsCast, prelude::*};
use wasm_bindgen_futures::spawn_local;
pub use web_sys::{MessageEvent, Storage, WebSocket, window};

pub fn get_local_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}

pub struct NetworkManager {
    socket: WebSocket,
}

impl NetworkManager {
    pub fn connect(url: &str) -> Self {
        let ws = WebSocket::new(url).unwrap();
        let onmessage = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                web_sys::console::log_1(&format!("Received from server: {}", txt).into());
            }
        });
        ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget();

        let ws_cloned = ws.clone();
        let onopen = Closure::<dyn FnMut()>::new(move || {
            web_sys::console::log_1(&"Connected to server".into());
            ws_cloned.send_with_str("Hello, from Bevy client!").unwrap();
        });
        ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
        onopen.forget();

        Self { socket: ws }
    }
}
