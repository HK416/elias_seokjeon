mod init;

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
            .add_systems(
                OnEnter(LevelStates::InMatchingCancel),
                (debug_label, show_interface, setup_ui_animation),
            )
            .add_systems(
                OnExit(LevelStates::InMatchingCancel),
                (
                    hide_interface,
                    cleanup_backout_anim::<MatchingCancelLevelEntity>,
                    cleanup_cancel_flag,
                ),
            )
            .add_systems(
                PreUpdate,
                (handle_keyboard_input, handle_button_interaction)
                    .run_if(resource_exists::<Interactable>)
                    .run_if(not(resource_exists::<CancelFlag>))
                    .run_if(in_state(LevelStates::InMatchingCancel)),
            )
            .add_systems(
                Update,
                (
                    update_backout_anim::<MatchingCancelLevelEntity>,
                    check_backout_anim_finished::<MatchingCancelLevelEntity>
                        .run_if(not(resource_exists::<Interactable>)),
                    handle_spine_animation_completed,
                    update_wave_animation,
                )
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

fn show_interface(mut query: Query<&mut Visibility, (With<UI>, With<MatchingCancelLevelEntity>)>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn setup_ui_animation(
    mut commands: Commands,
    query: Query<(Entity, &UI), With<MatchingCancelLevelEntity>>,
) {
    for (entity, &ui) in query.iter() {
        match ui {
            UI::InMatchingCancelModal => {
                commands.entity(entity).insert(UiBackOutScale::new(
                    UI_POPUP_DURATION,
                    Vec2::ZERO,
                    Vec2::ONE,
                ));
            }
            _ => { /* empty */ }
        }
    }
}

// --- CLEANUP SYSTEMS ---

fn hide_interface(mut query: Query<&mut Visibility, (With<UI>, With<MatchingCancelLevelEntity>)>) {
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
        next_state.set(LevelStates::InMatching);
    }
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn handle_button_interaction(
    #[cfg(target_arch = "wasm32")] network: Res<Network>,
    #[cfg(target_arch = "wasm32")] player_info: Res<PlayerInfo>,
    mut commands: Commands,
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
                send_cancel_game_message(&network);
                commands.insert_resource(CancelFlag);
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
            Ok(packet) => match packet {
                Packet::CancelSuccess => {
                    next_state.set(LevelStates::InTitle);
                }
                Packet::MatchingSuccess { other, hero } => {
                    commands.insert_resource(OtherInfo { name: other, hero });
                    next_state.set(LevelStates::LoadGame);
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
