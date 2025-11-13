use super::*;

const MAX_WAIT_TIME: u32 = 5_000;
const MAX_LOOP: usize = 100;

pub async fn wait(left: Player, right: Player) {
    left.tx.send(Packet::PrepareInGame).unwrap();
    right.tx.send(Packet::PrepareInGame).unwrap();

    let mut wait_players = vec![left, right];
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
        mem::swap(&mut wait_players, &mut temp);

        if wait_players.is_empty() {
            return;
        }
    }
}
