use super::*;

const MAX_WAIT_TIME: u32 = 15_000;
const MAX_LOOP: usize = 100;

pub async fn wait(left: Player, right: Player) {
    left.tx
        .send(Packet::MatchingSuccess {
            other: right.name.clone(),
            hero: right.hero,
        })
        .unwrap();
    right
        .tx
        .send(Packet::MatchingSuccess {
            other: left.name.clone(),
            hero: left.hero,
        })
        .unwrap();

    let mut wait_players = vec![left, right];
    let mut loaded_player: Vec<Player> = Vec::new();
    let mut temp = Vec::new();

    const TICK: u64 = 1_000 / 15;
    const PERIOD: Duration = Duration::from_millis(TICK);
    let mut previous_instant = Instant::now();
    let mut interval = time::interval(PERIOD);
    let mut millis = MAX_WAIT_TIME;
    while millis > 0 {
        let instant = interval.tick().await;
        let elapsed = instant
            .saturating_duration_since(previous_instant)
            .as_millis();
        millis = millis.saturating_sub(elapsed as u32);
        previous_instant = instant;

        'update: while let Some(mut p) = wait_players.pop() {
            let mut cnt = MAX_LOOP;
            while cnt > 0 {
                match poll_stream_nonblocking(&mut p.read) {
                    StreamPollResult::Pending => break,
                    StreamPollResult::Item(message) => {
                        if let Message::Text(s) = message
                            && let Ok(packet) = serde_json::from_str::<Packet>(&s)
                        {
                            match packet {
                                Packet::GameLoadSuccess => {
                                    loaded_player.push(p);
                                    continue 'update; // Player is removed from waiting.
                                }
                                _ => { /* empty */ }
                            }
                        }
                    }
                    StreamPollResult::Error(e) => {
                        println!("WebSocket disconnected ({:?}): {e}", p);
                        continue 'update; // Player is removed due to error.
                    }
                    StreamPollResult::Closed => {
                        println!("WebSocket disconnected ({:?})", p);
                        continue 'update; // Player is removed due to closure.
                    }
                }
                cnt -= 1;
            }
            temp.push(p);
        }
        mem::swap(&mut wait_players, &mut temp);

        'update: while let Some(mut p) = loaded_player.pop() {
            let mut cnt = MAX_LOOP;
            while cnt > 0 {
                match poll_stream_nonblocking(&mut p.read) {
                    StreamPollResult::Pending => break,
                    StreamPollResult::Item(_) => { /* empty */ }
                    StreamPollResult::Error(e) => {
                        println!("WebSocket disconnected ({:?}): {e}", p);
                        continue 'update; // Player is removed due to error.
                    }
                    StreamPollResult::Closed => {
                        println!("WebSocket disconnected ({:?})", p);
                        continue 'update; // Player is removed due to closure.
                    }
                }
                cnt -= 1;
            }
            temp.push(p);
        }
        mem::swap(&mut loaded_player, &mut temp);

        if loaded_player.len() == 2 {
            #[cfg(not(feature = "no-debuging-log"))]
            println!("All players loaded!");
            return;
        }
    }

    while let Some(player) = wait_players.pop() {
        player.tx.send(Packet::GameLoadTimeout).unwrap();
        next_state(State::Title, player);
    }

    while let Some(player) = loaded_player.pop() {
        // --- Temp Code ---
        #[cfg(not(feature = "no-debuging-log"))]
        println!(
            "FIXME: Single player mode is not implemented yet. ({}/{})",
            file!(),
            line!()
        );

        player.tx.send(Packet::GameLoadTimeout).unwrap();
        next_state(State::Title, player);
        //------------------
    }
}
