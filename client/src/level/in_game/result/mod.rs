mod init;
mod switch;

// Import necessary Bevy modules.
use bevy::{
    input::{ButtonState, mouse::MouseButtonInput, touch::TouchPhase},
    prelude::*,
};

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(init::InnerPlugin)
            .add_plugins(switch::InnerPlugin)
            .add_systems(OnEnter(LevelStates::InGameResult), debug_label)
            .add_systems(
                OnExit(LevelStates::InGameResult),
                (
                    setup_loading_screen,
                    cleanup_in_game_assets,
                    cleanup_in_game_entities,
                    cleanup_background_sounds,
                ),
            )
            .add_systems(
                PreUpdate,
                (
                    handle_keyboard_inputs,
                    handle_mouse_inputs,
                    handle_touch_inputs,
                )
                    .run_if(in_state(LevelStates::InGameResult)),
            )
            .add_systems(
                Update,
                (
                    update_grabbed_timer,
                    added_grabbed_component,
                    removed_grabbed_component,
                    update_spine_bone_position,
                    update_spine_bone_position_for_mobile,
                )
                    .run_if(in_state(LevelStates::InGameResult)),
            );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            Update,
            packet_receive_loop.run_if(in_state(LevelStates::InGameResult)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InGameResult");
}

// --- CLEANUP SYSTEMS ---

fn cleanup_in_game_assets(mut commands: Commands) {
    commands.remove_resource::<InGameAssets>();
}

fn cleanup_in_game_entities(mut commands: Commands, query: Query<Entity, With<InGameLevelRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn cleanup_background_sounds(mut commands: Commands, query: Query<Entity, With<BackgroundSound>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// --- PREUPDATE SYSTEMS ---

fn handle_keyboard_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(LevelStates::LoadTitle);
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_mouse_inputs(
    mut commands: Commands,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut button_inputs: MessageReader<MouseButtonInput>,
    collider_query: Query<(Entity, &Collider2d, &GlobalTransform)>,
    grabbed_query: Query<Entity, With<Grabbed>>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    'input: for event in button_inputs.read() {
        match (event.button, event.state) {
            (MouseButton::Left, ButtonState::Pressed) => {
                if grabbed_query.is_empty()
                    && let Some(cursor_viewport_position) = window.cursor_position()
                    && let Ok(point) =
                        camera.viewport_to_world_2d(camera_transform, cursor_viewport_position)
                {
                    for (entity, collider, transform) in collider_query.iter() {
                        if Collider2d::contains((collider, transform), point) {
                            commands.entity(entity).insert(Grabbed::default());
                            continue 'input;
                        }
                    }
                    next_state.set(LevelStates::LoadTitle);
                }
            }
            (MouseButton::Left, ButtonState::Released) => {
                for entity in grabbed_query.iter() {
                    commands.entity(entity).remove::<Grabbed>();
                }
            }
            _ => { /* empty */ }
        }
    }
}

fn handle_touch_inputs(
    mut commands: Commands,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut touch_inputs: MessageReader<TouchInput>,
    collider_query: Query<(Entity, &Collider2d, &GlobalTransform)>,
    grabbed_query: Query<Entity, With<Grabbed>>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    'input: for event in touch_inputs.read() {
        match event.phase {
            TouchPhase::Started => {
                if grabbed_query.is_empty()
                    && let Ok(point) = camera.viewport_to_world_2d(camera_transform, event.position)
                {
                    for (entity, collider, transform) in collider_query.iter() {
                        if Collider2d::contains((collider, transform), point) {
                            commands.entity(entity).insert(Grabbed::new(event.id));
                            continue 'input;
                        }
                    }
                    next_state.set(LevelStates::LoadTitle);
                }
            }
            TouchPhase::Ended => {
                for entity in grabbed_query.iter() {
                    commands.entity(entity).remove::<Grabbed>();
                }
            }
            _ => { /* empty */ }
        }
    }
}
