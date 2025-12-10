use super::*;

const MAX_WAIT_TIME: u32 = 5_000;
const MAX_LOOP: usize = 100;

struct Node {
    session: Box<dyn Session>,
    left_side: bool,
}

impl Node {
    pub fn new(session: Box<dyn Session>, left_side: bool) -> Self {
        Self { session, left_side }
    }
}

pub async fn wait(mut left: Box<dyn Session>, mut right: Box<dyn Session>, mut num_player: usize) {
    let message = Packet::PrepareInGame;
    left = send_message(left, &message, &mut num_player);
    right = send_message(right, &message, &mut num_player);
    if num_player == 0 {
        #[cfg(not(feature = "no-debugging-log"))]
        println!("Stop preparing");
        return;
    }

    let mut wait_sessions = vec![Node::new(left, true), Node::new(right, false)];
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
            match n.session.reader() {
                Some(reader) => {
                    let mut cnt = MAX_LOOP;
                    while cnt > 0 {
                        match poll_stream_nonblocking(reader) {
                            StreamPollResult::Pending => break,
                            StreamPollResult::Item(_) => { /* empty */ }
                            StreamPollResult::Error(e) => {
                                println!("WebSocket disconnected ({:?}): {e}", n.session);

                                #[cfg(not(feature = "no-debugging-log"))]
                                println!("{:?} replaced by Bot", n.session);

                                let bot = Box::new(Bot::from(n.session));
                                wait_sessions.push(Node::new(bot, n.left_side));
                                num_player -= 1;

                                continue 'update; // Player is removed due to error.
                            }
                            StreamPollResult::Closed => {
                                println!("WebSocket disconnected ({:?})", n.session);

                                #[cfg(not(feature = "no-debugging-log"))]
                                println!("{:?} replaced by Bot", n.session);

                                let bot = Box::new(Bot::from(n.session));
                                wait_sessions.push(Node::new(bot, n.left_side));
                                num_player -= 1;

                                continue 'update; // Player is removed due to closure.
                            }
                        }
                        cnt -= 1;
                    }
                }
                None => { /* empty */ }
            }
            temp.push(n);
        }
        mem::swap(&mut wait_sessions, &mut temp);

        if num_player == 0 {
            #[cfg(not(feature = "no-debugging-log"))]
            println!("Stop preparing");
            return;
        }
    }

    if wait_sessions.len() == 2 {
        #[cfg(not(feature = "no-debugging-log"))]
        println!(
            "[{:?} VS {:?}] - Both players are ready",
            wait_sessions[0].session, wait_sessions[1].session,
        );

        let n0 = wait_sessions.pop().unwrap();
        let n1 = wait_sessions.pop().unwrap();
        let (left, right) = if n0.left_side {
            (n0.session, n1.session)
        } else {
            (n1.session, n0.session)
        };

        tokio::spawn(in_game::play(left, right, num_player));
    }
}
