use glam::FloatExt;
use protocol::PROJECTILE_SIZE;

use super::*;

const MAX_LOOP: usize = 100;
const TICK: u64 = 1_000 / 15;
const PERIOD: Duration = Duration::from_millis(TICK);

#[derive(Default, Clone, Copy)]
enum GameState {
    #[default]
    LeftTurn,
    RightTurn,
    LeftProjectileThrown {
        hit: bool,
    },
    RightProjectileThrown {
        hit: bool,
    },
}

impl GameState {
    pub fn is_projectile_thrown(&self) -> bool {
        matches!(
            self,
            GameState::LeftProjectileThrown { .. } | GameState::RightProjectileThrown { .. }
        )
    }
}

pub async fn play(mut left: Box<dyn Session>, mut right: Box<dyn Session>, mut num_player: usize) {
    let mut turn_counter = 0;
    let mut left_health = MAX_HEALTH_COUNT;
    let mut right_health = MAX_HEALTH_COUNT;
    let left_collider = COLLIDER_DATA.get(&left.hero()).unwrap();
    let right_collider = COLLIDER_DATA.get(&right.hero()).unwrap();
    let mut control = None;
    let (mut wind_angle, mut wind_power, mut wind_vel) = update_wind_parameter();
    let mut projectile_vel = Vec2::ZERO;
    let mut projectile_pos = Vec2::new(LEFT_THROW_POS_X, LEFT_THROW_POS_Y);
    let mut game_state = GameState::default();
    let mut remaining_millis = MAX_CTRL_TIME;
    let mut total_remaining_millis = MAX_PLAY_TIME;
    let mut interval = time::interval(PERIOD);
    let mut previous_instant = Instant::now();

    let message = Packet::InGameTurnSetup {
        wind_angle,
        wind_power,
    };
    left = send_message(left, &message, &mut num_player);
    right = send_message(right, &message, &mut num_player);
    if num_player == 0 {
        #[cfg(not(feature = "no-debugging-log"))]
        println!("Stop play game");
        return;
    }

    while total_remaining_millis > 0 || game_state.is_projectile_thrown() {
        let instant = interval.tick().await;
        let elapsed = instant
            .saturating_duration_since(previous_instant)
            .as_millis();
        let elapsed_u16 = elapsed.min(u16::MAX as u128) as u16;
        let elapsed_i32 = elapsed.min(i32::MAX as u128) as i32;
        previous_instant = instant;

        total_remaining_millis -= elapsed_i32;

        match left.reader() {
            Some(stream) => {
                let mut cnt = MAX_LOOP;
                'update: while cnt > 0 {
                    match poll_stream_nonblocking(stream) {
                        StreamPollResult::Pending => break,
                        StreamPollResult::Item(message) => {
                            if let Message::Text(s) = message
                                && let Ok(packet) = serde_json::from_str::<Packet>(&s)
                            {
                                match (game_state, packet) {
                                    (
                                        GameState::LeftTurn,
                                        Packet::UpdateThrowParams { angle, power },
                                    ) => {
                                        control = Some((angle, power));
                                    }
                                    (GameState::LeftTurn, Packet::ThrowProjectile) => {
                                        projectile_pos =
                                            Vec2::new(LEFT_THROW_POS_X, LEFT_THROW_POS_Y);
                                        projectile_vel = control
                                            .map(|(angle, power)| {
                                                let delta = angle as f32 / 255.0;
                                                let radian = LEFT_START_ANGLE
                                                    + (LEFT_END_ANGLE - LEFT_START_ANGLE) * delta;
                                                let direction =
                                                    Vec2::new(radian.cos(), radian.sin());
                                                let power = (power as f32 / 255.0) * THROW_POWER;
                                                direction * power
                                            })
                                            .unwrap_or_default();
                                        game_state = GameState::LeftProjectileThrown { hit: false };
                                        remaining_millis = THROW_END_TIME;
                                    }
                                    _ => { /* empty */ }
                                }
                            }
                        }
                        StreamPollResult::Error(e) => {
                            println!("WebSocket disconnected ({:?}): {e}", left);

                            #[cfg(not(feature = "no-debugging-log"))]
                            println!("Left player({:?}) replaced by Bot", left);

                            left = Box::new(Bot::from(left));
                            num_player -= 1;
                            break 'update; // Handle disconnection.
                        }
                        StreamPollResult::Closed => {
                            println!("WebSocket disconnected ({:?})", left);

                            #[cfg(not(feature = "no-debugging-log"))]
                            println!("Left player({:?}) replaced by Bot", left);

                            left = Box::new(Bot::from(left));
                            num_player -= 1;
                            break 'update; // Handle closure.
                        }
                    }
                    cnt -= 1;
                }
            }
            None => {
                // TODO!
            }
        }

        match right.reader() {
            Some(stream) => {
                let mut cnt = MAX_LOOP;
                'update: while cnt > 0 {
                    match poll_stream_nonblocking(stream) {
                        StreamPollResult::Pending => break,
                        StreamPollResult::Item(message) => {
                            if let Message::Text(s) = message
                                && let Ok(packet) = serde_json::from_str::<Packet>(&s)
                            {
                                match (game_state, packet) {
                                    (
                                        GameState::RightTurn,
                                        Packet::UpdateThrowParams { angle, power },
                                    ) => {
                                        control = Some((angle, power));
                                    }
                                    (GameState::RightTurn, Packet::ThrowProjectile) => {
                                        projectile_pos =
                                            Vec2::new(RIGHT_THROW_POS_X, RIGHT_THROW_POS_Y);
                                        projectile_vel = control
                                            .map(|(angle, power)| {
                                                let delta = angle as f32 / 255.0;
                                                let radian = RIGHT_START_ANGLE
                                                    + (RIGHT_END_ANGLE - RIGHT_START_ANGLE) * delta;
                                                let direction =
                                                    Vec2::new(radian.cos(), radian.sin());
                                                let power = (power as f32 / 255.0) * THROW_POWER;
                                                direction * power
                                            })
                                            .unwrap_or_default();
                                        game_state =
                                            GameState::RightProjectileThrown { hit: false };
                                        remaining_millis = THROW_END_TIME;
                                    }
                                    _ => { /* empty */ }
                                }
                            }
                        }
                        StreamPollResult::Error(e) => {
                            println!("WebSocket disconnected ({:?}): {e}", right);

                            #[cfg(not(feature = "no-debugging-log"))]
                            println!("Right player({:?}) replaced by Bot", left);

                            right = Box::new(Bot::from(right));
                            num_player -= 1;
                            break 'update; // Handle disconnection.
                        }
                        StreamPollResult::Closed => {
                            println!("WebSocket disconnected ({:?})", right);

                            #[cfg(not(feature = "no-debugging-log"))]
                            println!("Right player({:?}) replaced by Bot", left);

                            right = Box::new(Bot::from(right));
                            num_player -= 1;
                            break 'update; // Handle closure.
                        }
                    }
                    cnt -= 1;
                }
            }
            None => {
                // TODO!
            }
        }

        if num_player == 0 {
            #[cfg(not(feature = "no-debugging-log"))]
            println!("Stop play game");
            return;
        }

        match game_state {
            GameState::LeftTurn => {
                remaining_millis = remaining_millis.saturating_sub(elapsed_u16);

                let message = Packet::InGameLeftTurn {
                    total_remaining_millis,
                    remaining_millis,
                    left_health_cnt: left_health as u8,
                    right_health_cnt: right_health as u8,
                    control,
                };
                left = send_message(left, &message, &mut num_player);
                right = send_message(right, &message, &mut num_player);
                if num_player == 0 {
                    #[cfg(not(feature = "no-debugging-log"))]
                    println!("Stop play game");
                    return;
                }

                if remaining_millis == 0 {
                    #[cfg(not(feature = "no-debugging-log"))]
                    println!("Left turn ended.");

                    turn_counter += 1;
                    (wind_angle, wind_power, wind_vel) = update_wind_parameter();
                    let message = Packet::InGameTurnSetup {
                        wind_angle,
                        wind_power,
                    };
                    left = send_message(left, &message, &mut num_player);
                    right = send_message(right, &message, &mut num_player);
                    if num_player == 0 {
                        #[cfg(not(feature = "no-debugging-log"))]
                        println!("Stop play game");
                        return;
                    }

                    game_state = GameState::RightTurn;
                    remaining_millis = MAX_CTRL_TIME;
                    control = None;
                }
            }
            GameState::RightTurn => {
                remaining_millis = remaining_millis.saturating_sub(elapsed_u16);

                let message = Packet::InGameRightTurn {
                    total_remaining_millis,
                    remaining_millis,
                    left_health_cnt: left_health as u8,
                    right_health_cnt: right_health as u8,
                    control,
                };
                left = send_message(left, &message, &mut num_player);
                right = send_message(right, &message, &mut num_player);
                if num_player == 0 {
                    #[cfg(not(feature = "no-debugging-log"))]
                    println!("Stop play game");
                    return;
                }

                if remaining_millis == 0 {
                    #[cfg(not(feature = "no-debugging-log"))]
                    println!("Right turn ended.");

                    turn_counter += 1;
                    (wind_angle, wind_power, wind_vel) = update_wind_parameter();
                    let message = Packet::InGameTurnSetup {
                        wind_angle,
                        wind_power,
                    };
                    left = send_message(left, &message, &mut num_player);
                    right = send_message(right, &message, &mut num_player);
                    if num_player == 0 {
                        #[cfg(not(feature = "no-debugging-log"))]
                        println!("Stop play game");
                        return;
                    }

                    game_state = GameState::LeftTurn;
                    remaining_millis = MAX_CTRL_TIME;
                    control = None;
                }
            }
            GameState::LeftProjectileThrown { hit } => {
                let delta_time = elapsed_u16 as f32 / 1000.0;
                projectile_vel.y += GRAVITY * delta_time;
                projectile_pos += (projectile_vel + wind_vel) * delta_time;

                if projectile_pos.y < LEFT_PLAYER_POS_Y {
                    wind_vel = Vec2::ZERO;
                    projectile_vel.x = projectile_vel.x.lerp(0.0, 0.25);
                    projectile_pos.y = LEFT_PLAYER_POS_Y;
                }

                let mut collider_pos: Vec2 = right_collider.center.into();
                collider_pos += Vec2::new(RIGHT_PLAYER_POS_X, RIGHT_PLAYER_POS_Y);
                let distance_squared = (projectile_pos - collider_pos).length_squared();
                let radius_sum = PROJECTILE_SIZE * 0.5 + right_collider.radius;
                if !hit && distance_squared <= radius_sum * radius_sum {
                    game_state = GameState::LeftProjectileThrown { hit: true };
                    right_health -= 1;
                }

                if projectile_pos.y <= LEFT_PLAYER_POS_Y
                    || projectile_pos.x <= WORLD_MIN_X
                    || projectile_pos.x >= WORLD_MAX_X
                {
                    remaining_millis = remaining_millis.saturating_sub(elapsed_u16);
                }

                let message = Packet::InGameProjectileThrown {
                    total_remaining_millis,
                    remaining_millis,
                    left_health_cnt: left_health as u8,
                    right_health_cnt: right_health as u8,
                    projectile_pos: projectile_pos.into(),
                    projectile_vel: projectile_vel.into(),
                };
                left = send_message(left, &message, &mut num_player);
                right = send_message(right, &message, &mut num_player);
                if num_player == 0 {
                    #[cfg(not(feature = "no-debugging-log"))]
                    println!("Stop play game");
                    return;
                }

                if remaining_millis == 0 {
                    #[cfg(not(feature = "no-debugging-log"))]
                    println!("Projectile thrown.");

                    if right_health == 0 {
                        break;
                    }

                    turn_counter += 1;
                    (wind_angle, wind_power, wind_vel) = update_wind_parameter();
                    let message = Packet::InGameTurnSetup {
                        wind_angle,
                        wind_power,
                    };
                    left = send_message(left, &message, &mut num_player);
                    right = send_message(right, &message, &mut num_player);
                    if num_player == 0 {
                        #[cfg(not(feature = "no-debugging-log"))]
                        println!("Stop play game");
                        return;
                    }

                    game_state = match turn_counter % 2 == 0 {
                        true => GameState::LeftTurn,
                        false => GameState::RightTurn,
                    };
                    remaining_millis = MAX_CTRL_TIME;
                    control = None;
                }
            }
            GameState::RightProjectileThrown { hit } => {
                let delta_time = elapsed_u16 as f32 / 1000.0;
                projectile_vel.y += GRAVITY * delta_time;
                projectile_pos += (projectile_vel + wind_vel) * delta_time;

                if projectile_pos.y < LEFT_PLAYER_POS_Y {
                    wind_vel = Vec2::ZERO;
                    projectile_vel.x = projectile_vel.x.lerp(0.0, 0.25);
                    projectile_pos.y = LEFT_PLAYER_POS_Y;
                }

                let mut collider_pos: Vec2 = left_collider.center.into();
                collider_pos += Vec2::new(LEFT_PLAYER_POS_X, LEFT_PLAYER_POS_Y);
                let distance_squared = (projectile_pos - collider_pos).length_squared();
                let radius_sum = PROJECTILE_SIZE * 0.5 + left_collider.radius;
                if !hit && distance_squared <= radius_sum * radius_sum {
                    game_state = GameState::RightProjectileThrown { hit: true };
                    left_health -= 1;
                }

                if projectile_pos.y <= LEFT_PLAYER_POS_Y
                    || projectile_pos.x <= WORLD_MIN_X
                    || projectile_pos.x >= WORLD_MAX_X
                {
                    remaining_millis = remaining_millis.saturating_sub(elapsed_u16);
                }

                let message = Packet::InGameProjectileThrown {
                    total_remaining_millis,
                    remaining_millis,
                    left_health_cnt: left_health as u8,
                    right_health_cnt: right_health as u8,
                    projectile_pos: projectile_pos.into(),
                    projectile_vel: projectile_vel.into(),
                };
                left = send_message(left, &message, &mut num_player);
                right = send_message(right, &message, &mut num_player);
                if num_player == 0 {
                    #[cfg(not(feature = "no-debugging-log"))]
                    println!("Stop play game");
                    return;
                }

                if remaining_millis == 0 {
                    #[cfg(not(feature = "no-debugging-log"))]
                    println!("Projectile thrown.");

                    if left_health == 0 {
                        break;
                    }

                    turn_counter += 1;
                    (wind_angle, wind_power, wind_vel) = update_wind_parameter();
                    let message = Packet::InGameTurnSetup {
                        wind_angle,
                        wind_power,
                    };
                    left = send_message(left, &message, &mut num_player);
                    right = send_message(right, &message, &mut num_player);
                    if num_player == 0 {
                        #[cfg(not(feature = "no-debugging-log"))]
                        println!("Stop play game");
                        return;
                    }

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

    match left_health.cmp(&right_health) {
        std::cmp::Ordering::Less => {
            #[cfg(not(feature = "no-debugging-log"))]
            println!("Right player won!");

            left.increase_lose();
            let message = Packet::GameResult {
                win: left.win(),
                lose: left.lose(),
                victory: false,
            };
            left = send_message(left, &message, &mut num_player);
            let result: Result<Box<Player>, Box<dyn Any>> = left.into_any().downcast();
            if let Ok(player) = result {
                next_state(State::Title, player);
            }

            right.increase_win();
            let message = Packet::GameResult {
                win: right.win(),
                lose: right.lose(),
                victory: true,
            };
            right = send_message(right, &message, &mut num_player);
            let result: Result<Box<Player>, Box<dyn Any>> = right.into_any().downcast();
            if let Ok(player) = result {
                next_state(State::Title, player);
            }
        }
        std::cmp::Ordering::Equal => {
            #[cfg(not(feature = "no-debugging-log"))]
            println!("Draw!");

            let message = Packet::GameResultDraw;
            left.increase_draw();
            left = send_message(left, &message, &mut num_player);
            let result: Result<Box<Player>, Box<dyn Any>> = left.into_any().downcast();
            if let Ok(player) = result {
                next_state(State::Title, player);
            }

            right.increase_draw();
            right = send_message(right, &message, &mut num_player);
            let result: Result<Box<Player>, Box<dyn Any>> = right.into_any().downcast();
            if let Ok(player) = result {
                next_state(State::Title, player);
            }
        }
        std::cmp::Ordering::Greater => {
            #[cfg(not(feature = "no-debugging-log"))]
            println!("Left player won!");

            left.increase_win();
            let message = Packet::GameResult {
                win: left.win(),
                lose: left.lose(),
                victory: true,
            };
            left = send_message(left, &message, &mut num_player);
            let result: Result<Box<Player>, Box<dyn Any>> = left.into_any().downcast();
            if let Ok(player) = result {
                next_state(State::Title, player);
            }

            right.increase_lose();
            let message = Packet::GameResult {
                win: right.win(),
                lose: right.lose(),
                victory: false,
            };
            right = send_message(right, &message, &mut num_player);
            let result: Result<Box<Player>, Box<dyn Any>> = right.into_any().downcast();
            if let Ok(player) = result {
                next_state(State::Title, player);
            }
        }
    }
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
