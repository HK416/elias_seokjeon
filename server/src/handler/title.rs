use super::*;

pub async fn update(mut session: Session) {
    #[cfg(not(feature = "no-debugging-log"))]
    println!("{:?} - Current State: Title", session);

    while let Some(result) = session.read.next().await {
        let message = match result {
            Ok(message) => message,
            Err(e) => {
                println!("WebSocket disconnected ({:?}): {}", session, e);
                return;
            }
        };

        if let Message::Text(s) = message
            && let Ok(packet) = serde_json::from_str::<Packet>(&s)
        {
            match packet {
                Packet::EnterGame => {
                    return next_state(State::Matching, session);
                }
                _ => { /* empty */ }
            }
        }
    }
}
