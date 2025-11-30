use super::*;

const MAX_LOOP: usize = 100;
const WIND_THRESHOLD: usize = 6;

struct Broadcaster {
    left: Session,
    right: Session,
}

impl Broadcaster {
    fn new(left: Session, right: Session) -> Self {
        Self { left, right }
    }

    fn broadcast(&self, packet: &Packet) {
        self.left.tx.send(packet.clone()).unwrap();
        self.right.tx.send(packet.clone()).unwrap();
    }
}

#[derive(Default, Clone, Copy)]
enum GameState {
    #[default]
    LeftTurn,
    RightTurn,
    ProjectileThrown,
}

pub async fn play(left: Session, right: Session) {
    let mut broadcaster = Broadcaster::new(left, right);

    const TICK: u64 = 1_000 / 15;
    const PERIOD: Duration = Duration::from_millis(TICK);
    let mut turn_counter = 0;
    let mut left_health = MAX_HEALTH;
    let mut right_health = MAX_HEALTH;
    let mut control = None;
    let mut wind_angle = 0;
    let mut wind_power = 0;
    let mut game_state = GameState::default();
    let mut remaining_millis = MAX_CTRL_TIME;
    let mut total_remaining_millis = MAX_PLAY_TIME;
    let mut interval = time::interval(PERIOD);
    let mut previous_instant = Instant::now();
    while total_remaining_millis > 0 {
        let instant = interval.tick().await;
        let elapsed = instant
            .saturating_duration_since(previous_instant)
            .as_millis();
        let elapsed_u16 = elapsed.min(u16::MAX as u128) as u16;
        let elapsed_u32 = elapsed.min(u32::MAX as u128) as u32;
        previous_instant = instant;

        total_remaining_millis = total_remaining_millis.saturating_sub(elapsed_u32);
        if !matches!(game_state, GameState::ProjectileThrown) {
            remaining_millis = remaining_millis.saturating_sub(elapsed_u16);
        }

        let mut cnt = MAX_LOOP;
        'update: while cnt > 0 {
            match poll_stream_nonblocking(&mut broadcaster.left.read) {
                StreamPollResult::Pending => break,
                StreamPollResult::Item(message) => {
                    if let Message::Text(s) = message
                        && let Ok(packet) = serde_json::from_str::<Packet>(&s)
                    {
                        match (game_state, packet) {
                            (GameState::LeftTurn, Packet::UpdateThrowParams { angle, power }) => {
                                control = Some((angle, power));
                            }
                            (GameState::LeftTurn, Packet::ThrowProjectile) => {
                                game_state = GameState::ProjectileThrown;
                                remaining_millis = 0;
                            }
                            _ => { /* empty */ }
                        }
                    }
                }
                StreamPollResult::Error(e) => {
                    println!("WebSocket disconnected ({:?}): {e}", broadcaster.left);
                    break 'update; // Handle disconnection.
                }
                StreamPollResult::Closed => {
                    println!("WebSocket disconnected ({:?})", broadcaster.left);
                    break 'update; // Handle closure.
                }
            }
            cnt -= 1;
        }

        let mut cnt = MAX_LOOP;
        'update: while cnt > 0 {
            match poll_stream_nonblocking(&mut broadcaster.right.read) {
                StreamPollResult::Pending => break,
                StreamPollResult::Item(message) => {
                    if let Message::Text(s) = message
                        && let Ok(packet) = serde_json::from_str::<Packet>(&s)
                    {
                        match (game_state, packet) {
                            (GameState::RightTurn, Packet::UpdateThrowParams { angle, power }) => {
                                control = Some((angle, power));
                            }
                            (GameState::RightTurn, Packet::ThrowProjectile) => {
                                game_state = GameState::ProjectileThrown;
                                remaining_millis = 0;
                            }
                            _ => { /* empty */ }
                        }
                    }
                }
                StreamPollResult::Error(e) => {
                    println!("WebSocket disconnected ({:?}): {e}", broadcaster.right);
                    break 'update; // Handle disconnection.
                }
                StreamPollResult::Closed => {
                    println!("WebSocket disconnected ({:?})", broadcaster.right);
                    break 'update; // Handle closure.
                }
            }
            cnt -= 1;
        }

        match game_state {
            GameState::LeftTurn => {
                broadcaster.broadcast(&Packet::InGameLeftTurn {
                    total_remaining_millis,
                    remaining_millis,
                    left_health,
                    right_health,
                    control,
                });

                if remaining_millis == 0 {
                    #[cfg(not(feature = "no-debuging-log"))]
                    println!("Left turn ended.");

                    turn_counter += 1;
                    if turn_counter > WIND_THRESHOLD {
                        wind_angle = rand::random_range(0..255);
                        wind_power = rand::random_range(128..255);
                    }
                    broadcaster.broadcast(&Packet::InGameTurnSetup {
                        wind_angle,
                        wind_power,
                    });

                    game_state = GameState::RightTurn;
                    remaining_millis = MAX_CTRL_TIME;
                    control = None;
                }
            }
            GameState::RightTurn => {
                broadcaster.broadcast(&Packet::InGameRightTurn {
                    total_remaining_millis,
                    remaining_millis,
                    left_health,
                    right_health,
                    control,
                });

                if remaining_millis == 0 {
                    #[cfg(not(feature = "no-debuging-log"))]
                    println!("Right turn ended.");

                    turn_counter += 1;
                    if turn_counter > WIND_THRESHOLD {
                        wind_angle = rand::random_range(0..255);
                        wind_power = rand::random_range(128..255);
                    }
                    broadcaster.broadcast(&Packet::InGameTurnSetup {
                        wind_angle,
                        wind_power,
                    });

                    game_state = GameState::LeftTurn;
                    remaining_millis = MAX_CTRL_TIME;
                    control = None;
                }
            }
            GameState::ProjectileThrown => {
                #[cfg(not(feature = "no-debuging-log"))]
                println!("Projectile thrown.");

                // TODO

                turn_counter += 1;
                if turn_counter > WIND_THRESHOLD {
                    wind_angle = rand::random_range(0..255);
                    wind_power = rand::random_range(128..255);
                }
                broadcaster.broadcast(&Packet::InGameTurnSetup {
                    wind_angle,
                    wind_power,
                });

                game_state = match turn_counter % 2 == 0 {
                    true => GameState::LeftTurn,
                    false => GameState::RightTurn,
                };
                remaining_millis = MAX_CTRL_TIME;
                control = None;
            }
        }
    }

    #[cfg(not(feature = "no-debuging-log"))]
    println!("Game ended.");
}
