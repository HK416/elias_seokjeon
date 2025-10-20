#![cfg(target_arch = "wasm32")]
pub use bevy::prelude::*;
pub use protocol::{Header, Packet, uuid::Uuid};
pub use wasm_bindgen::{JsCast, JsValue, closure::Closure};
pub use web_sys::{BinaryType, CloseEvent, ErrorEvent, MessageEvent, Storage, WebSocket, window};

pub fn get_local_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}

#[derive(Debug, PartialEq, Eq)]
pub enum NetError {
    NotFound,
    Closed(u16),
    Error(String),
}

#[derive(Resource)]
pub struct Network {
    pub socket: WebSocket,
    pub receiver: flume::Receiver<Result<Packet, NetError>>,
}

impl Network {
    pub fn new(url: &str) -> Result<Self, NetError> {
        let socket = match WebSocket::new(url) {
            Ok(socket) => socket,
            Err(e) => {
                error!("Failed to connect to the game server: {:?}", e);
                return Err(NetError::NotFound);
            }
        };

        let (sender, receiver) = flume::unbounded::<Result<Packet, NetError>>();
        let sender_cloned = sender.clone();
        let on_message_closure = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            if let Ok(text) = e.data().dyn_into::<js_sys::JsString>()
                && let Some(text) = text.as_string()
                && let Ok(message) = serde_json::from_str::<Packet>(&text)
            {
                info!("Received packet: {:?}", message);
                let _ = sender_cloned.send(Ok(message));
            }
        });

        let sender_cloned = sender.clone();
        let on_close_closure = Closure::<dyn FnMut(_)>::new(move |e: CloseEvent| {
            info!("WebSocket closed: {}:{}", e.code(), e.reason());
            let _ = sender_cloned.send(Err(NetError::Closed(e.code())));
        });

        let sender_cloned = sender;
        let on_error_closure = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
            error!("WebSocket error: {}", e.message());
            let _ = sender_cloned.send(Err(NetError::Error(e.message())));
        });

        socket.set_binary_type(BinaryType::Arraybuffer);
        socket.set_onmessage(Some(on_message_closure.as_ref().unchecked_ref()));
        socket.set_onclose(Some(on_close_closure.as_ref().unchecked_ref()));
        socket.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));
        on_message_closure.forget();
        on_close_closure.forget();
        on_error_closure.forget();

        Ok(Self { socket, receiver })
    }

    pub fn send(&self, message: &Packet) -> Result<(), JsValue> {
        let text = serde_json::to_string(message).unwrap();
        self.socket.send_with_str(&text)
    }

    pub fn try_iter(&self) -> flume::TryIter<'_, Result<Packet, NetError>> {
        self.receiver.try_iter()
    }
}

impl Drop for Network {
    fn drop(&mut self) {
        let _ = self.socket.close();
    }
}

unsafe impl Send for Network {}
unsafe impl Sync for Network {}
