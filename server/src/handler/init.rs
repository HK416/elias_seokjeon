use super::*;

pub async fn setup(addr: SocketAddr, ws_stream: WebSocketStream<TcpStream>) {
    #[cfg(not(feature = "no-debuging-log"))]
    println!("Addr:{addr} - Current State: Init");

    let uuid = Uuid::new_v4();
    let name = "Text".to_string();
    let hero = rand::random();
    let (tx, mut rx) = unbounded_channel::<Packet>();
    let (mut write, read) = ws_stream.split();
    let write_task = tokio::spawn(async move {
        while let Some(s) = rx.recv().await {
            let s = serde_json::to_string(&s).unwrap();
            let result = write.send(Message::text(s)).await;
            if let Err(e) = result {
                eprintln!("Failed to send message to WebSocket (Address:{addr}): {e}");
                return write;
            }
        }
        write
    });

    let player = Player {
        uuid,
        name,
        hero,
        addr,
        read,
        tx,
        _write_task: write_task,
    };
    player
        .tx
        .send(Packet::Connection {
            uuid,
            name: player.name.clone(),
            hero,
        })
        .unwrap();

    next_state(State::Title, player);
}
