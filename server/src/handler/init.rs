use super::*;

pub async fn setup(addr: SocketAddr, mut ws_stream: WebSocketStream<TcpStream>) {
    #[cfg(not(feature = "no-debuging-log"))]
    println!("Addr:{addr} - Current State: Init");

    let uuid = Uuid::new_v4();
    let packet = Packet::Connection {
        uuid,
        username: "Test".into(),
    };

    let s = serde_json::to_string(&packet).unwrap();
    let item = Message::text(s);
    if let Err(e) = ws_stream.send(item).await {
        println!("WebSocket disconnected (Address:{addr}): {e}");
    }

    next_state(uuid, State::Title, ws_stream, addr);
}
