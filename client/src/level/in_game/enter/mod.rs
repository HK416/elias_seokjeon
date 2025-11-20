mod init;
mod switch;

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
        app.add_plugins(init::InnerPlugin)
            .add_plugins(switch::InnerPlugin)
            .add_systems(
                OnEnter(LevelStates::LoadGame),
                (debug_label, setup_timeout_retry, load_necessary_assets),
            )
            .add_systems(OnExit(LevelStates::LoadGame), cleanup_timeout_retry)
            .add_systems(
                Update,
                (
                    check_loading_progress,
                    check_and_retry_asset_load_timeout,
                    update_loading_progress,
                    update_loading_minimi,
                )
                    .run_if(in_state(LevelStates::LoadGame)),
            );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            PreUpdate,
            handle_received_packets.run_if(in_state(LevelStates::LoadGame)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: LoadGame");
}

fn load_necessary_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_info: Res<PlayerInfo>,
    other_info: Res<OtherInfo>,
) {
    load_assets(
        &mut commands,
        &asset_server,
        &[player_info.hero, other_info.hero],
    );
}

fn load_assets(commands: &mut Commands, asset_server: &AssetServer, heros: &[Hero]) {
    let mut loading_assets = InGameAssets::default();

    // --- Font Loading ---
    let handle: Handle<Font> = asset_server.load(FONT_PATH);
    loading_assets.push(handle);

    // --- Model Loading ---
    for hero in heros {
        let path = MODEL_PATH_HEROS.get(hero).copied().unwrap();
        let handle: Handle<SkeletonData> = asset_server.load(path);
        loading_assets.push(handle);
    }

    // --- Texture Loading ---
    let handle: Handle<Image> = asset_server.load(IMG_PATH_FX_FIRECARTOON);
    loading_assets.push(handle);

    let handle: Handle<TextureAtlasLayout> = asset_server.load(ATLAS_PATH_FX_FIRECARTOON);
    loading_assets.push(handle);

    let handle: Handle<Image> = asset_server.load(IMG_PATH_PVP_INGAME_VS);
    loading_assets.push(handle);

    let handle: Handle<Image> = asset_server.load(IMG_PATH_WIND_INDICATOR_DECO);
    loading_assets.push(handle);

    let handle: Handle<Image> = asset_server.load(IMG_PATH_GREEN_FLAG);
    loading_assets.push(handle);

    let handle: Handle<Image> = asset_server.load(IMG_PATH_RED_FLAG);
    loading_assets.push(handle);

    let handle: Handle<Image> = asset_server.load(IMG_PATH_RED_DOT);
    loading_assets.push(handle);

    let handle: Handle<Image> = asset_server.load(IMG_PATH_ID_PANEL);
    loading_assets.push(handle);

    let handle: Handle<TextureAtlasLayout> = asset_server.load(ATLAS_PATH_ID_PANEL);
    loading_assets.push(handle);

    // --- Stage Loading ---
    for path in IMG_PATH_BG_FAIRY {
        let handle: Handle<Image> = asset_server.load(path);
        loading_assets.push(handle);
    }

    // --- Resource Insertion ---
    commands.insert_resource(loading_assets);
}

// --- PREUPDATE SYSTEMS ---

#[cfg(target_arch = "wasm32")]
fn handle_received_packets(
    mut commands: Commands,
    mut next_state: ResMut<NextState<LevelStates>>,
    network: Res<Network>,
) {
    for result in network.try_iter() {
        match result {
            Ok(packet) => match packet {
                Packet::GameLoadTimeout => {
                    commands.insert_resource(ErrorMessage::new(
                        "game_load_timeout",
                        "Failed to enter the game due to a connection timeout.",
                    ));
                    next_state.set(LevelStates::SwitchToTitleMessage);
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

// --- UPDATE SYSTEMS ---

fn check_loading_progress(
    asset_server: Res<AssetServer>,
    loading_assets: Res<InGameAssets>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    let all_loaded = loading_assets
        .ids()
        .iter()
        .all(|&id| asset_server.is_loaded_with_dependencies(id));

    if all_loaded {
        next_state.set(LevelStates::InitGame);
    }
}

#[allow(clippy::too_many_arguments)]
fn check_and_retry_asset_load_timeout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<LevelStates>>,
    mut counter: ResMut<RetryCounter>,
    mut scene_timer: ResMut<SceneTimer>,
    player_info: Res<PlayerInfo>,
    other_info: Res<OtherInfo>,
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
            load_assets(
                &mut commands,
                &asset_server,
                &[player_info.hero, other_info.hero],
            );
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_loading_progress(
    asset_server: Res<AssetServer>,
    loading_assets: Res<InGameAssets>,
    mut set: ParamSet<(
        Query<&mut Node, With<EnterGameLoadingBar>>,
        Query<&mut Node, With<EnterGameLoadingCursor>>,
    )>,
) {
    let loaded_count = loading_assets
        .ids()
        .iter()
        .filter(|&&id| asset_server.is_loaded_with_dependencies(id))
        .count();

    let total_count = loading_assets.len();
    let progress = if total_count > 0 {
        loaded_count as f32 / total_count as f32
    } else {
        1.0
    };

    if let Ok(mut node) = set.p0().single_mut() {
        node.width = Val::Percent(progress * 100.0);
    }

    if let Ok(mut node) = set.p1().single_mut() {
        node.left = Val::Percent(progress * 100.0);
    }
}

fn update_loading_minimi(
    mut query: Query<(&mut ImageNode, &mut AnimationTimer), With<EnterGameLevelEntity>>,
    time: Res<Time>,
) {
    for (mut image_node, mut timer) in query.iter_mut() {
        timer.tick(time.delta_secs());
        if let Some(atlas) = image_node.texture_atlas.as_mut() {
            atlas.index = timer.frame_index();
        }
    }
}
