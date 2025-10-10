mod init;
mod load;

// Import necessary Bevy modules.
use bevy::{
    input::{ButtonState, mouse::MouseButtonInput},
    prelude::*,
};
use bevy_spine::{SkeletonController, Spine, SpineEvent, SpineReadyEvent};

use super::*;

// --- CONSTANTS ---
const RANGE: f32 = 30.0;
const THRESHOLD: f32 = 0.25;

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
                (
                    handle_button_interaction,
                    handle_mouse_input,
                    handle_cursor_moved.run_if(resource_exists::<GrabbedCharacter>),
                )
                    .run_if(in_state(LevelStates::InTitle)),
            )
            .add_systems(
                Update,
                (
                    (
                        update_grabbed_character_timer,
                        update_grabbed_spine_animation,
                    )
                        .run_if(resource_exists::<GrabbedCharacter>),
                    handle_spine_animation_completed,
                )
                    .run_if(in_state(LevelStates::InTitle)),
            )
            .add_systems(
                PostUpdate,
                (
                    update_collider_transform.after(TransformSystems::Propagate),
                    update_grabbed_spine_position.run_if(resource_exists::<GrabbedCharacter>),
                )
                    .run_if(in_state(LevelStates::InTitle)),
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
fn show_interface(mut query: Query<(&mut Visibility, &UI)>) {
    for (mut visibility, ui) in query.iter_mut() {
        match ui {
            UI::InTitleGameStartButton | UI::InTitleOptionButton | UI::InTitleHowToPlayButton => {
                *visibility = Visibility::Visible;
            }
            _ => { /* empty */ }
        }
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
fn hide_interface(mut query: Query<(&mut Visibility, &UI)>) {
    for (mut visibility, ui) in query.iter_mut() {
        match ui {
            UI::InTitleGameStartButton | UI::InTitleOptionButton | UI::InTitleHowToPlayButton => {
                *visibility = Visibility::Hidden;
            }
            _ => { /* empty */ }
        }
    }
}

// --- PREUPDATE SYSTEMS ---

#[allow(unreachable_patterns)]
#[allow(clippy::type_complexity)]
fn handle_button_interaction(
    children_query: Query<&Children>,
    mut text_color_query: Query<(&mut TextColor, &OriginColor)>,
    mut button_color_query: Query<(&mut BackgroundColor, &OriginColor)>,
    mut interaction_query: Query<(Entity, &UI, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for (entity, ui, interaction) in interaction_query.iter_mut() {
        match ui {
            UI::InTitleGameStartButton => {
                update_button_visual(
                    entity,
                    interaction,
                    &children_query,
                    &mut text_color_query,
                    &mut button_color_query,
                );
            }
            UI::InTitleOptionButton => {
                update_button_visual(
                    entity,
                    interaction,
                    &children_query,
                    &mut text_color_query,
                    &mut button_color_query,
                );
            }
            UI::InTitleHowToPlayButton => {
                update_button_visual(
                    entity,
                    interaction,
                    &children_query,
                    &mut text_color_query,
                    &mut button_color_query,
                );
            }
            _ => { /* empty */ }
        }
    }
}

fn handle_cursor_moved(
    mut grabbed_character: ResMut<GrabbedCharacter>,
    mut cursor_moved_reader: MessageReader<CursorMoved>,
) {
    for event in cursor_moved_reader.read() {
        if let Some(delta) = event.delta {
            grabbed_character.cursor_delta -= delta;
        }
    }
}

#[allow(clippy::type_complexity)]
fn handle_mouse_input(
    mut commands: Commands,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    grabbed_character: Option<Res<GrabbedCharacter>>,
    mut input_events: MessageReader<MouseButtonInput>,
    mut spine_query: Query<(&mut Spine, &Character, &mut CharacterAnimState)>,
    collider_query: Query<
        (
            &TargetSpine,
            &TargetSpineBone,
            &Collider2d,
            &GlobalTransform,
            &ColliderType,
        ),
        With<TitleLevelEntity>,
    >,
) {
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    for event in input_events.read() {
        match (event.button, event.state) {
            (MouseButton::Left, ButtonState::Pressed) => {
                let Ok(window) = window_query.get(event.window) else {
                    return;
                };
                let Some(point) = window.cursor_position() else {
                    return;
                };

                let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, point) else {
                    return;
                };

                for (target_spine, target_spine_bone, collider, transform, &ty) in
                    collider_query.iter()
                {
                    if Collider2d::contains((collider, transform), world_pos) {
                        info!("Grabbed Character: {:?}", target_spine.entity);
                        let (mut spine, ..) = spine_query.get_mut(target_spine.entity).unwrap();
                        let bone = spine
                            .skeleton
                            .bone_at_index_mut(target_spine_bone.bone_index)
                            .unwrap();

                        commands.insert_resource(GrabbedCharacter {
                            target: target_spine.entity,
                            bone_index: target_spine_bone.bone_index,
                            bone_position: bone.position().into(),
                            cursor_delta: Vec2::ZERO,
                            duration: 0.0,
                            ty,
                        });

                        match ty {
                            ColliderType::Ball => {
                                if let Ok((mut spine, character, mut anim_state)) =
                                    spine_query.get_mut(target_spine.entity)
                                {
                                    *anim_state = CharacterAnimState::TouchIdle;
                                    play_character_animation(&mut spine, *character, *anim_state);
                                }
                            }
                            _ => { /* empty */ }
                        }

                        break;
                    }
                }
            }
            (MouseButton::Left, ButtonState::Released) => {
                if let Some(grabbed_character) = &grabbed_character
                    && let Ok((mut spine, character, mut anim_state)) =
                        spine_query.get_mut(grabbed_character.target)
                {
                    let mut bone = spine
                        .skeleton
                        .bone_at_index_mut(grabbed_character.bone_index)
                        .unwrap();
                    bone.set_position(grabbed_character.bone_position);
                    spine.skeleton.update_world_transform();

                    *anim_state = match grabbed_character.ty {
                        ColliderType::Ball => CharacterAnimState::TouchEnd,
                        ColliderType::Head => {
                            if grabbed_character.duration <= THRESHOLD {
                                CharacterAnimState::SmashEnd1
                            } else {
                                CharacterAnimState::PatEnd
                            }
                        }
                    };

                    play_character_animation(&mut spine, *character, *anim_state);
                }

                commands.remove_resource::<GrabbedCharacter>();
            }
            _ => { /* empty */ }
        }
    }
}

// --- UPDATE SYSTEMS ---

fn update_grabbed_character_timer(
    mut grabbed_character: ResMut<GrabbedCharacter>,
    time: Res<Time>,
) {
    grabbed_character.duration += time.delta_secs();
}

fn update_grabbed_spine_animation(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    grabbed_character: Res<GrabbedCharacter>,
    mut spine_query: Query<(&mut Spine, &Character, &mut CharacterAnimState)>,
) {
    if let Ok((mut spine, character, mut anim_state)) =
        spine_query.get_mut(grabbed_character.target)
    {
        match grabbed_character.ty {
            ColliderType::Ball => {
                let Ok(window) = window_query.single() else {
                    return;
                };
                let Ok((camera, camera_transform)) = camera_query.single() else {
                    return;
                };
                let Some(point) = window.cursor_position() else {
                    return;
                };

                let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, point) else {
                    return;
                };

                let mut bone = spine
                    .skeleton
                    .bone_at_index_mut(grabbed_character.bone_index)
                    .unwrap();
                bone.set_applied_position(world_pos);
            }
            ColliderType::Head => {
                if grabbed_character.duration > THRESHOLD
                    && !matches!(*anim_state, CharacterAnimState::PatIdle)
                {
                    *anim_state = CharacterAnimState::PatIdle;
                    play_character_animation(&mut spine, *character, *anim_state);
                }
            }
        }
    }
}

fn handle_spine_animation_completed(
    mut spine_events: MessageReader<SpineEvent>,
    mut spine_query: Query<
        (&mut Spine, &Character, &mut CharacterAnimState),
        With<TitleLevelEntity>,
    >,
) {
    for event in spine_events.read() {
        let SpineEvent::Complete { entity, .. } = event else {
            continue;
        };

        let Ok((mut spine, character, mut anim_state)) = spine_query.get_mut(*entity) else {
            continue;
        };

        *anim_state = match *anim_state {
            CharacterAnimState::PatEnd
            | CharacterAnimState::TouchEnd
            | CharacterAnimState::SmashEnd2 => CharacterAnimState::Idle,
            CharacterAnimState::SmashEnd1 => CharacterAnimState::SmashEnd2,
            _ => continue,
        };

        play_character_animation(&mut spine, *character, *anim_state);
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

fn update_grabbed_spine_position(
    grabbed_character: Res<GrabbedCharacter>,
    mut spine_query: Query<(&mut Spine, &GlobalTransform)>,
) {
    if let Ok((mut spine, transform)) = spine_query.get_mut(grabbed_character.target) {
        match grabbed_character.ty {
            ColliderType::Ball => {
                let mut bone = spine
                    .skeleton
                    .bone_at_index_mut(grabbed_character.bone_index)
                    .unwrap();

                let length = grabbed_character.cursor_delta.length();
                let mut delta = grabbed_character.cursor_delta;
                if length > RANGE {
                    delta = delta / length * RANGE;
                }
                bone.set_position(
                    grabbed_character.bone_position + delta.yx() * transform.scale().zx(),
                );
                spine.skeleton.update_world_transform();
            }
            _ => { /* empty */ }
        }
    }
}

// --- UTILITIES ---

fn play_character_animation(
    spine: &mut Spine,
    character: Character,
    anim_state: CharacterAnimState,
) {
    let (animation_name, looping) = match anim_state {
        CharacterAnimState::Idle => match character {
            Character::Butter => (BUTTER_TITLE_IDLE, true),
            Character::Kommy => (KOMMY_TITLE_TAUNT, true),
        },
        CharacterAnimState::PatIdle => (PAT_IDLE, true),
        CharacterAnimState::PatEnd => (PAT_END, false),
        CharacterAnimState::TouchIdle => (TOUCH_IDLE, true),
        CharacterAnimState::TouchEnd => (TOUCH_END, false),
        CharacterAnimState::SmashEnd1 => (SMASH_END_1, false),
        CharacterAnimState::SmashEnd2 => (SMASH_END_2, false),
    };

    spine
        .animation_state
        .set_animation_by_name(0, animation_name, looping)
        .unwrap();
}
