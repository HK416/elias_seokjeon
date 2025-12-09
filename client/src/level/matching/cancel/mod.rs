mod init;
mod switch;

// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- Resource ---

#[derive(Resource)]
struct CancelFlag;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(init::InnerPlugin)
            .add_plugins(switch::InnerPlugin)
            .add_systems(OnEnter(LevelStates::InMatchingCancel), debug_label)
            .add_systems(
                OnExit(LevelStates::InMatchingCancel),
                (hide_matching_cancel_entities, cleanup_cancel_flag),
            )
            .add_systems(
                PreUpdate,
                (handle_keyboard_input, handle_button_interaction)
                    .run_if(not(resource_exists::<CancelFlag>))
                    .run_if(in_state(LevelStates::InMatchingCancel)),
            );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            Update,
            handle_received_packets.run_if(in_state(LevelStates::InMatchingCancel)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InMatchingCancel");
}

// --- CLEANUP SYSTEMS ---

#[allow(clippy::type_complexity)]
fn hide_matching_cancel_entities(
    mut query: Query<&mut Visibility, (With<MatchingCancelLevelEntity>, With<TitleLevelRoot>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn cleanup_cancel_flag(mut commands: Commands) {
    commands.remove_resource::<CancelFlag>();
}

// --- PREUPDATE SYSTEMS ---

fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(LevelStates::SwitchToInMatching);
    }
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn handle_button_interaction(
    mut commands: Commands,
    #[cfg(target_arch = "wasm32")] network: Res<Network>,
    mut next_state: ResMut<NextState<LevelStates>>,
    children_query: Query<&Children>,
    mut text_color_query: Query<(&mut TextColor, &OriginColor<TextColor>)>,
    mut button_color_query: Query<(&mut BackgroundColor, &OriginColor<BackgroundColor>)>,
    mut interaction_query: Query<
        (Entity, &PNButton, &Interaction),
        (With<MatchingCancelLevelEntity>, Changed<Interaction>),
    >,
) {
    for (entity, &button, interaction) in interaction_query.iter_mut() {
        update_button_visual(
            entity,
            interaction,
            &children_query,
            &mut text_color_query,
            &mut button_color_query,
        );

        match (button, interaction) {
            (PNButton::Positive, Interaction::Pressed) => {
                #[cfg(target_arch = "wasm32")]
                send_cancel_game_message(&network);
                commands.insert_resource(CancelFlag);
            }
            (PNButton::Negative, Interaction::Pressed) => {
                next_state.set(LevelStates::SwitchToInMatching);
            }
            _ => { /* empty */ }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn handle_received_packets(
    mut commands: Commands,
    mut next_state: ResMut<NextState<LevelStates>>,
    network: Res<Network>,
    player_info: Res<PlayerInfo>,
) {
    for result in network.try_iter() {
        match result {
            Ok(packet) => match packet {
                Packet::CancelSuccess => {
                    next_state.set(LevelStates::InTitle);
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

// --- UTILITIES ---

#[cfg(target_arch = "wasm32")]
fn send_cancel_game_message(network: &Network) {
    let packet = Packet::TryCancelGame;
    network.send(&packet).unwrap();
}
