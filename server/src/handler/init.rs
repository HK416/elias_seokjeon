use super::*;

pub async fn setup(addr: SocketAddr, ws_stream: WebSocketStream<TcpStream>) {
    #[cfg(not(feature = "no-debugging-log"))]
    println!("Addr:{addr} - Current State: Init");

    let uuid = Uuid::new_v4();
    let name = "Text".to_string();
    let hero = rand::random();
    let score = DEF_SCORE;
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

    let session = Session {
        uuid,
        name,
        hero,
        score,
        addr,
        read,
        tx,
        _write_task: write_task,
    };

    session
        .tx
        .send(Packet::Connection {
            uuid,
            name: session.name.clone(),
            hero,
            score,
        })
        .unwrap();

    next_state(State::Title, session);
}
