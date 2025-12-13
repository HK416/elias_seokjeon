use super::*;

pub async fn update(mut player: Box<Player>, redis_conn: MultiplexedConnection) {
    #[cfg(not(feature = "no-debugging-log"))]
    println!("{:?} - Current State: Title", player);

    while let Some(result) = player.read.next().await {
        let message = match result {
            Ok(message) => message,
            Err(e) => {
                println!("WebSocket disconnected ({:?}): {}", &player, e);
                return;
            }
        };

        if let Message::Text(s) = message
            && let Ok(packet) = serde_json::from_str::<Packet>(&s)
        {
            match packet {
                Packet::EnterGame => {
                    return next_state(State::Matching, player, redis_conn);
                }
                _ => { /* empty */ }
            }
        }
    }
}
