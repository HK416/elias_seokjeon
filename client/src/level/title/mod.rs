mod init;
mod load;

// Import necessary Bevy modules.
use bevy::{
    input::{ButtonState, mouse::MouseButtonInput},
    prelude::*,
};
use bevy_spine::{SkeletonController, Spine, SpineEvent, SpineReadyEvent};

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
                    update_grabbed_character_timer.run_if(resource_exists::<GrabbedCharacter>),
                    update_spine_animation,
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
                scaling_mode: bevy::render::camera::ScalingMode::Fixed {
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

fn handle_mouse_input(
    mut commands: Commands,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    grabbed_character: Option<Res<GrabbedCharacter>>,
    mut input_events: EventReader<MouseButtonInput>,
    mut spine_query: Query<&mut Spine, With<TitleLevelEntity>>,
    collider_query: Query<
        (&ChildOf, &Collider2d, &GlobalTransform, &ColliderType),
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

                for (child_of, collider, transform, &ty) in collider_query.iter() {
                    if Collider2d::contains((collider, transform), world_pos) {
                        info!("Grabbed Character: {:?}", child_of.parent());
                        commands.insert_resource(GrabbedCharacter {
                            target: child_of.parent(),
                            duration: 0.0,
                            ty,
                        });
                    }
                }
            }
            (MouseButton::Left, ButtonState::Released) => {
                if let Some(grabbed_character) = &grabbed_character
                    && matches!(grabbed_character.ty, ColliderType::Head)
                    && grabbed_character.duration <= 0.25
                    && let Ok(mut spine) = spine_query.get_mut(grabbed_character.target)
                {
                    let Spine(SkeletonController {
                        animation_state, ..
                    }) = spine.as_mut();
                    animation_state
                        .set_animation_by_name(0, SMASH_END_1, false)
                        .unwrap();
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

fn update_spine_animation(
    mut spine_events: EventReader<SpineEvent>,
    mut spine_query: Query<(&mut Spine, &Character), With<TitleLevelEntity>>,
) {
    for event in spine_events.read() {
        let SpineEvent::Complete { entity, animation } = event else {
            continue;
        };

        let Ok((mut spine, &character)) = spine_query.get_mut(*entity) else {
            continue;
        };

        let Spine(SkeletonController {
            animation_state, ..
        }) = spine.as_mut();

        match animation.as_str() {
            SMASH_END_1 => {
                animation_state
                    .set_animation_by_name(0, SMASH_END_2, false)
                    .unwrap();
            }
            SMASH_END_2 => match character {
                Character::Butter => {
                    animation_state
                        .set_animation_by_name(0, BUTTER_TITLE_IDLE, true)
                        .unwrap();
                }
                Character::Kommy => {
                    animation_state
                        .set_animation_by_name(0, KOMMY_TITLE_TAUNT, true)
                        .unwrap();
                }
            },
            _ => { /* empty */ }
        }
    }
}
