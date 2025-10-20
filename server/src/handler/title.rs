use super::*;

pub async fn update(uuid: Uuid, addr: SocketAddr, ws_stream: WebSocketStream<TcpStream>) {
    #[cfg(not(feature = "no-debuging-log"))]
    println!("Addr:{addr} - Current State: Title");

    let mut state = State::Title;
    let (tx, mut rx) = unbounded_channel::<Packet>();
    let (mut write, mut read) = ws_stream.split();
    let write_task = tokio::spawn(async move {
        while let Some(packet) = rx.recv().await {
            let s = serde_json::to_string(&packet).unwrap();
            let result = write.send(Message::text(s)).await;
            if let Err(e) = result {
                eprintln!("Failed to send message to WebSocket (Address:{addr}): {e}");
                return write;
            }
        }
        write
    });

    while let Some(result) = read.next().await {
        let message = match result {
            Ok(message) => message,
            Err(e) => {
                println!("WebSocket disconnected (Address:{addr}): {e}");
                return;
            }
        };

        if let Message::Text(s) = message
            && let Ok(packet) = serde_json::from_str::<Packet>(&s)
        {
            match packet {
                Packet::EnterGame => {
                    state = State::Matching;
                    break;
                }
                _ => { /* empty */ }
            }
        }
    }

    drop(tx);
    let other = write_task.await.unwrap();
    let ws_stream = read.reunite(other).unwrap();
    next_state(uuid, state, ws_stream, addr);
}
