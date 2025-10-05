// Import necessary Bevy modules.
use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::assets::config::ConfigData;

use super::*;

// --- CONSTANTS ---

const TIMEOUT: f32 = 5.0;
const MAX_RETRY_COUNT: u32 = 3;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::Connect),
            (debug_label, connect_game_server, setup_timeout_retry),
        )
        .add_systems(OnExit(LevelStates::Connect), cleanup_timeout_retry)
        .add_systems(
            Update,
            (
                check_and_retry_connection_timeout,
                check_connection_progress,
            )
                .run_if(in_state(LevelStates::Connect)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: Connect");
}

#[cfg(not(target_arch = "wasm32"))]
fn connect_game_server(mut commands: Commands) {
    commands.insert_resource(ErrorMessage {
        tag: "unsupport_platform".into(),
        message: "This platform is not supported.".into(),
    });
}

#[cfg(target_arch = "wasm32")]
fn connect_game_server(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config_assets: Res<Assets<ConfigData>>,
) {
    let handle: Handle<ConfigData> = asset_server.load(CONFIG_PATH);
    let Some(config) = config_assets.get(handle.id()) else {
        commands.insert_resource(ErrorMessage {
            tag: "connection_failed".into(),
            message: "Failed to connect to the game server.".into(),
        });
        return;
    };

    let result = WebSocketManager::connect(&config.server_url);
    let ws = match result {
        Ok(ws) => ws,
        Err(e) => {
            error!("Failed to connect to the game server: {:?}", e);
            commands.insert_resource(ErrorMessage {
                tag: "connection_failed".into(),
                message: "Failed to connect to the game server.".into(),
            });
            return;
        }
    };

    commands.insert_resource(ws);
}

// --- UPDATE SYSTEMS ---

#[cfg(not(target_arch = "wasm32"))]
fn check_connection_progress(mut commands: Commands) {
    commands.insert_resource(ErrorMessage {
        tag: "unsupport_platform".into(),
        message: "This platform is not supported.".into(),
    });
}

#[cfg(target_arch = "wasm32")]
fn check_connection_progress(
    ws: Option<Res<WebSocketManager>>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    if ws.is_some() {
        next_state.set(LevelStates::LoadGame);
    }
}

fn check_and_retry_connection_timeout(
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
            commands.insert_resource(ErrorMessage {
                tag: "connection_failed".into(),
                message: "Failed to connect to the game server.".into(),
            });
            next_state.set(LevelStates::Error);
        }
    }
}
