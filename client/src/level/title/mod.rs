mod init;
mod load;

// Import necessary Bevy modules.
use bevy::prelude::*;
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
                    play_animation,
                ),
            )
            .add_systems(OnExit(LevelStates::InTitle), hide_interface)
            .add_systems(
                PreUpdate,
                handle_button_interaction.run_if(in_state(LevelStates::InTitle)),
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

#[allow(unreachable_patterns)]
fn play_animation(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut spine_query: Query<(&mut Spine, &Character)>,
) {
    for event in spine_ready_event.read() {
        let Ok((mut spine, character)) = spine_query.get_mut(event.entity) else {
            continue;
        };

        let Spine(SkeletonController {
            skeleton,
            animation_state,
            ..
        }) = spine.as_mut();
        match character {
            Character::Butter => {
                skeleton.set_skin_by_name("Normal").unwrap();
                animation_state
                    .set_animation_by_name(0, "Idle_1", true)
                    .unwrap();
            }
            Character::Kommy => {
                skeleton.set_skin_by_name("Normal").unwrap();
                animation_state
                    .set_animation_by_name(0, "Taunt_1", true)
                    .unwrap();
            }
            _ => { /* empty */ }
        }
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
