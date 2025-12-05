use glam::FloatExt;

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
    let mut turn_counter = 0;
    let mut left_health = MAX_HEALTH_COUNT;
    let mut right_health = MAX_HEALTH_COUNT;
    let mut control = None;
    let (mut wind_angle, mut wind_power, mut wind_vel) = update_wind_parameter();
    let mut projectile_vel = Vec2::ZERO;
    let mut projectile_pos = Vec2::new(LEFT_THROW_POS_X, LEFT_THROW_POS_Y);
    let mut game_state = GameState::default();
    let mut remaining_millis = MAX_CTRL_TIME;
    let mut total_remaining_millis = MAX_PLAY_TIME;
    let mut interval = time::interval(PERIOD);
    let mut previous_instant = Instant::now();
    broadcaster.broadcast(&Packet::InGameTurnSetup {
        wind_angle,
        wind_power,
    });

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
                                projectile_pos = Vec2::new(LEFT_THROW_POS_X, LEFT_THROW_POS_Y);
                                projectile_vel = control
                                    .map(|(angle, power)| {
                                        let delta = angle as f32 / 255.0;
                                        let radian = LEFT_START_ANGLE
                                            + (LEFT_END_ANGLE - LEFT_START_ANGLE) * delta;
                                        let direction = Vec2::new(radian.cos(), radian.sin());
                                        let power = (power as f32 / 255.0) * THROW_POWER;
                                        direction * power
                                    })
                                    .unwrap_or_default();
                                game_state = GameState::ProjectileThrown;
                                remaining_millis = THROW_END_TIME;
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
                                projectile_pos = Vec2::new(RIGHT_THROW_POS_X, RIGHT_THROW_POS_Y);
                                projectile_vel = control
                                    .map(|(angle, power)| {
                                        let delta = angle as f32 / 255.0;
                                        let radian = RIGHT_START_ANGLE
                                            + (RIGHT_END_ANGLE - RIGHT_START_ANGLE) * delta;
                                        let direction = Vec2::new(radian.cos(), radian.sin());
                                        let power = (power as f32 / 255.0) * THROW_POWER;
                                        direction * power
                                    })
                                    .unwrap_or_default();
                                game_state = GameState::ProjectileThrown;
                                remaining_millis = THROW_END_TIME;
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
                    left_health_cnt: left_health as u8,
                    right_health_cnt: right_health as u8,
                    control,
                });

                if remaining_millis == 0 {
                    #[cfg(not(feature = "no-debugging-log"))]
                    println!("Left turn ended.");

                    turn_counter += 1;
                    (wind_angle, wind_power, wind_vel) = update_wind_parameter();
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
                    left_health_cnt: left_health as u8,
                    right_health_cnt: right_health as u8,
                    control,
                });

                if remaining_millis == 0 {
                    #[cfg(not(feature = "no-debugging-log"))]
                    println!("Right turn ended.");

                    turn_counter += 1;
                    (wind_angle, wind_power, wind_vel) = update_wind_parameter();
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
                let delta_time = elapsed_u16 as f32 / 1000.0;
                projectile_vel.y += GRAVITY * delta_time;
                projectile_pos += (projectile_vel + wind_vel) * delta_time;

                if projectile_pos.y < LEFT_PLAYER_POS_Y {
                    wind_vel = Vec2::ZERO;
                    projectile_vel.x = projectile_vel.x.lerp(0.0, 0.25);
                    projectile_pos.y = LEFT_PLAYER_POS_Y;
                }

                if projectile_pos.y <= LEFT_PLAYER_POS_Y
                    || projectile_pos.x <= WORLD_MIN_X
                    || projectile_pos.x >= WORLD_MAX_X
                {
                    remaining_millis = remaining_millis.saturating_sub(elapsed_u16);
                }

                broadcaster.broadcast(&Packet::InGameProjectileThrown {
                    total_remaining_millis,
                    remaining_millis,
                    left_health_cnt: left_health as u8,
                    right_health_cnt: right_health as u8,
                    projectile_pos: projectile_pos.into(),
                    projectile_vel: projectile_vel.into(),
                });

                if remaining_millis == 0 {
                    #[cfg(not(feature = "no-debugging-log"))]
                    println!("Projectile thrown.");

                    turn_counter += 1;
                    (wind_angle, wind_power, wind_vel) = update_wind_parameter();
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
    }

    #[cfg(not(feature = "no-debugging-log"))]
    println!("Game ended.");
}

fn update_wind_parameter() -> (u8, u8, Vec2) {
    let wind_angle = rand::random_range(0..255);
    let wind_power = rand::random_range(128..255);

    let radian = (wind_angle as f32 / 255.0) * TAU;
    let direction = Vec2::new(radian.cos(), radian.sin());
    let power = (wind_power as f32 / 255.0) * WIND_POWER;
    let wind_vel = direction * power;

    (wind_angle, wind_power, wind_vel)
}
