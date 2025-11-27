mod enter;
mod init;
mod prepare;
mod switch;

// Import necessary Bevy modules.
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use protocol::{
    LEFT_CAM_POS_X, LEFT_END_ANGLE, LEFT_START_ANGLE, LEFT_THROW_POS_X, LEFT_THROW_POS_Y,
    MAX_CTRL_TIME, RIGHT_CAM_POS_X, RIGHT_END_ANGLE, RIGHT_START_ANGLE, RIGHT_THROW_POS_X,
    RIGHT_THROW_POS_Y, THROW_RANGE,
};

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(enter::InnerPlugin)
            .add_plugins(init::InnerPlugin)
            .add_plugins(prepare::InnerPlugin)
            .add_plugins(switch::InnerPlugin)
            .add_systems(OnEnter(LevelStates::InGame), (debug_label, setup_resource))
            .add_systems(
                OnExit(LevelStates::InGame),
                (cleanup_resource, reset_camera_position),
            )
            .add_systems(
                Update,
                (
                    update_hud_ingame_timer,
                    update_hud_player_timer,
                    update_camera_position,
                    update_wind_indicator,
                    draw_range_indicator,
                    draw_range_arrow_indicator,
                )
                    .run_if(in_state(LevelStates::InGame)),
            );

        #[cfg(target_arch = "wasm32")]
        {
            app.add_systems(
                PreUpdate,
                (
                    handle_received_packets,
                    handle_cursor_movement.after(handle_received_packets),
                )
                    .run_if(in_state(LevelStates::InGame)),
            );
        }
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InGame");
}

fn setup_resource(mut commands: Commands) {
    commands.insert_resource(InGameTimer::default());
    commands.insert_resource(PlayerTimer::default());
    commands.insert_resource(PlayerHealth::default());
    commands.insert_resource(PlaySide::default());
    commands.insert_resource(Wind::default());
}

// --- CLEANUP SYSTEMS ---

fn cleanup_resource(mut commands: Commands) {
    commands.remove_resource::<InGameTimer>();
    commands.remove_resource::<PlayerTimer>();
    commands.remove_resource::<PlayerHealth>();
    commands.remove_resource::<PlaySide>();
    commands.remove_resource::<Wind>();
}

fn reset_camera_position(mut query: Query<&mut Transform, With<Camera>>) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    transform.translation.x = 0.0;
}

// --- PREUPDATE SYSTEMS ---

#[cfg(target_arch = "wasm32")]
#[allow(clippy::too_many_arguments)]
fn handle_received_packets(
    mut commands: Commands,
    mut wind: ResMut<Wind>,
    mut side: ResMut<PlaySide>,
    mut health: ResMut<PlayerHealth>,
    mut player_timer: ResMut<PlayerTimer>,
    mut in_game_timer: ResMut<InGameTimer>,
    mut next_state: ResMut<NextState<LevelStates>>,
    network: Res<Network>,
) {
    for result in network.try_iter() {
        match result {
            Ok(packet) => match packet {
                Packet::InGameLeftTurn {
                    total_remaining_millis,
                    remaining_millis,
                    wind_angle,
                    wind_power,
                    left_health,
                    right_health,
                    angle,
                    power,
                } => {
                    *side = PlaySide::Left { angle, power };
                    in_game_timer.miliis = total_remaining_millis;
                    player_timer.miliis = remaining_millis;
                    *wind = Wind::new(wind_angle, wind_power);
                    *health = PlayerHealth::new(left_health, right_health);
                }
                Packet::InGameRightTurn {
                    total_remaining_millis,
                    remaining_millis,
                    wind_angle,
                    wind_power,
                    left_health,
                    right_health,
                    angle,
                    power,
                } => {
                    *side = PlaySide::Right { angle, power };
                    in_game_timer.miliis = total_remaining_millis;
                    player_timer.miliis = remaining_millis;
                    *wind = Wind::new(wind_angle, wind_power);
                    *health = PlayerHealth::new(left_health, right_health);
                }
                Packet::InGameProjectileThrown {
                    total_remaining_millis,
                    wind_angle,
                    wind_power,
                    left_health,
                    right_health,
                    ..
                } => {
                    *side = PlaySide::Thrown;
                    in_game_timer.miliis = total_remaining_millis;
                    *wind = Wind::new(wind_angle, wind_power);
                    *health = PlayerHealth::new(left_health, right_health);
                }
                _ => { /* empty */ }
            },
            Err(e) => {
                commands.insert_resource(ErrorMessage::from(e));
                next_state.set(LevelStates::Error);
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[allow(clippy::too_many_arguments)]
fn handle_cursor_movement(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut play_side: ResMut<PlaySide>,
    other_info: Res<OtherInfo>,
    network: Res<Network>,
) {
    match (*play_side, other_info.left_side) {
        (PlaySide::Left { .. }, false) => {
            if let Ok(window) = windows.single()
                && let Ok((camera, camera_transform)) = cameras.single()
                && let Some(cursor_pos) = window.cursor_position()
            {
                let result = camera.viewport_to_world_2d(camera_transform, cursor_pos);
                let Ok(cursor_pos) = result else { return };

                let start = Vec2::new(LEFT_THROW_POS_X, LEFT_THROW_POS_Y);
                let dist = cursor_pos - start;
                let length = dist.length();
                if length > f32::EPSILON {
                    let angle = dist.to_angle();
                    let clamped_angle = angle.clamp(LEFT_START_ANGLE, LEFT_END_ANGLE);
                    let power = length.clamp(THROW_RANGE, 2.0 * THROW_RANGE);

                    let angle = ((clamped_angle - LEFT_START_ANGLE)
                        / (LEFT_END_ANGLE - LEFT_START_ANGLE)
                        * 255.0) as u8;
                    let power = (power / (2.0 * THROW_RANGE) * 255.0) as u8;

                    network
                        .send(&Packet::UpdateThrowParams { angle, power })
                        .unwrap();

                    *play_side = PlaySide::Left {
                        angle: Some(angle),
                        power: Some(power),
                    };
                }
            };
        }
        (PlaySide::Right { .. }, true) => {
            if let Ok(window) = windows.single()
                && let Ok((camera, camera_transform)) = cameras.single()
                && let Some(cursor_pos) = window.cursor_position()
            {
                let result = camera.viewport_to_world_2d(camera_transform, cursor_pos);
                let Ok(cursor_pos) = result else { return };

                let start = Vec2::new(RIGHT_THROW_POS_X, RIGHT_THROW_POS_Y);
                let dist = cursor_pos - start;
                let length = dist.length();
                if length > f32::EPSILON {
                    let mut angle = dist.to_angle();
                    if angle < 0.0 {
                        angle += TAU;
                    }
                    let clamped_angle = angle.clamp(RIGHT_START_ANGLE, RIGHT_END_ANGLE);
                    let power = length.clamp(THROW_RANGE, 2.0 * THROW_RANGE);

                    let angle = ((clamped_angle - RIGHT_START_ANGLE)
                        / (RIGHT_END_ANGLE - RIGHT_START_ANGLE)
                        * 255.0) as u8;
                    let power = (power / (2.0 * THROW_RANGE) * 255.0) as u8;

                    network
                        .send(&Packet::UpdateThrowParams { angle, power })
                        .unwrap();

                    *play_side = PlaySide::Right {
                        angle: Some(angle),
                        power: Some(power),
                    };
                }
            };
        }
        _ => { /* empty */ }
    }
}

// --- UPDATE SYSTEMS ---

fn update_hud_ingame_timer(
    timer: Res<InGameTimer>,
    mut query: Query<&mut Text, With<HUDInGameTimer>>,
) {
    for mut text in query.iter_mut() {
        let seconds = (timer.miliis as f32 / 1000.0).ceil() as u32;
        *text = Text::new(format!("{:0>3}", seconds));
    }
}

fn update_hud_player_timer(
    timer: Res<PlayerTimer>,
    side: Res<PlaySide>,
    other_info: Res<OtherInfo>,
    mut hud: Query<&mut Visibility, With<HUDPlayerTimer>>,
    mut timer_bar: Query<(&mut Node, &mut BackgroundColor), With<PlayerTimerBar>>,
) {
    let Ok(mut visibility) = hud.single_mut() else {
        return;
    };
    let Ok((mut node, mut color)) = timer_bar.single_mut() else {
        return;
    };

    match (*side, other_info.left_side) {
        (PlaySide::Left { .. }, false) | (PlaySide::Right { .. }, true) => {
            *visibility = Visibility::Visible;
            let p = (timer.miliis as f32 / MAX_CTRL_TIME as f32).clamp(0.0, 1.0);
            node.width = Val::Percent(p * 100.0);

            const MIN_VAL: f32 = 0.2;
            const MAX_VAL: f32 = 0.8;
            let (red, green) = if p < 0.5 {
                let red = MAX_VAL;
                let green = MIN_VAL.lerp(MAX_VAL, p * 2.0);
                (red, green)
            } else {
                let red = MAX_VAL.lerp(MIN_VAL, (p - 0.5) * 2.0);
                let green = MAX_VAL;
                (red, green)
            };
            *color = BackgroundColor(Color::srgb(red, green, MIN_VAL));
        }
        _ => {
            *visibility = Visibility::Hidden;
        }
    }
}

fn update_camera_position(
    mut query: Query<&mut Transform, With<Camera>>,
    play_side: Res<PlaySide>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    transform.translation.x = match *play_side {
        PlaySide::Left { .. } => LEFT_CAM_POS_X.lerp(transform.translation.x, 0.8),
        PlaySide::Right { .. } => RIGHT_CAM_POS_X.lerp(transform.translation.x, 0.8),
        PlaySide::Thrown => transform.translation.x,
    };
}

fn update_wind_indicator(mut query: Query<&mut UiTransform, With<WindIndicator>>, wind: Res<Wind>) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    transform.scale = wind.get_scale();
    transform.rotation = wind.get_rotation();
}

fn draw_range_indicator(play_side: Res<PlaySide>, mut painter: ShapePainter) {
    match *play_side {
        PlaySide::Left { .. } => {
            painter.cap = Cap::None;
            painter.hollow = true;
            painter.thickness = THROW_RANGE;
            painter.set_color(Color::WHITE.with_alpha(0.5));
            painter.set_translation(Vec3::new(LEFT_THROW_POS_X, LEFT_THROW_POS_Y, 0.6));

            let start_angle = FRAC_PI_2 - LEFT_END_ANGLE;
            let end_angle = FRAC_PI_2 - LEFT_START_ANGLE;
            painter.arc(2.0 * THROW_RANGE, start_angle, end_angle);
        }
        PlaySide::Right { .. } => {
            painter.cap = Cap::None;
            painter.hollow = true;
            painter.thickness = THROW_RANGE;
            painter.set_color(Color::WHITE.with_alpha(0.5));
            painter.set_translation(Vec3::new(RIGHT_THROW_POS_X, RIGHT_THROW_POS_Y, 0.6));

            let start_angle = FRAC_PI_2 - RIGHT_END_ANGLE;
            let end_angle = FRAC_PI_2 - RIGHT_START_ANGLE;
            painter.arc(2.0 * THROW_RANGE, start_angle, end_angle);
        }
        PlaySide::Thrown => { /* empty */ }
    }
}

fn draw_range_arrow_indicator(play_side: Res<PlaySide>, mut painter: ShapePainter) {
    let (angle, power) = match *play_side {
        PlaySide::Left { angle, power } => (angle, power),
        PlaySide::Right { angle, power } => (angle, power),
        PlaySide::Thrown => return,
    };

    if let (Some(angle), Some(power)) = (angle, power) {
        painter.cap = Cap::Round;
        painter.thickness = 12.0;
        painter.set_color(BG_RED_COLOR_0);

        let (start_pos, start_angle, end_angle) = if matches!(*play_side, PlaySide::Left { .. }) {
            (
                Vec3::new(LEFT_THROW_POS_X, LEFT_THROW_POS_Y, 0.7),
                LEFT_START_ANGLE,
                LEFT_END_ANGLE,
            )
        } else {
            (
                Vec3::new(RIGHT_THROW_POS_X, RIGHT_THROW_POS_Y, 0.7),
                RIGHT_START_ANGLE,
                RIGHT_END_ANGLE,
            )
        };

        // u8 값을 다시 각도와 길이로 변환
        let t = angle as f32 / 255.0;
        let angle_rad = start_angle.lerp(end_angle, t);

        let p = power as f32 / 255.0;
        let length = 2.0 * THROW_RANGE * p;

        let end = Vec3::new(angle_rad.cos() * length, angle_rad.sin() * length, 0.0);

        painter.set_translation(start_pos);
        painter.line(Vec3::ZERO, end);
    }
}
