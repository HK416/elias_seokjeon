mod init;
mod switch;

// Import necessary Bevy modules.
use bevy::{
    input::{ButtonState, mouse::MouseButtonInput},
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
                ),
            )
            .add_systems(
                PreUpdate,
                (handle_keyboard_inputs, handle_mouse_inputs)
                    .run_if(in_state(LevelStates::InGameResult)),
            )
            .add_systems(
                Update,
                (
                    update_grabbed_timer,
                    added_grabbed_component,
                    removed_grabbed_component,
                    update_spine_bone_position,
                )
                    .run_if(in_state(LevelStates::InGameResult)),
            )
            .add_systems(
                PostUpdate,
                update_collider_transform
                    .after(TransformSystems::Propagate)
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
    collider_query: Query<(Entity, &Collider2d, &GlobalTransform), With<InGameResultLevelEntity>>,
    grabbed_query: Query<Entity, With<Grabbed>>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    for event in button_inputs.read() {
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
                            return;
                        }
                    }
                    next_state.set(LevelStates::LoadTitle);
                }
            }
            (MouseButton::Left, ButtonState::Released) => {
                if let Ok(entity) = grabbed_query.single() {
                    commands.entity(entity).remove::<Grabbed>();
                }
            }
            _ => { /* empty */ }
        }
    }
}

// --- POSTUPDATE SYSTEMS ---

fn update_collider_transform(
    transform_query: Query<&GlobalTransform>,
    mut query: Query<(&mut Transform, &TargetSpineBone), With<InGameResultLevelEntity>>,
) {
    for (mut transform, target_spine_bone) in query.iter_mut() {
        let bone_transform = transform_query.get(target_spine_bone.entity).unwrap();
        transform.translation = bone_transform.translation();
        transform.rotation = bone_transform.rotation();
        transform.scale = bone_transform.scale();
    }
}
