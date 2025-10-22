use super::*;

pub async fn update(mut player: Player) {
    #[cfg(not(feature = "no-debuging-log"))]
    println!("{:?} - Current State: Title", player);

    let mut state = State::Title;
    while let Some(result) = player.read.next().await {
        let message = match result {
            Ok(message) => message,
            Err(e) => {
                println!("WebSocket disconnected ({:?}): {}", player, e);
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

    next_state(state, player);
}
