mod init;
mod switch;

// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(init::InnerPlugin)
            .add_plugins(switch::InnerPlugin)
            .add_systems(OnEnter(LevelStates::LeaderBoard), debug_label)
            .add_systems(
                OnExit(LevelStates::LeaderBoard),
                (cleanup_ranking_data, hide_leaderboard_interfaces),
            )
            .add_systems(
                PreUpdate,
                (handle_keyboard_inputs, handle_pn_button_pressed)
                    .run_if(in_state(LevelStates::LeaderBoard)),
            )
            .add_systems(
                Update,
                setup_leaderboard_interfaces
                    .run_if(resource_added::<RankingData>)
                    .run_if(in_state(LevelStates::LeaderBoard)),
            );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            Update,
            handle_received_packets.run_if(in_state(LevelStates::LeaderBoard)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: LeaderBoard");
}

// --- CLEANUP SYSTEMS --

fn cleanup_ranking_data(mut commands: Commands) {
    commands.remove_resource::<RankingData>();
}

fn hide_leaderboard_interfaces(
    mut query: Query<&mut Visibility, (With<LeaderBoardLevelEntity>, With<TitleLevelRoot>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

// --- PREUPDATE SYSTEMS ---

fn handle_keyboard_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(LevelStates::InTitle);
    }
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn handle_pn_button_pressed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    mut next_state: ResMut<NextState<LevelStates>>,
    children_query: Query<&Children>,
    mut text_color_query: Query<(&mut TextColor, &OriginColor<TextColor>)>,
    mut button_color_query: Query<(&mut BackgroundColor, &OriginColor<BackgroundColor>)>,
    mut interaction_query: Query<
        (Entity, &PNButton, &Interaction),
        (
            With<LeaderBoardLevelEntity>,
            Changed<Interaction>,
            With<Button>,
        ),
    >,
) {
    for (entity, &pn_button, interaction) in interaction_query.iter_mut() {
        update_button_visual(
            entity,
            interaction,
            &children_query,
            &mut text_color_query,
            &mut button_color_query,
        );

        match (pn_button, interaction) {
            (PNButton::Positive, Interaction::Pressed) => {
                let source = asset_server.load(SFX_PATH_COMMON_BUTTON_DOWN);
                play_effect_sound(&mut commands, &system_volume, source);
                next_state.set(LevelStates::InTitle);
            }
            (PNButton::Positive, Interaction::Hovered) => {
                let source = asset_server.load(SFX_PATH_COMMON_BUTTON_TOUCH);
                play_effect_sound(&mut commands, &system_volume, source);
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
    network: Res<Network>,
) {
    for result in network.receiver.try_iter() {
        match result {
            Ok(packet) => match packet {
                Packet::RankingResult { my_rank, top_list } => {
                    commands.insert_resource(RankingData::new(my_rank, top_list));
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

fn setup_leaderboard_interfaces(
    mut commands: Commands,
    ranking_data: Res<RankingData>,
    mut sets: ParamSet<(
        Query<(&mut Text, &RankEntry), With<RankItemUuid>>,
        Query<(&mut Text, &RankEntry), With<RankItemName>>,
        Query<(&mut Text, &RankEntry), With<RankItemWins>>,
        Query<(&mut Text, &RankEntry), With<RankItemLosses>>,
        Query<&mut Text, With<RankItemNum>>,
    )>,
) {
    for (mut text, entry) in sets.p0().iter_mut() {
        if let Some(item) = ranking_data.top_list.get(entry.0) {
            *text = Text::new(format!("{}", item.uuid));
        } else {
            *text = Text::new("-");
        }
    }

    for (mut text, entry) in sets.p1().iter_mut() {
        if let Some(item) = ranking_data.top_list.get(entry.0) {
            *text = Text::new(&item.name);
        } else {
            *text = Text::new("-");
        }
    }

    for (mut text, entry) in sets.p2().iter_mut() {
        if let Some(item) = ranking_data.top_list.get(entry.0) {
            *text = Text::new(format!("{}", item.wins));
        } else {
            *text = Text::new("-");
        }
    }

    for (mut text, entry) in sets.p3().iter_mut() {
        if let Some(item) = ranking_data.top_list.get(entry.0) {
            *text = Text::new(format!("{}", item.losses));
        } else {
            *text = Text::new("-");
        }
    }

    if let Ok(mut text) = sets.p4().single_mut() {
        if let Some(rank) = ranking_data.my_rank {
            *text = Text::new(format!("{}", rank));
        } else {
            *text = Text::new("-");
        }
    }

    commands.remove_resource::<RankingData>();
}
