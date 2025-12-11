mod init;
mod switch;

// Import necessary Bevy modules.
use bevy::{
    input::{ButtonState, mouse::MouseButtonInput},
    prelude::*,
};

use crate::assets::sound::SystemVolume;

use super::*;

// --- CONSTANTS ---

pub const SCENE_DURATION: f32 = 3.0;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(init::InnerPlugin)
            .add_plugins(switch::InnerPlugin)
            .add_systems(
                OnEnter(LevelStates::InPrepareGame),
                (
                    debug_label,
                    hide_loading_interfaces,
                    setup_scene_timer,
                    play_sound,
                ),
            )
            .add_systems(OnExit(LevelStates::InPrepareGame), cleanup_scene_timer)
            .add_systems(
                PreUpdate,
                handle_mouse_inputs.run_if(in_state(LevelStates::InPrepareGame)),
            )
            .add_systems(
                Update,
                (
                    update_scene_timer,
                    update_pvp_vs_fire_effect,
                    update_grabbed_timer,
                    added_grabbed_component,
                    removed_grabbed_component,
                    update_spine_bone_position,
                )
                    .run_if(in_state(LevelStates::InPrepareGame)),
            )
            .add_systems(
                PostUpdate,
                update_collider_transform
                    .after(TransformSystems::Propagate)
                    .run_if(in_state(LevelStates::InPrepareGame)),
            );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            Update,
            packet_receive_loop.run_if(in_state(LevelStates::InPrepareGame)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InPrepareGame");
}

#[allow(clippy::type_complexity)]
fn hide_loading_interfaces(
    mut query: Query<
        &mut Visibility,
        (
            Without<BackgroundPattern>,
            With<EnterGameLevelEntity>,
            With<TitleLevelRoot>,
        ),
    >,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn setup_scene_timer(mut commands: Commands) {
    commands.insert_resource(SceneTimer::default());
}

fn play_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
) {
    let source = asset_server.load(SFX_PATH_BOXING_BELL);
    play_effect_sound(&mut commands, &system_volume, source);

    let source = asset_server.load(SFX_PATH_CHEER);
    play_effect_sound(&mut commands, &system_volume, source);

    let source = asset_server.load(SFX_PATH_BOXING_BELL);
    play_effect_sound(&mut commands, &system_volume, source);
}

// --- CLEANUP SYSTEMS ---

fn cleanup_scene_timer(mut commands: Commands) {
    commands.remove_resource::<SceneTimer>();
}

// --- PREUPDATE SYSTEMS ---

fn handle_mouse_inputs(
    mut commands: Commands,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut button_inputs: MessageReader<MouseButtonInput>,
    collider_query: Query<(Entity, &Collider2d, &GlobalTransform), With<InPrepareLevelEntity>>,
    grabbed_query: Query<Entity, With<Grabbed>>,
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

// --- UPDATE SYSTEMS ---

fn update_scene_timer(
    mut next_state: ResMut<NextState<LevelStates>>,
    mut scene_timer: ResMut<SceneTimer>,
    time: Res<Time>,
) {
    scene_timer.tick(time.delta_secs());
    if scene_timer.elapsed_sec() >= SCENE_DURATION {
        next_state.set(LevelStates::SwitchToInGame);
    }
}

fn update_pvp_vs_fire_effect(
    mut query: Query<(&mut ImageNode, &mut AnimationTimer), With<InPrepareLevelEntity>>,
    time: Res<Time>,
) {
    for (mut image_node, mut timer) in query.iter_mut() {
        timer.tick(time.delta_secs());
        if let Some(atlas) = image_node.texture_atlas.as_mut() {
            atlas.index = timer.frame_index();
        }
    }
}

// --- POSTUPDATE SYSTEMS ---

fn update_collider_transform(
    transform_query: Query<&GlobalTransform>,
    mut query: Query<(&mut Transform, &TargetSpineBone), With<InPrepareLevelEntity>>,
) {
    for (mut transform, target_spine_bone) in query.iter_mut() {
        let bone_transform = transform_query.get(target_spine_bone.entity).unwrap();
        transform.translation = bone_transform.translation();
        transform.rotation = bone_transform.rotation();
        transform.scale = bone_transform.scale();
    }
}
