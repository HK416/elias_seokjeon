mod enter;
mod init;
mod prepare;
mod result;
mod switch;

// Import necessary Bevy modules.
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use protocol::{
    GRAVITY, Hero, LEFT_CAM_POS_X, LEFT_END_ANGLE, LEFT_PLAYER_POS_Y, LEFT_START_ANGLE,
    LEFT_THROW_POS_X, LEFT_THROW_POS_Y, MAX_CTRL_TIME, RIGHT_CAM_POS_X, RIGHT_END_ANGLE,
    RIGHT_START_ANGLE, RIGHT_THROW_POS_X, RIGHT_THROW_POS_Y,
};

use crate::assets::sound::SystemVolume;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(enter::InnerPlugin)
            .add_plugins(init::InnerPlugin)
            .add_plugins(prepare::InnerPlugin)
            .add_plugins(result::InnerPlugin)
            .add_plugins(switch::InnerPlugin)
            .add_systems(
                OnEnter(LevelStates::InGame),
                (
                    debug_label,
                    setup_resource,
                    cleanup_title_assets,
                    cleanup_title_entities,
                ),
            )
            .add_systems(
                OnExit(LevelStates::InGame),
                (cleanup_resource, reset_camera_position),
            )
            .add_systems(
                Update,
                (
                    update_hud_ingame_timer,
                    update_hud_player_timer,
                    update_wind_indicator.run_if(resource_exists::<Wind>),
                    draw_range_indicator,
                    draw_range_arrow_indicator,
                    update_left_health_heart
                        .run_if(resource_exists_and_changed::<LeftPlayerHealth>),
                    update_right_health_heart
                        .run_if(resource_exists_and_changed::<RightPlayerHealth>),
                    play_timer_sound.run_if(resource_exists_and_changed::<Wind>),
                    play_swing_sound.run_if(resource_added::<ProjectileObject>),
                    setup_projectile.run_if(resource_added::<ProjectileObject>),
                    update_projectile
                        .run_if(resource_exists::<Wind>)
                        .run_if(resource_exists::<ProjectileObject>),
                    cleanup_projectile.run_if(resource_removed::<ProjectileObject>),
                    highlight_my_character_position
                        .run_if(not(resource_exists::<TouchPressed>))
                        .run_if(not(resource_exists::<MouseButtonPressed>))
                        .run_if(not(resource_exists::<ProjectileObject>)),
                )
                    .run_if(in_state(LevelStates::InGame)),
            )
            .add_systems(
                FixedUpdate,
                check_collisions.run_if(in_state(LevelStates::InGame)),
            )
            .add_systems(
                PostUpdate,
                (update_camera_position).run_if(in_state(LevelStates::InGame)),
            );

        #[cfg(target_arch = "wasm32")]
        {
            app.add_systems(
                PreUpdate,
                (
                    handle_received_packets,
                    (
                        handle_mouse_button_pressed
                            .run_if(not(resource_exists::<MouseButtonPressed>)),
                        handle_mouse_button_released
                            .run_if(not(resource_exists::<TouchPressed>))
                            .run_if(resource_exists::<MouseButtonPressed>),
                        handle_cursor_movement
                            .run_if(not(resource_exists::<TouchPressed>))
                            .run_if(resource_exists::<MouseButtonPressed>),
                        handle_touch_pressed.run_if(not(resource_exists::<TouchPressed>)),
                        handle_touch_released
                            .run_if(not(resource_exists::<MouseButtonPressed>))
                            .run_if(resource_exists::<TouchPressed>),
                        handle_touch_movement
                            .run_if(not(resource_exists::<MouseButtonPressed>))
                            .run_if(resource_exists::<TouchPressed>),
                    )
                        .after(handle_received_packets),
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
    commands.insert_resource(LeftPlayerHealth::default());
    commands.insert_resource(RightPlayerHealth::default());
    commands.insert_resource(PlaySide::default());
}

fn cleanup_title_assets(mut commands: Commands) {
    commands.remove_resource::<TitleAssets>();
}

fn cleanup_title_entities(mut commands: Commands, query: Query<Entity, With<TitleLevelRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// --- CLEANUP SYSTEMS ---

fn cleanup_resource(mut commands: Commands) {
    commands.remove_resource::<ProjectileObject>();
    commands.remove_resource::<InGameTimer>();
    commands.remove_resource::<PlayerTimer>();
    commands.remove_resource::<LeftPlayerHealth>();
    commands.remove_resource::<RightPlayerHealth>();
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
    mut side: ResMut<PlaySide>,
    mut left_health: ResMut<LeftPlayerHealth>,
    mut right_health: ResMut<RightPlayerHealth>,
    mut player_info: ResMut<PlayerInfo>,
    mut player_timer: ResMut<PlayerTimer>,
    mut in_game_timer: ResMut<InGameTimer>,
    mut projectile: Option<ResMut<ProjectileObject>>,
    mut next_state: ResMut<NextState<LevelStates>>,
    network: Res<Network>,
) {
    for result in network.try_iter() {
        match result {
            Ok(packet) => match packet {
                Packet::InGameLeftTurn {
                    total_remaining_millis,
                    remaining_millis,
                    left_health_cnt,
                    right_health_cnt,
                    control,
                } => {
                    *side = PlaySide::Left(control);
                    in_game_timer.miliis = total_remaining_millis;
                    player_timer.miliis = remaining_millis;
                    if left_health.0 != left_health_cnt as usize {
                        left_health.0 = left_health_cnt as usize;
                    }
                    if right_health.0 != right_health_cnt as usize {
                        right_health.0 = right_health_cnt as usize;
                    }
                }
                Packet::InGameRightTurn {
                    total_remaining_millis,
                    remaining_millis,
                    left_health_cnt,
                    right_health_cnt,
                    control,
                } => {
                    *side = PlaySide::Right(control);
                    in_game_timer.miliis = total_remaining_millis;
                    player_timer.miliis = remaining_millis;
                    if left_health.0 != left_health_cnt as usize {
                        left_health.0 = left_health_cnt as usize;
                    }
                    if right_health.0 != right_health_cnt as usize {
                        right_health.0 = right_health_cnt as usize;
                    }
                }
                Packet::InGameTurnSetup {
                    wind_angle,
                    wind_power,
                } => {
                    commands.insert_resource(Wind::new(wind_angle, wind_power));
                    commands.remove_resource::<MouseButtonPressed>();
                    commands.remove_resource::<ProjectileObject>();
                }
                Packet::InGameProjectileThrown {
                    total_remaining_millis,
                    remaining_millis,
                    left_health_cnt,
                    right_health_cnt,
                    projectile_pos,
                    projectile_vel,
                } => {
                    *side = match *side {
                        PlaySide::Left(_) => PlaySide::LeftThrown,
                        PlaySide::Right(_) => PlaySide::RightThrown,
                        _ => *side,
                    };
                    in_game_timer.miliis = total_remaining_millis;
                    if left_health.0 != left_health_cnt as usize {
                        left_health.0 = left_health_cnt as usize;
                    }
                    if right_health.0 != right_health_cnt as usize {
                        right_health.0 = right_health_cnt as usize;
                    }

                    match projectile {
                        Some(ref mut projectile) => {
                            projectile.add_snapshot(
                                total_remaining_millis,
                                remaining_millis,
                                projectile_pos.into(),
                                projectile_vel.into(),
                            );
                        }
                        None => {
                            commands.insert_resource(ProjectileObject::new(
                                total_remaining_millis,
                                remaining_millis,
                                projectile_pos.into(),
                                projectile_vel.into(),
                            ));
                        }
                    }
                }
                Packet::GameResult { win, lose, victory } => {
                    player_info.win = win;
                    player_info.lose = lose;
                    if victory {
                        next_state.set(LevelStates::SwitchToGameVictory);
                    } else {
                        next_state.set(LevelStates::SwitchToGameDefeat);
                    }
                }
                Packet::GameResultDraw => {
                    next_state.set(LevelStates::SwitchToGameDraw);
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
fn handle_mouse_button_pressed(
    mut commands: Commands,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    left_player_trigger: Query<(&Collider2d, &GlobalTransform), With<LeftPlayerTrigger>>,
    right_player_trigger: Query<(&Collider2d, &GlobalTransform), With<RightPlayerTrigger>>,
    mouse_button_events: Res<ButtonInput<MouseButton>>,
    other_info: Res<OtherInfo>,
    play_side: Res<PlaySide>,
    network: Res<Network>,
) {
    match (*play_side, other_info.left_side) {
        (PlaySide::Left(_), false) => {
            if mouse_button_events.just_pressed(MouseButton::Left)
                && let Ok(window) = windows.single()
                && let Ok((camera, camera_transform)) = cameras.single()
                && let Ok((collider, transform)) = left_player_trigger.single()
                && let Some(viewport_position) = window.cursor_position()
                && let Ok(point) = camera.viewport_to_world_2d(camera_transform, viewport_position)
                && Collider2d::contains((collider, transform), point)
            {
                network
                    .send(&Packet::UpdateThrowParams { angle: 0, power: 0 })
                    .unwrap();
                commands.insert_resource(MouseButtonPressed);
            }
        }
        (PlaySide::Right(_), true) => {
            if mouse_button_events.just_pressed(MouseButton::Left)
                && let Ok(window) = windows.single()
                && let Ok((camera, camera_transform)) = cameras.single()
                && let Ok((collider, transform)) = right_player_trigger.single()
                && let Some(viewport_position) = window.cursor_position()
                && let Ok(point) = camera.viewport_to_world_2d(camera_transform, viewport_position)
                && Collider2d::contains((collider, transform), point)
            {
                network
                    .send(&Packet::UpdateThrowParams { angle: 0, power: 0 })
                    .unwrap();
                commands.insert_resource(MouseButtonPressed);
            }
        }
        _ => { /* empty */ }
    }
}

#[cfg(target_arch = "wasm32")]
#[allow(clippy::too_many_arguments)]
fn handle_mouse_button_released(
    mut commands: Commands,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_button_events: Res<ButtonInput<MouseButton>>,
    other_info: Res<OtherInfo>,
    network: Res<Network>,
) {
    if mouse_button_events.just_released(MouseButton::Left) {
        commands.remove_resource::<MouseButtonPressed>();

        if let Ok(window) = windows.single()
            && let Ok((camera, camera_transform)) = cameras.single()
            && let Some(viewport_position) = window.cursor_position()
            && let Ok(point) = camera.viewport_to_world_2d(camera_transform, viewport_position)
        {
            let center = match other_info.left_side {
                true => Vec2::new(RIGHT_THROW_POS_X, RIGHT_THROW_POS_Y),
                false => Vec2::new(LEFT_THROW_POS_X, LEFT_THROW_POS_Y),
            };
            let dist = center - point;
            let norm = dist.try_normalize().unwrap_or(Vec2::X);
            let length = dist.length().min(THROW_RANGE);
            let delta = length / THROW_RANGE;
            let power = (delta * 255.0) as u8;

            let mut angle = norm.to_angle();
            let angle = if other_info.left_side {
                // I'm right side player.
                if angle < 0.0 {
                    angle += TAU;
                }

                let clamped_angle = angle.clamp(RIGHT_START_ANGLE, RIGHT_END_ANGLE);
                let delta =
                    (clamped_angle - RIGHT_START_ANGLE) / (RIGHT_END_ANGLE - RIGHT_START_ANGLE);
                (delta * 255.0) as u8
            } else {
                // I'm left side player.
                let clamped_angle = angle.clamp(LEFT_START_ANGLE, LEFT_END_ANGLE);
                let delta =
                    (clamped_angle - LEFT_START_ANGLE) / (LEFT_END_ANGLE - LEFT_START_ANGLE);
                (delta * 255.0) as u8
            };

            network
                .send(&Packet::UpdateThrowParams { angle, power })
                .unwrap();
        }
        network.send(&Packet::ThrowProjectile).unwrap();
    }
}

#[cfg(target_arch = "wasm32")]
fn handle_cursor_movement(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut play_side: ResMut<PlaySide>,
    other_info: Res<OtherInfo>,
    network: Res<Network>,
) {
    match (*play_side, other_info.left_side) {
        (PlaySide::Left(_), false) => {
            if let Ok(window) = windows.single()
                && let Ok((camera, camera_transform)) = cameras.single()
                && let Some(viewport_position) = window.cursor_position()
                && let Ok(point) = camera.viewport_to_world_2d(camera_transform, viewport_position)
            {
                let center = Vec2::new(LEFT_THROW_POS_X, LEFT_THROW_POS_Y);
                let dist = center - point;
                let norm = dist.try_normalize().unwrap_or(Vec2::X);
                let length = dist.length().min(THROW_RANGE);
                let delta = length / THROW_RANGE;
                let power = (delta * 255.0) as u8;

                let angle = norm.to_angle();

                let clamped_angle = angle.clamp(LEFT_START_ANGLE, LEFT_END_ANGLE);
                let delta =
                    (clamped_angle - LEFT_START_ANGLE) / (LEFT_END_ANGLE - LEFT_START_ANGLE);
                let angle = (delta * 255.0) as u8;

                *play_side = PlaySide::Left(Some((angle, power)));
                network
                    .send(&Packet::UpdateThrowParams { angle, power })
                    .unwrap();
            }
        }
        (PlaySide::Right(_), true) => {
            if let Ok(window) = windows.single()
                && let Ok((camera, camera_transform)) = cameras.single()
                && let Some(viewport_position) = window.cursor_position()
                && let Ok(point) = camera.viewport_to_world_2d(camera_transform, viewport_position)
            {
                let center = Vec2::new(RIGHT_THROW_POS_X, RIGHT_THROW_POS_Y);
                let dist = center - point;
                let norm = dist.try_normalize().unwrap_or(Vec2::X);
                let length = dist.length().min(THROW_RANGE);
                let delta = length / THROW_RANGE;
                let power = (delta * 255.0) as u8;

                let mut angle = norm.to_angle();
                if angle < 0.0 {
                    angle += TAU;
                }

                let clamped_angle = angle.clamp(RIGHT_START_ANGLE, RIGHT_END_ANGLE);
                let delta =
                    (clamped_angle - RIGHT_START_ANGLE) / (RIGHT_END_ANGLE - RIGHT_START_ANGLE);
                let angle = (delta * 255.0) as u8;

                *play_side = PlaySide::Right(Some((angle, power)));
                network
                    .send(&Packet::UpdateThrowParams { angle, power })
                    .unwrap();
            }
        }
        _ => { /* empty */ }
    }
}

#[cfg(target_arch = "wasm32")]
#[allow(clippy::too_many_arguments)]
fn handle_touch_pressed(
    mut commands: Commands,
    cameras: Query<(&Camera, &GlobalTransform)>,
    left_player_trigger: Query<(&Collider2d, &GlobalTransform), With<LeftPlayerTrigger>>,
    right_player_trigger: Query<(&Collider2d, &GlobalTransform), With<RightPlayerTrigger>>,
    touches: Res<Touches>,
    other_info: Res<OtherInfo>,
    play_side: Res<PlaySide>,
    network: Res<Network>,
) {
    match (*play_side, other_info.left_side) {
        (PlaySide::Left(_), false) => {
            for touch in touches.iter() {
                if let Ok((camera, camera_transform)) = cameras.single()
                    && let Ok((collider, transform)) = left_player_trigger.single()
                    && let Ok(point) =
                        camera.viewport_to_world_2d(camera_transform, touch.position())
                    && Collider2d::contains((collider, transform), point)
                {
                    network
                        .send(&Packet::UpdateThrowParams { angle: 0, power: 0 })
                        .unwrap();
                    commands.insert_resource(TouchPressed { id: touch.id() });
                    break;
                }
            }
        }
        (PlaySide::Right(_), true) => {
            for touch in touches.iter() {
                if let Ok((camera, camera_transform)) = cameras.single()
                    && let Ok((collider, transform)) = right_player_trigger.single()
                    && let Ok(point) =
                        camera.viewport_to_world_2d(camera_transform, touch.position())
                    && Collider2d::contains((collider, transform), point)
                {
                    network
                        .send(&Packet::UpdateThrowParams { angle: 0, power: 0 })
                        .unwrap();
                    commands.insert_resource(TouchPressed { id: touch.id() });
                    break;
                }
            }
        }
        _ => { /* empty */ }
    }
}

#[cfg(target_arch = "wasm32")]
#[allow(clippy::too_many_arguments)]
fn handle_touch_released(
    mut commands: Commands,
    cameras: Query<(&Camera, &GlobalTransform)>,
    touches: Res<Touches>,
    touch_pressed: Res<TouchPressed>,
    other_info: Res<OtherInfo>,
    network: Res<Network>,
) {
    if let Some(touch) = touches.get_released(touch_pressed.id) {
        commands.remove_resource::<TouchPressed>();

        if let Ok((camera, camera_transform)) = cameras.single()
            && let Ok(point) = camera.viewport_to_world_2d(camera_transform, touch.position())
        {
            let center = match other_info.left_side {
                true => Vec2::new(RIGHT_THROW_POS_X, RIGHT_THROW_POS_Y),
                false => Vec2::new(LEFT_THROW_POS_X, LEFT_THROW_POS_Y),
            };
            let dist = center - point;
            let norm = dist.try_normalize().unwrap_or(Vec2::X);
            let length = dist.length().min(THROW_RANGE);
            let delta = length / THROW_RANGE;
            let power = (delta * 255.0) as u8;

            let mut angle = norm.to_angle();
            let angle = if other_info.left_side {
                // I'm right side player.
                if angle < 0.0 {
                    angle += TAU;
                }

                let clamped_angle = angle.clamp(RIGHT_START_ANGLE, RIGHT_END_ANGLE);
                let delta =
                    (clamped_angle - RIGHT_START_ANGLE) / (RIGHT_END_ANGLE - RIGHT_START_ANGLE);
                (delta * 255.0) as u8
            } else {
                // I'm left side player.
                let clamped_angle = angle.clamp(LEFT_START_ANGLE, LEFT_END_ANGLE);
                let delta =
                    (clamped_angle - LEFT_START_ANGLE) / (LEFT_END_ANGLE - LEFT_START_ANGLE);
                (delta * 255.0) as u8
            };

            network
                .send(&Packet::UpdateThrowParams { angle, power })
                .unwrap();
        }
        network.send(&Packet::ThrowProjectile).unwrap();
    }
}

#[cfg(target_arch = "wasm32")]
fn handle_touch_movement(
    cameras: Query<(&Camera, &GlobalTransform)>,
    touches: Res<Touches>,
    touch_pressed: Res<TouchPressed>,
    mut play_side: ResMut<PlaySide>,
    other_info: Res<OtherInfo>,
    network: Res<Network>,
) {
    match (*play_side, other_info.left_side) {
        (PlaySide::Left(_), false) => {
            if let Ok((camera, camera_transform)) = cameras.single()
                && let Some(touch) = touches.get_pressed(touch_pressed.id)
                && let Ok(point) = camera.viewport_to_world_2d(camera_transform, touch.position())
            {
                let center = Vec2::new(LEFT_THROW_POS_X, LEFT_THROW_POS_Y);
                let dist = center - point;
                let norm = dist.try_normalize().unwrap_or(Vec2::X);
                let length = dist.length().min(THROW_RANGE);
                let delta = length / THROW_RANGE;
                let power = (delta * 255.0) as u8;

                let angle = norm.to_angle();

                let clamped_angle = angle.clamp(LEFT_START_ANGLE, LEFT_END_ANGLE);
                let delta =
                    (clamped_angle - LEFT_START_ANGLE) / (LEFT_END_ANGLE - LEFT_START_ANGLE);
                let angle = (delta * 255.0) as u8;

                *play_side = PlaySide::Left(Some((angle, power)));
                network
                    .send(&Packet::UpdateThrowParams { angle, power })
                    .unwrap();
            }
        }
        (PlaySide::Right(_), true) => {
            if let Ok((camera, camera_transform)) = cameras.single()
                && let Some(touch) = touches.get_pressed(touch_pressed.id)
                && let Ok(point) = camera.viewport_to_world_2d(camera_transform, touch.position())
            {
                let center = Vec2::new(RIGHT_THROW_POS_X, RIGHT_THROW_POS_Y);
                let dist = center - point;
                let norm = dist.try_normalize().unwrap_or(Vec2::X);
                let length = dist.length().min(THROW_RANGE);
                let delta = length / THROW_RANGE;
                let power = (delta * 255.0) as u8;

                let mut angle = norm.to_angle();
                if angle < 0.0 {
                    angle += TAU;
                }

                let clamped_angle = angle.clamp(RIGHT_START_ANGLE, RIGHT_END_ANGLE);
                let delta =
                    (clamped_angle - RIGHT_START_ANGLE) / (RIGHT_END_ANGLE - RIGHT_START_ANGLE);
                let angle = (delta * 255.0) as u8;

                *play_side = PlaySide::Right(Some((angle, power)));
                network
                    .send(&Packet::UpdateThrowParams { angle, power })
                    .unwrap();
            }
        }
        _ => { /* empty */ }
    }
}

// --- UPDATE SYSTEMS ---

fn update_hud_ingame_timer(
    timer: Res<InGameTimer>,
    mut query: Query<&mut Text, With<RemainingTimer>>,
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
    mut hud: Query<&mut Visibility, With<UiTurnTimer>>,
    mut timer_bar: Query<(&mut Node, &mut BackgroundColor), With<TurnTimer>>,
) {
    let Ok(mut visibility) = hud.single_mut() else {
        return;
    };
    let Ok((mut node, mut color)) = timer_bar.single_mut() else {
        return;
    };

    match (*side, other_info.left_side) {
        (PlaySide::Left(_), false) | (PlaySide::Right(_), true) => {
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

fn update_wind_indicator(
    mut query: Query<&mut UiTransform, With<WindIndicator>>,
    wind: Res<Wind>,
    time: Res<Time>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    let t = wind.get_power();
    let cycle = time.elapsed_secs() * 20.0 * t;
    let offset = PI * 0.025 * t * cycle.sin();

    transform.scale = Vec2::splat(t);
    transform.rotation = wind.get_rotation(offset);
}

#[allow(clippy::too_many_arguments)]
fn update_left_health_heart(
    mut commands: Commands,
    health_cnt: Res<LeftPlayerHealth>,
    left_health_1: Query<Entity, With<LeftHealth1>>,
    left_health_2: Query<Entity, With<LeftHealth2>>,
    left_health_3: Query<Entity, With<LeftHealth3>>,
    left_health_4: Query<Entity, With<LeftHealth4>>,
    left_health_5: Query<Entity, With<LeftHealth5>>,
) {
    match health_cnt.0 {
        4 => {
            if let Ok(entity) = left_health_5.single() {
                commands.entity(entity).insert(UiSmoothScale::new(
                    UI_POPUP_DURATION,
                    Vec2::ONE,
                    Vec2::ZERO,
                ));
            }
        }
        3 => {
            if let Ok(entity) = left_health_4.single() {
                commands.entity(entity).insert(UiSmoothScale::new(
                    UI_POPUP_DURATION,
                    Vec2::ONE,
                    Vec2::ZERO,
                ));
            }
        }
        2 => {
            if let Ok(entity) = left_health_3.single() {
                commands.entity(entity).insert(UiSmoothScale::new(
                    UI_POPUP_DURATION,
                    Vec2::ONE,
                    Vec2::ZERO,
                ));
            }
        }
        1 => {
            if let Ok(entity) = left_health_2.single() {
                commands.entity(entity).insert(UiSmoothScale::new(
                    UI_POPUP_DURATION,
                    Vec2::ONE,
                    Vec2::ZERO,
                ));
            }
        }
        0 => {
            if let Ok(entity) = left_health_1.single() {
                commands.entity(entity).insert(UiSmoothScale::new(
                    UI_POPUP_DURATION,
                    Vec2::ONE,
                    Vec2::ZERO,
                ));
            }
        }
        _ => { /* empty */ }
    }
}

#[allow(clippy::too_many_arguments)]
fn update_right_health_heart(
    mut commands: Commands,
    health_cnt: Res<RightPlayerHealth>,
    right_health_1: Query<Entity, With<RightHealth1>>,
    right_health_2: Query<Entity, With<RightHealth2>>,
    right_health_3: Query<Entity, With<RightHealth3>>,
    right_health_4: Query<Entity, With<RightHealth4>>,
    right_health_5: Query<Entity, With<RightHealth5>>,
) {
    match health_cnt.0 {
        4 => {
            if let Ok(entity) = right_health_5.single() {
                commands.entity(entity).insert(UiSmoothScale::new(
                    UI_POPUP_DURATION,
                    Vec2::ONE,
                    Vec2::ZERO,
                ));
            }
        }
        3 => {
            if let Ok(entity) = right_health_4.single() {
                commands.entity(entity).insert(UiSmoothScale::new(
                    UI_POPUP_DURATION,
                    Vec2::ONE,
                    Vec2::ZERO,
                ));
            }
        }
        2 => {
            if let Ok(entity) = right_health_3.single() {
                commands.entity(entity).insert(UiSmoothScale::new(
                    UI_POPUP_DURATION,
                    Vec2::ONE,
                    Vec2::ZERO,
                ));
            }
        }
        1 => {
            if let Ok(entity) = right_health_2.single() {
                commands.entity(entity).insert(UiSmoothScale::new(
                    UI_POPUP_DURATION,
                    Vec2::ONE,
                    Vec2::ZERO,
                ));
            }
        }
        0 => {
            if let Ok(entity) = right_health_1.single() {
                commands.entity(entity).insert(UiSmoothScale::new(
                    UI_POPUP_DURATION,
                    Vec2::ONE,
                    Vec2::ZERO,
                ));
            }
        }
        _ => { /* empty */ }
    }
}

fn draw_range_indicator(play_side: Res<PlaySide>, mut painter: ShapePainter) {
    match *play_side {
        PlaySide::Left(_) => {
            painter.cap = Cap::None;
            painter.hollow = true;
            painter.thickness = THROW_RANGE * 0.5;
            painter.set_color(Color::WHITE.with_alpha(0.5));
            painter.set_translation(Vec3::new(LEFT_THROW_POS_X, LEFT_THROW_POS_Y, 0.6));

            let start_angle = FRAC_PI_2 - LEFT_END_ANGLE;
            let end_angle = FRAC_PI_2 - LEFT_START_ANGLE;
            painter.arc(THROW_RANGE, start_angle, end_angle);
        }
        PlaySide::Right(_) => {
            painter.cap = Cap::None;
            painter.hollow = true;
            painter.thickness = THROW_RANGE * 0.5;
            painter.set_color(Color::WHITE.with_alpha(0.5));
            painter.set_translation(Vec3::new(RIGHT_THROW_POS_X, RIGHT_THROW_POS_Y, 0.6));

            let start_angle = FRAC_PI_2 - RIGHT_END_ANGLE;
            let end_angle = FRAC_PI_2 - RIGHT_START_ANGLE;
            painter.arc(THROW_RANGE, start_angle, end_angle);
        }
        _ => { /* empty */ }
    }
}

fn draw_range_arrow_indicator(play_side: Res<PlaySide>, mut painter: ShapePainter) {
    let control = match *play_side {
        PlaySide::Left(control) => control,
        PlaySide::Right(control) => control,
        _ => return,
    };

    if let Some((angle, power)) = control {
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
        let length = THROW_RANGE * p;

        let end = Vec3::new(angle_rad.cos() * length, angle_rad.sin() * length, 0.0);

        painter.set_translation(start_pos);
        painter.line(Vec3::ZERO, end);
    }
}

fn highlight_my_character_position(
    play_side: Res<PlaySide>,
    other_info: Res<OtherInfo>,
    timer: Res<PlayerTimer>,
    mut painter: ShapePainter,
) {
    match (*play_side, other_info.left_side) {
        (PlaySide::Left(_), false) => {
            let delta = timer.miliis as f32 / 1000.0 * 0.5 * PI;
            let radius = 130.0 + 10.0 * (delta * 4.0).sin();

            painter.cap = Cap::None;
            painter.hollow = true;
            painter.thickness = 4.0;
            painter.set_color(BG_RED_COLOR_0);
            painter.set_translation(Vec3::new(LEFT_THROW_POS_X, LEFT_THROW_POS_Y, 0.7));
            painter.circle(radius);
        }
        (PlaySide::Right(_), true) => {
            let delta = timer.miliis as f32 / 1000.0 * 0.5 * PI;
            let radius = 100.0 + 20.0 * (delta * 4.0).sin();

            painter.cap = Cap::None;
            painter.hollow = true;
            painter.thickness = 4.0;
            painter.set_color(BG_RED_COLOR_0);
            painter.set_translation(Vec3::new(RIGHT_THROW_POS_X, RIGHT_THROW_POS_Y, 0.7));
            painter.circle(radius);
        }
        _ => { /* empty */ }
    }
}

fn play_timer_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
) {
    let source = asset_server.load(SFX_PATH_INGAME_TIME_OVER);
    play_effect_sound(&mut commands, &system_volume, source);
}

fn play_swing_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
) {
    let source = asset_server.load(SFX_PATH_SWING);
    play_effect_sound(&mut commands, &system_volume, source);
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn setup_projectile(
    mut commands: Commands,
    play_side: Res<PlaySide>,
    other_info: Res<OtherInfo>,
    player_info: ResMut<PlayerInfo>,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    mut query: Query<(
        &mut Visibility,
        &mut Sprite,
        &mut Transform,
        &mut Projectile,
    )>,
    projectile: Res<ProjectileObject>,
) {
    if let Some(snapshot) = projectile.front()
        && let Ok((mut visibility, mut sprite, mut transform, mut projectile)) = query.single_mut()
    {
        *visibility = Visibility::Hidden;
        sprite.color = sprite.color.with_alpha(1.0);
        transform.rotation = Quat::IDENTITY;
        transform.translation.x = snapshot.position.x;
        transform.translation.y = snapshot.position.y;
        transform.translation.z = 0.8;
        projectile.hit = false;

        match (*play_side, other_info.left_side) {
            (PlaySide::LeftThrown, false) | (PlaySide::RightThrown, true) => {
                let hero = player_info.hero;
                let path = HERO_VOICE_SETS[hero as usize]
                    .shout()
                    .choose(&mut rand::rng())
                    .copied()
                    .unwrap();
                let source = asset_server.load(path);
                play_voice_sound(&mut commands, &system_volume, source, VoiceChannel::MySelf);
            }
            (PlaySide::LeftThrown, true) | (PlaySide::RightThrown, false) => {
                let hero = other_info.hero;
                let path = HERO_VOICE_SETS[hero as usize]
                    .shout()
                    .choose(&mut rand::rng())
                    .copied()
                    .unwrap();
                let source = asset_server.load(path);
                play_voice_sound(&mut commands, &system_volume, source, VoiceChannel::Other);
            }
            _ => { /* empty */ }
        };
    }
}

fn update_projectile(
    mut query: Query<(&mut Sprite, &mut Visibility, &mut Transform), With<Projectile>>,
    mut projectile: ResMut<ProjectileObject>,
    play_side: Res<PlaySide>,
    wind: Res<Wind>,
    time: Res<Time>,
) {
    let elapsed_time = time.delta().as_millis().min(i32::MAX as u128) as i32;
    let (timepoint, prev, next) = projectile.get(elapsed_time);
    match (prev, next) {
        (Some(prev), Some(next)) => {
            let range = prev.timepoint - next.timepoint;
            let t = (prev.timepoint - timepoint) as f32 / range as f32;
            let position = prev.position.lerp(next.position, t);
            let alpha = projectile.get_alpha();

            if let Ok((mut sprite, mut visibility, mut transform)) = query.single_mut() {
                sprite.color = sprite.color.with_alpha(alpha);
                *visibility = Visibility::Visible;
                transform.translation.x = position.x;
                transform.translation.y = position.y;
                transform.translation.z = 0.8;

                if position.y > LEFT_PLAYER_POS_Y {
                    let angle = match *play_side {
                        PlaySide::LeftThrown => -time.delta_secs(),
                        PlaySide::RightThrown => time.delta_secs(),
                        _ => 0.0,
                    };

                    transform.rotate_z(angle);
                }
            }
        }
        (Some(prev), None) => {
            let t = prev.timepoint - timepoint;
            let delta_seconds = t as f32 / 1000.0;
            let mut position = prev.position;
            let mut velocity = prev.velocity;
            let wind_vel = wind.velocity();
            velocity += GRAVITY * delta_seconds;
            position += (velocity + wind_vel) * delta_seconds;
            let alpha = projectile.get_alpha();

            if position.y <= LEFT_PLAYER_POS_Y {
                position.y = LEFT_PLAYER_POS_Y;
            }

            if let Ok((mut sprite, mut visibility, mut transform)) = query.single_mut() {
                sprite.color = sprite.color.with_alpha(alpha);
                *visibility = Visibility::Visible;
                transform.translation.x = position.x;
                transform.translation.y = position.y;
                transform.translation.z = 0.8;

                if position.y > LEFT_PLAYER_POS_Y {
                    let angle = match *play_side {
                        PlaySide::LeftThrown => -time.delta_secs(),
                        PlaySide::RightThrown => time.delta_secs(),
                        _ => 0.0,
                    };

                    transform.rotate_z(angle);
                }
            }
        }
        _ => {
            if let Ok((_, mut visibility, _)) = query.single_mut() {
                *visibility = Visibility::Hidden;
            }
        }
    };
}

fn cleanup_projectile(mut query: Query<&mut Visibility, With<Projectile>>) {
    if let Ok(mut visibility) = query.single_mut() {
        *visibility = Visibility::Hidden;
    }
}

// POSTUPDATE SYSTEMS

fn update_camera_position(
    mut query: Query<&mut Transform, (With<Camera2d>, Without<Projectile>)>,
    projectile: Query<&Transform, (With<Projectile>, Without<Camera2d>)>,
    play_side: Res<PlaySide>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    transform.translation.x = match *play_side {
        PlaySide::Left { .. } => LEFT_CAM_POS_X.lerp(transform.translation.x, 0.8),
        PlaySide::Right { .. } => RIGHT_CAM_POS_X.lerp(transform.translation.x, 0.8),
        _ => match projectile.single() {
            Ok(transform) => transform
                .translation
                .x
                .clamp(LEFT_CAM_POS_X, RIGHT_CAM_POS_X),
            Err(_) => transform.translation.x,
        },
    };
}

#[allow(clippy::too_many_arguments)]
fn check_collisions(
    mut commands: Commands,
    play_side: Res<PlaySide>,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    voices: Query<(Entity, &VoiceSound)>,
    mut spines: Query<(
        &mut Spine,
        &Character,
        &VoiceChannel,
        &mut CharacterAnimState,
    )>,
    left_collider: Query<(&Collider2d, &GlobalTransform, &LeftPlayerHead)>,
    right_collider: Query<(&Collider2d, &GlobalTransform, &RightPlayerHead)>,
    mut projectile: Query<(&Collider2d, &GlobalTransform, &mut Projectile)>,
) {
    match *play_side {
        PlaySide::LeftThrown => {
            if let Ok((projectile_collider, projectile_transform, mut projectile)) =
                projectile.single_mut()
                && !projectile.hit
                && let Ok((hero_collider, hero_transform, parent)) = right_collider.single()
                && Collider2d::intersects(
                    (hero_collider, hero_transform),
                    (projectile_collider, projectile_transform),
                )
                && let Ok((mut spine, character, channel, mut anim_state)) =
                    spines.get_mut(parent.0)
            {
                projectile.hit = true;
                *anim_state = CharacterAnimState::InGameHit1;
                play_character_animation(&mut spine, *character, *anim_state);

                cleanup_voices(channel, &mut commands, &voices);
                let hero: Hero = (*character).into();
                let path = HERO_VOICE_SETS[hero as usize]
                    .hit()
                    .choose(&mut rand::rng())
                    .copied()
                    .unwrap();
                let source = asset_server.load(path);
                play_voice_sound(&mut commands, &system_volume, source, *channel);

                let source = asset_server.load(SFX_PATH_EMOTICON_HIT);
                play_effect_sound(&mut commands, &system_volume, source);
            }
        }
        PlaySide::RightThrown => {
            if let Ok((projectile_collider, projectile_transform, mut projectile)) =
                projectile.single_mut()
                && !projectile.hit
                && let Ok((hero_collider, hero_transform, parent)) = left_collider.single()
                && Collider2d::intersects(
                    (hero_collider, hero_transform),
                    (projectile_collider, projectile_transform),
                )
                && let Ok((mut spine, character, channel, mut anim_state)) =
                    spines.get_mut(parent.0)
            {
                projectile.hit = true;
                *anim_state = CharacterAnimState::InGameHit1;
                play_character_animation(&mut spine, *character, *anim_state);

                cleanup_voices(channel, &mut commands, &voices);
                let hero: Hero = (*character).into();
                let path = HERO_VOICE_SETS[hero as usize]
                    .hit()
                    .choose(&mut rand::rng())
                    .copied()
                    .unwrap();
                let source = asset_server.load(path);
                play_voice_sound(&mut commands, &system_volume, source, *channel);

                let source = asset_server.load(SFX_PATH_EMOTICON_HIT);
                play_effect_sound(&mut commands, &system_volume, source);
            }
        }
        _ => { /* empty */ }
    }
}
