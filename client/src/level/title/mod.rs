mod init;
mod load;

// Import necessary Bevy modules.
use bevy::{
    input::{ButtonState, mouse::MouseButtonInput},
    prelude::*,
};
use bevy_spine::{SkeletonController, Spine, SpineReadyEvent};

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(init::InnerPlugin)
            .add_plugins(load::InnerPlugin)
            .add_systems(
                OnEnter(LevelStates::InTitle),
                (
                    debug_label,
                    show_entities,
                    show_interface,
                    setup_title_screen,
                ),
            )
            .add_systems(OnExit(LevelStates::InTitle), hide_interface)
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
                (update_collider_transform.after(TransformSystems::Propagate),)
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

fn show_entities(mut query: Query<&mut Visibility, (With<TitleLevelRoot>, Without<UI>)>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

#[allow(unreachable_patterns)]
fn show_interface(mut query: Query<&mut Visibility, (With<UI>, With<TitleLevelEntity>)>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn setup_title_screen(mut commands: Commands, camera_query: Query<(), With<Camera2d>>) {
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
            TitleLevelRoot,
        ));
    }
}

// --- CLEANUP SYSTEMS ---

#[allow(unreachable_patterns)]
fn hide_interface(mut query: Query<&mut Visibility, (With<UI>, With<TitleLevelEntity>)>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

// --- PREUPDATE SYSTEMS ---

#[allow(unreachable_patterns)]
#[allow(clippy::type_complexity)]
fn handle_button_interaction(
    #[cfg(target_arch = "wasm32")] network: Res<Network>,
    mut next_state: ResMut<NextState<LevelStates>>,
    children_query: Query<&Children>,
    mut text_color_query: Query<(&mut TextColor, &OriginColor<TextColor>)>,
    mut button_color_query: Query<(&mut BackgroundColor, &OriginColor<BackgroundColor>)>,
    mut interaction_query: Query<
        (Entity, &UI, &Interaction),
        (Changed<Interaction>, With<TitleLevelEntity>, With<Button>),
    >,
) {
    for (entity, ui, interaction) in interaction_query.iter_mut() {
        update_button_visual(
            entity,
            interaction,
            &children_query,
            &mut text_color_query,
            &mut button_color_query,
        );

        match (ui, interaction) {
            (UI::InTitleGameStartButton, Interaction::Pressed) => {
                #[cfg(target_arch = "wasm32")]
                send_enter_game_message(&network);
                next_state.set(LevelStates::SwitchToInMatching);
            }
            (UI::InTitleOptionButton, Interaction::Pressed) => {
                next_state.set(LevelStates::SwitchToInOption);
            }
            (UI::InTitleHowToPlayButton, Interaction::Pressed) => {}
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

// --- UPDATE SYSTEMS ---

fn update_grabbed_timer(
    mut spine_query: Query<(&mut Spine, &Character, &mut CharacterAnimState)>,
    mut grabbed_query: Query<(&TargetSpine, &ColliderType, &mut Grabbed)>,
    time: Res<Time>,
) {
    for (target_spine, ty, mut grabbed) in grabbed_query.iter_mut() {
        grabbed.elapsed += time.delta_secs();
        if matches!(ty, ColliderType::Head)
            && grabbed.elapsed > GRABBED_TIME_THRESHOLD
            && let Ok((mut spine, character, mut anim_state)) =
                spine_query.get_mut(target_spine.entity)
            && !matches!(*anim_state, CharacterAnimState::PatIdle)
        {
            *anim_state = CharacterAnimState::PatIdle;
            play_character_animation(&mut spine, *character, *anim_state);
        }
    }
}

fn added_grabbed_component(
    bone_query: Query<&GlobalTransform>,
    mut spine_query: Query<(&mut Spine, &Character, &mut CharacterAnimState)>,
    mut grabbed_query: Query<
        (
            &TargetSpine,
            &TargetSpineBone,
            &ColliderType,
            &mut SpineBoneOriginPosition,
        ),
        Added<Grabbed>,
    >,
) {
    for (target_spine, target_spine_bone, ty, mut origin_position) in grabbed_query.iter_mut() {
        if let Ok((mut spine, character, mut anim_state)) = spine_query.get_mut(target_spine.entity)
            && let Ok(transform) = bone_query.get(target_spine_bone.entity)
        {
            match ty {
                ColliderType::Ball => {
                    origin_position.world = transform.translation().xy();
                    *anim_state = CharacterAnimState::TouchIdle;
                    play_character_animation(&mut spine, *character, *anim_state);
                }
                _ => { /* empty */ }
            };
        }
    }
}

#[allow(clippy::type_complexity)]
fn removed_grabbed_component(
    mut commands: Commands,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut entities: RemovedComponents<Grabbed>,
    mut spine_query: Query<(&mut Spine, &Character, &mut CharacterAnimState)>,
    grabbed_query: Query<(Entity, &TargetSpine, &TargetSpineBone, &ColliderType)>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    for entity in entities.read() {
        if let Ok((entity, target_spine, target_spine_bone, ty)) = grabbed_query.get(entity)
            && let Ok((mut spine, character, mut anim_state)) =
                spine_query.get_mut(target_spine.entity)
        {
            match ty {
                ColliderType::Ball => {
                    *anim_state = CharacterAnimState::TouchEnd;
                    play_character_animation(&mut spine, *character, *anim_state);

                    if let Some(cursor_viewport_position) = window.cursor_position()
                        && let Ok(point) =
                            camera.viewport_to_world_2d(camera_transform, cursor_viewport_position)
                        && let Some(bone) = spine.skeleton.bone_at_index(target_spine_bone.index)
                    {
                        let w_bone_position: Vec2 = bone.world_position().into();
                        let distance = point - w_bone_position;
                        let length = distance.length();
                        if length > f32::EPSILON {
                            commands.entity(entity).insert(BallWaveAnimation {
                                elapsed: 0.0,
                                direction: distance / length,
                                power: length.min(BALL_MOVE_RANGE * 0.5),
                            });
                        }
                    }
                }
                ColliderType::Head => {
                    if matches!(*anim_state, CharacterAnimState::PatIdle) {
                        *anim_state = CharacterAnimState::PatEnd;
                        play_character_animation(&mut spine, *character, *anim_state);
                    } else {
                        *anim_state = CharacterAnimState::SmashEnd1;
                        play_character_animation(&mut spine, *character, *anim_state);
                    }
                }
            }
        }
    }
}

fn update_spine_bone_position(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut spine_query: Query<(&mut Spine, &GlobalTransform)>,
    mut grabbed_query: Query<
        (
            &TargetSpine,
            &TargetSpineBone,
            &SpineBoneOriginPosition,
            &ColliderType,
        ),
        With<Grabbed>,
    >,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    for (target_spine, target_spine_bone, origin_position, ty) in grabbed_query.iter_mut() {
        if matches!(ty, ColliderType::Ball)
            && let Ok((mut spine, transform)) = spine_query.get_mut(target_spine.entity)
            && let Some(mut bone) = spine.skeleton.bone_at_index_mut(target_spine_bone.index)
            && let Some(cursor_viewport_position) = window.cursor_position()
            && let Ok(point) =
                camera.viewport_to_world_2d(camera_transform, cursor_viewport_position)
        {
            let w_bone_position = origin_position.world;
            let distance = point - w_bone_position;
            let length = distance.length();
            let offset = vec2(1.0, -transform.scale().x);
            if length > f32::EPSILON {
                bone.set_position(
                    origin_position.local
                        + distance.yx() * offset / length * length.min(BALL_MOVE_RANGE),
                );
            } else {
                bone.set_position(w_bone_position);
            }
        }
    }
}

// --- POSTUPDATE SYSTEMS ---

fn update_collider_transform(
    transform_query: Query<&GlobalTransform>,
    mut query: Query<(&mut Transform, &TargetSpineBone)>,
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
