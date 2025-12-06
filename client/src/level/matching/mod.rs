mod cancel;
mod init;
mod switch;

// Import necessary Bevy modules.
use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::assets::locale::Locale;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(cancel::InnerPlugin)
            .add_plugins(init::InnerPlugin)
            .add_plugins(switch::InnerPlugin)
            .add_systems(OnEnter(LevelStates::InMatching), debug_label)
            .add_systems(OnExit(LevelStates::InMatching), hide_interface)
            .add_systems(
                PreUpdate,
                (handle_keyboard_input, handle_button_interaction)
                    .run_if(in_state(LevelStates::InMatching)),
            )
            .add_systems(
                Update,
                (handle_spine_animation_completed, update_wave_animation)
                    .run_if(in_state(LevelStates::InMatching)),
            );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            Update,
            handle_received_packets.run_if(in_state(LevelStates::InMatching)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InMatching");
}

// --- CLEANUP SYSTEMS ---

#[allow(clippy::type_complexity)]
fn hide_interface(
    mut query: Query<&mut Visibility, (With<UI>, With<TitleLevelRoot>, With<MatchingLevelEntity>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

// --- PREUPDATE SYSTEMS ---

fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(LevelStates::SwitchToInMatchingCancel);
    }
}

#[allow(clippy::type_complexity)]
fn handle_button_interaction(
    mut next_state: ResMut<NextState<LevelStates>>,
    children_query: Query<&Children>,
    mut text_color_query: Query<(&mut TextColor, &OriginColor<TextColor>)>,
    mut button_color_query: Query<(&mut BackgroundColor, &OriginColor<BackgroundColor>)>,
    mut interaction_query: Query<
        (Entity, &UI, &Interaction),
        (With<MatchingLevelEntity>, Changed<Interaction>),
    >,
) {
    for (entity, &ui, interaction) in interaction_query.iter_mut() {
        update_button_visual(
            entity,
            interaction,
            &children_query,
            &mut text_color_query,
            &mut button_color_query,
        );

        match (ui, interaction) {
            (UI::NegativeButton, Interaction::Pressed) => {
                next_state.set(LevelStates::SwitchToInMatchingCancel);
            }
            _ => { /* empty */ }
        }
    }
}

// --- UPDATE SYSTEMS ---

#[cfg(target_arch = "wasm32")]
fn handle_received_packets(
    mut commands: Commands,
    mut next_state: ResMut<NextState<LevelStates>>,
    mut query: Query<&mut Text, With<MatchingStatusMessage>>,
    locale: Res<Locale>,
    network: Res<Network>,
    player_info: Res<PlayerInfo>,
) {
    for result in network.try_iter() {
        match result {
            Ok(packet) => match packet {
                Packet::MatchingStatus { millis } => {
                    if let Ok(mut text) = query.single_mut() {
                        *text = Text::new(match *locale {
                            Locale::En => {
                                format!("Remaining time: {}", millis / 1000)
                            }
                            Locale::Ja => {
                                format!("残り時間: {}", millis / 1000)
                            }
                            Locale::Ko => {
                                format!("남은 시간: {}", millis / 1000)
                            }
                        });
                    }
                }
                Packet::MatchingSuccess { left, right } => {
                    let (other, left_side) = if left.uuid == player_info.uuid {
                        (right, false)
                    } else {
                        (left, true)
                    };

                    commands.insert_resource(OtherInfo {
                        left_side,
                        name: other.name.clone(),
                        hero: other.hero,
                        win: other.win,
                        lose: other.lose,
                    });
                    next_state.set(LevelStates::SwitchToLoadGame);
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
