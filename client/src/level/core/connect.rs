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
        commands.insert_resource(ErrorMessage {
            tag: "connection_failed".into(),
            message: "Failed to connect to the game server.".into(),
        });
        return;
    };

    let result = WebSocket::new(&config.server_url);
    let socket = match result {
        Ok(socket) => socket,
        Err(e) => {
            error!("Failed to connect to the game server: {:?}", e);
            commands.insert_resource(ErrorMessage {
                tag: "connection_failed".into(),
                message: "Failed to connect to the game server".into(),
            });
            return;
        }
    };

    use protocol::Packet;
    let (sender, receiver) = flume::unbounded();
    let closure = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        if let Ok(text) = e.data().dyn_into::<js_sys::JsString>()
            && let Ok(msg) = serde_json::from_str::<Packet>(&text.as_string().unwrap())
        {
            info!("Received packet: {:?}", msg);
            let _ = sender.send(msg);
        }
    });
    socket.set_binary_type(BinaryType::Arraybuffer);
    socket.set_onmessage(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    commands.insert_resource(Network::new(socket, receiver));
}

// --- UPDATE SYSTEMS ---

#[cfg(target_arch = "wasm32")]
fn packet_receive_loop(mut commands: Commands, network: Option<Res<Network>>) {
    use protocol::{ConnectionPacket, Header};
    if let Some(network) = network.as_ref() {
        for msg in network.receiver.try_iter() {
            match msg.header {
                Header::Connection => {
                    let connection = serde_json::from_str::<ConnectionPacket>(&msg.json).unwrap();
                    commands.insert_resource(PlayerInfo {
                        uuid: connection.uuid,
                        username: connection.username,
                    });
                }
                _ => { /* empty */ }
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
            "UUID: {}, Username:{}",
            player_info.uuid.urn(),
            player_info.username
        );
        next_state.set(LevelStates::InitOption);
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
            commands.insert_resource(ErrorMessage {
                tag: "connection_failed".into(),
                message: "Failed to connect to the game server.".into(),
            });
            next_state.set(LevelStates::Error);
        }
    }
}
