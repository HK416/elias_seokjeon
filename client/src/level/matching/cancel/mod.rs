mod init;

// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(init::InnerPlugin)
            .add_systems(
                OnEnter(LevelStates::InMatchingCancel),
                (debug_label, show_interface),
            )
            .add_systems(OnExit(LevelStates::InMatchingCancel), hide_interface)
            .add_systems(
                PreUpdate,
                (handle_button_interaction,).run_if(in_state(LevelStates::InMatchingCancel)),
            )
            .add_systems(
                Update,
                (handle_spine_animation_completed, update_wave_animation)
                    .run_if(in_state(LevelStates::InMatchingCancel)),
            );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InMatchingCancel");
}

fn show_interface(mut query: Query<&mut Visibility, (With<UI>, With<MatchingCancelLevelEntity>)>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

// --- CLEANUP SYSTEMS ---

fn hide_interface(mut query: Query<&mut Visibility, (With<UI>, With<MatchingCancelLevelEntity>)>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

// --- PREUPDATE SYSTEMS ---

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn handle_button_interaction(
    #[cfg(target_arch = "wasm32")] network: Res<Network>,
    #[cfg(target_arch = "wasm32")] player_info: Res<PlayerInfo>,
    mut next_state: ResMut<NextState<LevelStates>>,
    children_query: Query<&Children>,
    mut text_color_query: Query<(&mut TextColor, &OriginColor)>,
    mut button_color_query: Query<(&mut BackgroundColor, &OriginColor)>,
    mut interaction_query: Query<
        (Entity, &UI, &Interaction),
        (With<MatchingCancelLevelEntity>, Changed<Interaction>),
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
            (UI::InMatchingCancelYesButton, Interaction::Pressed) => {
                #[cfg(target_arch = "wasm32")]
                send_cancel_game_message(&network, player_info.uuid);
                next_state.set(LevelStates::InTitle);
            }
            (UI::InMatchingCancelNoButton, Interaction::Pressed) => {
                next_state.set(LevelStates::InMatching);
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
) {
    for result in network.try_iter() {
        match result {
            Ok(msg) => match msg.header {
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
fn send_cancel_game_message(network: &Network, uuid: Uuid) {
    use protocol::CancelGamePacket;
    let message: Packet = CancelGamePacket { uuid }.into();
    network.send(&message).unwrap();
}
