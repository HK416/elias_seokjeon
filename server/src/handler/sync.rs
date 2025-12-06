use super::*;

const MAX_WAIT_TIME: u32 = 15_000;
const MAX_LOOP: usize = 100;

struct Node {
    session: Session,
    left_side: bool,
}

impl Node {
    pub fn new(session: Session, left_side: bool) -> Self {
        Self { session, left_side }
    }
}

pub async fn wait(left: Session, right: Session) {
    let message = Packet::MatchingSuccess {
        left: Player {
            uuid: left.uuid,
            name: left.name.clone(),
            hero: left.hero,
            win: left.win,
            lose: left.lose,
        },
        right: Player {
            uuid: right.uuid,
            name: right.name.clone(),
            hero: right.hero,
            win: right.win,
            lose: right.lose,
        },
    };

    left.tx.send(message.clone()).unwrap();
    right.tx.send(message.clone()).unwrap();

    let mut wait_sessions = vec![Node::new(left, true), Node::new(right, false)];
    let mut loaded_sessions: Vec<Node> = Vec::new();
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

        'update: while let Some(mut n) = wait_sessions.pop() {
            let mut cnt = MAX_LOOP;
            while cnt > 0 {
                match poll_stream_nonblocking(&mut n.session.read) {
                    StreamPollResult::Pending => break,
                    StreamPollResult::Item(message) => {
                        if let Message::Text(s) = message
                            && let Ok(packet) = serde_json::from_str::<Packet>(&s)
                        {
                            match packet {
                                Packet::GameLoadSuccess => {
                                    loaded_sessions.push(n);
                                    continue 'update; // Session is removed from waiting.
                                }
                                _ => { /* empty */ }
                            }
                        }
                    }
                    StreamPollResult::Error(e) => {
                        println!("WebSocket disconnected ({:?}): {e}", n.session);
                        continue 'update; // Session is removed due to error.
                    }
                    StreamPollResult::Closed => {
                        println!("WebSocket disconnected ({:?})", n.session);
                        continue 'update; // Session is removed due to closure.
                    }
                }
                cnt -= 1;
            }
            temp.push(n);
        }
        mem::swap(&mut wait_sessions, &mut temp);

        'update: while let Some(mut n) = loaded_sessions.pop() {
            let mut cnt = MAX_LOOP;
            while cnt > 0 {
                match poll_stream_nonblocking(&mut n.session.read) {
                    StreamPollResult::Pending => break,
                    StreamPollResult::Item(_) => { /* empty */ }
                    StreamPollResult::Error(e) => {
                        println!("WebSocket disconnected ({:?}): {e}", n.session);
                        continue 'update; // Session is removed due to error.
                    }
                    StreamPollResult::Closed => {
                        println!("WebSocket disconnected ({:?})", n.session);
                        continue 'update; // Session is removed due to closure.
                    }
                }
                cnt -= 1;
            }
            temp.push(n);
        }
        mem::swap(&mut loaded_sessions, &mut temp);

        if loaded_sessions.len() == 2 {
            #[cfg(not(feature = "no-debugging-log"))]
            println!("All players loaded!");

            let n0 = loaded_sessions.pop().unwrap();
            let n1 = loaded_sessions.pop().unwrap();
            let (left, right) = if n0.left_side {
                (n0.session, n1.session)
            } else {
                (n1.session, n0.session)
            };

            tokio::spawn(prepare::wait(left, right));
            return;
        }
    }

    while let Some(n) = wait_sessions.pop() {
        n.session.tx.send(Packet::GameLoadTimeout).unwrap();
        next_state(State::Title, n.session);
    }

    while let Some(n) = loaded_sessions.pop() {
        // --- Temp Code ---
        #[cfg(not(feature = "no-debugging-log"))]
        println!(
            "FIXME: Single player mode is not implemented yet. ({}/{})",
            file!(),
            line!()
        );

        n.session.tx.send(Packet::GameLoadTimeout).unwrap();
        next_state(State::Title, n.session);
        //------------------
    }
}
