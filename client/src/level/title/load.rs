// Import necessary Bevy modules.
use bevy::prelude::*;
use bevy_spine::SkeletonData;
use protocol::Hero;

use super::*;

// --- CONSTANTS ---

const TIMEOUT: f32 = 5.0;
const MAX_RETRY_COUNT: u32 = 2;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::LoadTitle),
            (debug_label, setup_timeout_retry, load_necessary_assets),
        )
        .add_systems(OnExit(LevelStates::LoadTitle), cleanup_timeout_retry)
        .add_systems(
            Update,
            (
                check_loading_progress,
                update_asset_loading_progress::<TitleAssets>,
                check_and_retry_asset_load_timeout,
            )
                .run_if(in_state(LevelStates::LoadTitle)),
        );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            Update,
            packet_receive_loop.run_if(in_state(LevelStates::LoadTitle)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: LoadTitle");
}

fn load_necessary_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_info: Res<PlayerInfo>,
) {
    load_assets(&mut commands, &asset_server, player_info.hero);
}

fn load_assets(commands: &mut Commands, asset_server: &AssetServer, hero: Hero) {
    let mut loading_assets = TitleAssets::default();

    // --- Font Loading ---
    let handle: Handle<Font> = asset_server.load(FONT_PATH);
    loading_assets.push(handle);

    // --- Texture Loading ---
    let handle: Handle<Image> = asset_server.load(IMG_PATH_BACKGROUND);
    loading_assets.push(handle);

    let handle: Handle<Image> = asset_server.load(IMG_PATH_LABEL_DECO_0);
    loading_assets.push(handle);

    let handle: Handle<Image> = asset_server.load(IMG_PATH_LABEL_DECO_1);
    loading_assets.push(handle);

    let handle: Handle<Image> = asset_server.load(IMG_PATH_LABEL_DECO_2);
    loading_assets.push(handle);

    // --- Model Loading ---
    let path = MODEL_PATH_HEROS.get(&hero).copied().unwrap();
    let handle: Handle<SkeletonData> = asset_server.load(path);
    loading_assets.push(handle);

    // --- Resource Insersion ---
    commands.insert_resource(loading_assets);
}

// --- UPDATE SYSTEMS ---

fn check_loading_progress(
    asset_server: Res<AssetServer>,
    loading_assets: Res<TitleAssets>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    let all_loaded = loading_assets
        .ids()
        .iter()
        .all(|&id| asset_server.is_loaded_with_dependencies(id));

    if all_loaded {
        next_state.set(LevelStates::InitTitle);
    }
}

fn check_and_retry_asset_load_timeout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<LevelStates>>,
    mut counter: ResMut<RetryCounter>,
    mut scene_timer: ResMut<SceneTimer>,
    player_info: Res<PlayerInfo>,
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
            load_assets(&mut commands, &asset_server, player_info.hero);
        }
    }
}
