use super::*;

const MAX_WAIT_TIME: u32 = 15_000;
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

pub async fn wait(
    mut left: Box<dyn Session>,
    mut right: Box<dyn Session>,
    mut num_player: usize,
    redis_conn: MultiplexedConnection,
) {
    let message = Packet::MatchingSuccess {
        left: PlayData {
            uuid: left.uuid(),
            name: left.name().to_string(),
            hero: left.hero(),
            win: left.win(),
            lose: left.lose(),
        },
        right: PlayData {
            uuid: right.uuid(),
            name: right.name().to_string(),
            hero: right.hero(),
            win: right.win(),
            lose: right.lose(),
        },
    };
    left = send_message(left, &message, &mut num_player);
    right = send_message(right, &message, &mut num_player);
    if num_player == 0 {
        #[cfg(not(feature = "no-debugging-log"))]
        println!("Stop waiting.");
        return;
    }

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
            match n.session.reader() {
                Some(stream) => {
                    let mut cnt = MAX_LOOP;
                    while cnt > 0 {
                        match poll_stream_nonblocking(stream) {
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

                                #[cfg(not(feature = "no-debugging-log"))]
                                println!("{:?} replaced by Bot", n.session);

                                let bot = Box::new(Bot::from(n.session));
                                wait_sessions.push(Node::new(bot, n.left_side));
                                num_player -= 1;
                                continue 'update; // Session is removed due to error.
                            }
                            StreamPollResult::Closed => {
                                println!("WebSocket disconnected ({:?})", n.session);

                                #[cfg(not(feature = "no-debugging-log"))]
                                println!("{:?} replaced by Bot", n.session);

                                let bot = Box::new(Bot::from(n.session));
                                wait_sessions.push(Node::new(bot, n.left_side));
                                num_player -= 1;
                                continue 'update; // Session is removed due to closure.
                            }
                        }
                        cnt -= 1;
                    }
                }
                None => {
                    loaded_sessions.push(n);
                    continue 'update; // Session is removed from waiting.
                }
            }
            temp.push(n);
        }
        mem::swap(&mut wait_sessions, &mut temp);

        if num_player == 0 {
            #[cfg(not(feature = "no-debugging-log"))]
            println!("Stop waiting.");
            return;
        }

        'update: while let Some(mut n) = loaded_sessions.pop() {
            match n.session.reader() {
                Some(stream) => {
                    let mut cnt = MAX_LOOP;
                    while cnt > 0 {
                        match poll_stream_nonblocking(stream) {
                            StreamPollResult::Pending => break,
                            StreamPollResult::Item(_) => { /* empty */ }
                            StreamPollResult::Error(e) => {
                                println!("WebSocket disconnected ({:?}): {e}", n.session);

                                #[cfg(not(feature = "no-debugging-log"))]
                                println!("{:?} replaced by Bot", n.session);

                                let bot = Box::new(Bot::from(n.session));
                                loaded_sessions.push(Node::new(bot, n.left_side));
                                num_player -= 1;
                                continue 'update; // Session is removed due to error.
                            }
                            StreamPollResult::Closed => {
                                println!("WebSocket disconnected ({:?})", n.session);

                                #[cfg(not(feature = "no-debugging-log"))]
                                println!("{:?} replaced by Bot", n.session);

                                let bot = Box::new(Bot::from(n.session));
                                loaded_sessions.push(Node::new(bot, n.left_side));
                                num_player -= 1;
                                continue 'update; // Session is removed due to closure.
                            }
                        }
                        cnt -= 1;
                    }
                }
                None => { /* empty */ }
            }
            temp.push(n);
        }
        mem::swap(&mut loaded_sessions, &mut temp);

        if num_player == 0 {
            #[cfg(not(feature = "no-debugging-log"))]
            println!("Stop waiting.");
            return;
        }

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

            let redis_conn_cloned = redis_conn.clone();
            tokio::spawn(prepare::wait(left, right, num_player, redis_conn_cloned));
            return;
        }
    }

    while let Some(n) = wait_sessions.pop() {
        let mut session = n.session;
        let bot = Box::new(Bot::from(session.as_ref()));
        loaded_sessions.push(Node::new(bot, n.left_side));

        #[cfg(not(feature = "no-debugging-log"))]
        println!("{:?} replaced by Bot", session);

        let message = Packet::GameLoadTimeout;
        session = send_message(session, &message, &mut num_player);
        let result: Result<Box<Player>, Box<dyn Any + Send>> = session.into_any().downcast();
        if let Ok(player) = result {
            let redis_conn_cloned = redis_conn.clone();
            next_state(State::Title, player, redis_conn_cloned);
        }
    }

    if loaded_sessions.len() == 2 {
        let n0 = loaded_sessions.pop().unwrap();
        let n1 = loaded_sessions.pop().unwrap();
        let (left, right) = if n0.left_side {
            (n0.session, n1.session)
        } else {
            (n1.session, n0.session)
        };

        tokio::spawn(prepare::wait(left, right, num_player, redis_conn));
    }
}
