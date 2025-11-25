mod enter;
mod init;
mod prepare;
mod switch;

// Import necessary Bevy modules.
use bevy::prelude::*;
use protocol::{LEFT_CAM_POS_X, RIGHT_CAM_POS_X};

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
                    update_hud_timer,
                    update_camera_position,
                    update_wind_indicator,
                )
                    .run_if(in_state(LevelStates::InGame)),
            );

        #[cfg(target_arch = "wasm32")]
        {
            app.add_systems(
                PreUpdate,
                handle_received_packets.run_if(in_state(LevelStates::InGame)),
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
    commands.insert_resource(PlaySide::default());
    commands.insert_resource(Wind::default());
}

// --- CLEANUP SYSTEMS ---

fn cleanup_resource(mut commands: Commands) {
    commands.remove_resource::<InGameTimer>();
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
fn handle_received_packets(
    mut commands: Commands,
    mut wind: ResMut<Wind>,
    mut side: ResMut<PlaySide>,
    mut timer: ResMut<InGameTimer>,
    mut next_state: ResMut<NextState<LevelStates>>,
    network: Res<Network>,
) {
    for result in network.try_iter() {
        match result {
            Ok(packet) => match packet {
                Packet::InGameLeftTurn {
                    total_remaining_millis,
                    wind_angle,
                    wind_power,
                    ..
                } => {
                    timer.miliis = total_remaining_millis;
                    *wind = Wind::new(wind_angle, wind_power);
                    *side = PlaySide::Left;
                }
                Packet::InGameRightTurn {
                    total_remaining_millis,
                    wind_angle,
                    wind_power,
                    ..
                } => {
                    timer.miliis = total_remaining_millis;
                    *wind = Wind::new(wind_angle, wind_power);
                    *side = PlaySide::Right;
                }
                Packet::InGameProjectileThrown {
                    total_remaining_millis,
                    wind_angle,
                    wind_power,
                    ..
                } => {
                    timer.miliis = total_remaining_millis;
                    *wind = Wind::new(wind_angle, wind_power);
                    *side = PlaySide::Thrown;
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

// --- UPDATE SYSTEMS ---

fn update_hud_timer(timer: Res<InGameTimer>, mut query: Query<&mut Text, With<HUDInGameTimer>>) {
    for mut text in query.iter_mut() {
        let seconds = (timer.miliis as f32 / 1000.0).ceil() as u32;
        *text = Text::new(format!("{:0>3}", seconds));
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
        PlaySide::Left => LEFT_CAM_POS_X.lerp(transform.translation.x, 0.8),
        PlaySide::Right => RIGHT_CAM_POS_X.lerp(transform.translation.x, 0.8),
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
