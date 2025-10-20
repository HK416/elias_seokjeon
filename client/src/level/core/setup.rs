// Import necessary Bevy modules.
use bevy::prelude::*;

use crate::assets::{
    config::ConfigData,
    locale::{Locale, LocalizationAssets, LocalizationData},
    sound::SystemVolume,
};

#[cfg(target_arch = "wasm32")]
use crate::web::*;

use super::*;

// --- CONSTANTS ---

const TIMEOUT: f32 = 5.0;
const MAX_RETRY_COUNT: u32 = 3;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::Setup),
            (
                debug_label,
                setup_timeout_retry,
                setup_loading_screen,
                load_necessary_assets,
                setup_locale,
                setup_volume,
            ),
        )
        .add_systems(OnExit(LevelStates::Setup), cleanup_timeout_retry)
        .add_systems(
            Update,
            (
                check_loading_progress,
                update_asset_loading_progress::<SystemAssets>,
                check_and_retry_asset_load_timeout,
            )
                .run_if(in_state(LevelStates::Setup)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: Setup");
}

fn load_necessary_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    load_assets(&mut commands, &asset_server);
}

fn load_assets(commands: &mut Commands, asset_server: &AssetServer) {
    let mut loading_assets = SystemAssets::default();
    let mut localizations = LocalizationAssets::default();

    // --- Config Loading ---
    let handle: Handle<ConfigData> = asset_server.load(CONFIG_PATH);
    loading_assets.push(handle);

    // --- Locale Loading ---
    // Load localization data for each supported language.
    let handle: Handle<LocalizationData> = asset_server.load(LOCALE_PATH_EN);
    localizations.locale.insert(Locale::En, handle.clone());
    loading_assets.push(handle);

    let handle: Handle<LocalizationData> = asset_server.load(LOCALE_PATH_JA);
    localizations.locale.insert(Locale::Ja, handle.clone());
    loading_assets.push(handle);

    let handle: Handle<LocalizationData> = asset_server.load(LOCALE_PATH_KO);
    localizations.locale.insert(Locale::Ko, handle.clone());
    loading_assets.push(handle);

    // --- Font Loading ---
    let handle: Handle<Font> = asset_server.load(FONT_PATH);
    loading_assets.push(handle);

    // --- Resource Insertion ---
    commands.insert_resource(loading_assets);
    commands.insert_resource(localizations);
}

#[cfg(target_arch = "wasm32")]
fn setup_locale(mut commands: Commands) {
    let locale = window()
        .and_then(|w| w.navigator().language())
        .unwrap_or_else(|| "en-US".to_string());
    info!("Detected browser language: {}", locale);

    let locale = match locale.as_str() {
        "ja-JP" => Locale::Ja,
        "ko-KR" => Locale::Ko,
        _ => Locale::En,
    };
    info!("Use language: {}", locale);
    commands.insert_resource(locale);
}

#[cfg(not(target_arch = "wasm32"))]
fn setup_locale(mut commands: Commands) {
    let locale = Locale::Ko;
    info!("Use default language: {}", locale);
    commands.insert_resource(locale);
}

#[cfg(target_arch = "wasm32")]
fn setup_volume(mut commands: Commands) {
    if let Some(storage) = get_local_storage()
        && let Ok(storage_item) = storage.get_item(SYSTEM_VOLUME_KEY)
        && let Some(volume_str) = storage_item
        && let Ok(system_volume) = serde_json::from_str::<SystemVolume>(&volume_str)
    {
        commands.insert_resource(system_volume);
    } else {
        commands.insert_resource(SystemVolume::default());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn setup_volume(mut commands: Commands) {
    commands.insert_resource(SystemVolume::default());
}

// --- UPDATE SYSTEMS ---

fn check_loading_progress(
    asset_server: Res<AssetServer>,
    loading_assets: Res<SystemAssets>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    let all_loaded = loading_assets
        .ids()
        .iter()
        .all(|&id| asset_server.is_loaded_with_dependencies(id));

    if all_loaded {
        next_state.set(LevelStates::Connect);
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
