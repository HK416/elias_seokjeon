// Import necessary Bevy modules.
use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::assets::config::ConfigData;

use super::*;

// --- CONSTANTS ---

const TIMEOUT: f32 = 10.0;
const MAX_RETRY_COUNT: u32 = 0;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::Connect),
            (debug_label, setup_timeout_retry),
        )
        .add_systems(OnExit(LevelStates::Connect), cleanup_timeout_retry)
        .add_systems(
            Update,
            check_connection.run_if(in_state(LevelStates::Connect)),
        );

        #[cfg(target_arch = "wasm32")]
        {
            app.add_systems(OnEnter(LevelStates::Connect), connect_game_server)
                .add_systems(
                    Update,
                    (packet_receive_loop, check_connection_progress)
                        .run_if(in_state(LevelStates::Connect)),
                );
        }
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: Connect");
}

#[cfg(target_arch = "wasm32")]
fn connect_game_server(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config_assets: Res<Assets<ConfigData>>,
) {
    let handle: Handle<ConfigData> = asset_server.load(CONFIG_PATH);
    let Some(config) = config_assets.get(handle.id()) else {
        commands.insert_resource(ErrorMessage::new(
            "net_not_found",
            "Failed to connect to the game server.",
        ));
        return;
    };

    match Network::new(&config.server_url) {
        Ok(network) => {
            commands.insert_resource(network);
        }
        Err(e) => {
            commands.insert_resource(ErrorMessage::from(e));
        }
    };
}

// --- UPDATE SYSTEMS ---

#[cfg(target_arch = "wasm32")]
fn packet_receive_loop(mut commands: Commands, network: Option<Res<Network>>) {
    if let Some(network) = network.as_ref() {
        for result in network.receiver.try_iter() {
            match result {
                Ok(packet) => match packet {
                    Packet::Connection(p) => {
                        commands.insert_resource(PlayerInfo {
                            uuid: p.uuid,
                            name: p.name,
                            hero: p.hero,
                            win: p.win,
                            lose: p.lose,
                        });
                    }
                    _ => { /* empty */ }
                },
                Err(e) => {
                    commands.insert_resource(ErrorMessage::from(e));
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn check_connection_progress(
    player_info: Option<Res<PlayerInfo>>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    if let Some(player_info) = &player_info {
        info!("Connected to the game server.");
        info!(
            "UUID: {}, Username:{}, Hero:{:?}",
            player_info.uuid.urn(),
            player_info.name,
            player_info.hero,
        );
        next_state.set(LevelStates::LoadTitle);
    }
}

fn check_connection(
    mut commands: Commands,
    message: Option<Res<ErrorMessage>>,
    mut next_state: ResMut<NextState<LevelStates>>,
    mut counter: ResMut<RetryCounter>,
    mut scene_timer: ResMut<SceneTimer>,
    time: Res<Time>,
) {
    if message.is_some() {
        next_state.set(LevelStates::Error);
        return;
    }

    scene_timer.tick(time.delta_secs());
    if scene_timer.elapsed_sec() >= TIMEOUT {
        scene_timer.reset();

        counter.0 += 1;
        if counter.0 > MAX_RETRY_COUNT {
            error!("The connection to the game server timed out.");
            commands.insert_resource(ErrorMessage::new(
                "connection_failed",
                "Failed to connect to the game server.",
            ));
            next_state.set(LevelStates::Error);
        }
    }
}
