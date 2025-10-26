// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- CONSTANTS ---

const TIMEOUT: f32 = 5.0;
const MAX_RETRY_COUNT: u32 = 2;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::LoadEnterGame),
            (debug_label, setup_timeout_retry, load_necessary_assets),
        )
        .add_systems(OnExit(LevelStates::LoadEnterGame), cleanup_timeout_retry)
        .add_systems(
            Update,
            (
                check_loading_progress,
                update_asset_loading_progress::<EnterGameAssets>,
                check_and_retry_asset_load_timeout,
            )
                .run_if(in_state(LevelStates::LoadEnterGame)),
        );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            Update,
            packet_receive_loop.run_if(in_state(LevelStates::LoadEnterGame)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: LoadEnterGame");
}

fn load_necessary_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    load_assets(&mut commands, &asset_server);
}

fn load_assets(commands: &mut Commands, asset_server: &AssetServer) {
    let mut loading_assets = EnterGameAssets::default();

    // --- Font Loading ---
    let handle: Handle<Font> = asset_server.load(FONT_PATH);
    loading_assets.push(handle);

    // --- Background Loading ---
    let handle: Handle<Image> = asset_server.load(IMG_PATH_BACKGROUND_BLURED);
    loading_assets.push(handle);

    // --- Resource Insertion ---
    commands.insert_resource(loading_assets);
}

// --- UPDATE SYSTEMS ---

fn check_loading_progress(
    asset_server: Res<AssetServer>,
    loading_assets: Res<EnterGameAssets>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    let all_loaded = loading_assets
        .ids()
        .iter()
        .all(|&id| asset_server.is_loaded_with_dependencies(id));

    if all_loaded {
        next_state.set(LevelStates::InitEnterGame);
    }
}

fn check_and_retry_asset_load_timeout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<LevelStates>>,
    mut counter: ResMut<RetryCounter>,
    mut scene_timer: ResMut<SceneTimer>,
    time: Res<Time>,
) {
    scene_timer.tick(time.delta_secs());
    if scene_timer.elapsed_sec() >= TIMEOUT {
        scene_timer.reset();

        counter.0 += 1;
        if counter.0 > MAX_RETRY_COUNT {
            error!("Asset load request timed out.");
            commands.insert_resource(ErrorMessage::new(
                "asset_load_timeout",
                "Asset load request timed out.\nPlease refresh your browser.",
            ));
            next_state.set(LevelStates::Error);
        } else {
            load_assets(&mut commands, &asset_server);
        }
    }
}
