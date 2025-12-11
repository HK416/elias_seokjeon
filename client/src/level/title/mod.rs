mod init;
mod load;
mod message;

// Import necessary Bevy modules.
use bevy::{
    audio::PlaybackMode,
    input::{ButtonState, mouse::MouseButtonInput},
    prelude::*,
};
use bevy_spine::{SkeletonController, Spine, SpineReadyEvent};

use crate::assets::sound::SystemVolume;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(init::InnerPlugin)
            .add_plugins(load::InnerPlugin)
            .add_plugins(message::InnerPlugin)
            .add_systems(
                OnEnter(LevelStates::InTitle),
                (
                    debug_label,
                    show_title_entities,
                    setup_camera,
                    play_background_sound,
                ),
            )
            .add_systems(OnExit(LevelStates::InTitle), hide_title_entities)
            .add_systems(
                PreUpdate,
                (handle_button_interaction, handle_mouse_input)
                    .run_if(in_state(LevelStates::InTitle)),
            )
            .add_systems(
                Update,
                (
                    update_grabbed_timer,
                    added_grabbed_component,
                    removed_grabbed_component,
                    update_spine_bone_position,
                )
                    .run_if(in_state(LevelStates::InTitle)),
            )
            .add_systems(
                PostUpdate,
                update_collider_transform
                    .after(TransformSystems::Propagate)
                    .run_if(in_state(LevelStates::InTitle)),
            );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            Update,
            packet_receive_loop.run_if(in_state(LevelStates::InTitle)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InTitle");
}

#[allow(clippy::type_complexity)]
fn show_title_entities(
    mut query: Query<&mut Visibility, (With<TitleLevelRoot>, With<TitleLevelEntity>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn setup_camera(mut commands: Commands, camera_query: Query<(), With<Camera2d>>) {
    if camera_query.is_empty() {
        commands.spawn((
            Camera2d,
            Transform::from_xyz(0.0, 540.0, 0.0),
            Projection::Orthographic(OrthographicProjection {
                area: Rect::from_center_half_size(Vec2::ZERO, Vec2::ONE),
                scaling_mode: bevy::camera::ScalingMode::Fixed {
                    width: 1920.0,
                    height: 1080.0,
                },
                ..OrthographicProjection::default_2d()
            }),
        ));
    }
}

fn play_background_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    query: Query<(), With<BackgroundSound>>,
) {
    if query.is_empty() {
        commands.spawn((
            AudioPlayer::new(asset_server.load(BGM_PATH_BACKGROUND)),
            PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: system_volume.get_background(),
                ..Default::default()
            },
            BackgroundSound,
        ));
    }
}

// --- CLEANUP SYSTEMS ---

#[allow(clippy::type_complexity)]
fn hide_title_entities(
    mut query: Query<
        &mut Visibility,
        (
            With<TitleLevelRoot>,
            With<TitleLevelEntity>,
            Without<TitleBackground>,
        ),
    >,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

// --- PREUPDATE SYSTEMS ---

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn handle_button_interaction(
    mut commands: Commands,
    #[cfg(target_arch = "wasm32")] network: Res<Network>,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    mut next_state: ResMut<NextState<LevelStates>>,
    children_query: Query<&Children>,
    mut text_color_query: Query<(&mut TextColor, &OriginColor<TextColor>)>,
    mut button_color_query: Query<(&mut BackgroundColor, &OriginColor<BackgroundColor>)>,
    mut interaction_query: Query<
        (Entity, &TitleButton, &Interaction),
        (Changed<Interaction>, With<TitleLevelEntity>, With<Button>),
    >,
) {
    for (entity, button, interaction) in interaction_query.iter_mut() {
        update_button_visual(
            entity,
            interaction,
            &children_query,
            &mut text_color_query,
            &mut button_color_query,
        );

        match (button, interaction) {
            (TitleButton::GameStart, Interaction::Pressed) => {
                #[cfg(target_arch = "wasm32")]
                send_enter_game_message(&network);
                let source = asset_server.load(SFX_PATH_COMMON_BUTTON_DOWN);
                play_effect_sound(&mut commands, &system_volume, source);
                next_state.set(LevelStates::SwitchToInMatching);
            }
            (TitleButton::Option, Interaction::Pressed) => {
                let source = asset_server.load(SFX_PATH_COMMON_BUTTON_DOWN);
                play_effect_sound(&mut commands, &system_volume, source);
                next_state.set(LevelStates::SwitchToInOption);
            }
            (TitleButton::HowToPlay, Interaction::Pressed) => {
                let source = asset_server.load(SFX_PATH_COMMON_BUTTON_DOWN);
                play_effect_sound(&mut commands, &system_volume, source);
            }
            (TitleButton::GameStart, Interaction::Hovered)
            | (TitleButton::Option, Interaction::Hovered)
            | (TitleButton::HowToPlay, Interaction::Hovered) => {
                let source = asset_server.load(SFX_PATH_COMMON_BUTTON_TOUCH);
                play_effect_sound(&mut commands, &system_volume, source);
            }
            _ => { /* empty */ }
        }
    }
}

fn handle_mouse_input(
    mut commands: Commands,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut button_inputs: MessageReader<MouseButtonInput>,
    collider_query: Query<(Entity, &Collider2d, &GlobalTransform)>,
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
                            break;
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

// --- POSTUPDATE SYSTEMS ---

fn update_collider_transform(
    transform_query: Query<&GlobalTransform>,
    mut query: Query<(&mut Transform, &TargetSpineBone), With<TitleLevelEntity>>,
) {
    for (mut transform, target_spine_bone) in query.iter_mut() {
        let bone_transform = transform_query.get(target_spine_bone.entity).unwrap();
        transform.translation = bone_transform.translation();
        transform.rotation = bone_transform.rotation();
        transform.scale = bone_transform.scale();
    }
}

// --- UTILITIES ---

#[cfg(target_arch = "wasm32")]
fn send_enter_game_message(network: &Network) {
    let packet = Packet::EnterGame;
    network.send(&packet).unwrap();
}
