use super::*;

const MAX_LOOP: usize = 100;

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
    let mut left_health = MAX_HEALTH;
    let mut right_health = MAX_HEALTH;
    let mut throw_angle = None;
    let mut throw_power = None;
    let mut wind_angle = rand::random_range(0..255);
    let mut wind_power = rand::random_range(0..255);
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
        remaining_millis = remaining_millis.saturating_sub(elapsed_u16);

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
                                throw_angle = Some(angle);
                                throw_power = Some(power);
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
                                throw_angle = Some(angle);
                                throw_power = Some(power);
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
                    wind_angle,
                    wind_power,
                    left_health,
                    right_health,
                    angle: throw_angle,
                    power: throw_power,
                });

                if remaining_millis == 0 {
                    #[cfg(not(feature = "no-debuging-log"))]
                    println!("Left turn ended.");
                    game_state = GameState::RightTurn;
                    remaining_millis = MAX_CTRL_TIME;
                    throw_angle = None;
                    throw_power = None;
                }
            }
            GameState::RightTurn => {
                broadcaster.broadcast(&Packet::InGameRightTurn {
                    total_remaining_millis,
                    remaining_millis,
                    wind_angle,
                    wind_power,
                    left_health,
                    right_health,
                    angle: throw_angle,
                    power: throw_power,
                });

                if remaining_millis == 0 {
                    #[cfg(not(feature = "no-debuging-log"))]
                    println!("Right turn ended.");
                    game_state = GameState::LeftTurn;
                    remaining_millis = MAX_CTRL_TIME;
                    throw_angle = None;
                    throw_power = None;
                }
            }
            GameState::ProjectileThrown => {
                // TODO
            }
        }
    }

    #[cfg(not(feature = "no-debuging-log"))]
    println!("Game ended.");
}
