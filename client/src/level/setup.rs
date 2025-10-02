// Import necessary Bevy modules.
use bevy::prelude::*;

use crate::{
    assets::{
        locale::{Locale, LocalizationAssets, LocalizationData},
        sound::SystemVolume,
    },
    level::error::ErrorMessage,
};

#[cfg(target_arch = "wasm32")]
use crate::web::*;

use super::*;

// --- CONSTANTS ---

const TIMEOUT: f32 = 5.0;
const MAX_RETRY_COUNT: u32 = 5;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::Setup),
            (
                debug_label,
                setup_asset_load_timeout_retry,
                setup_loading_screen,
                load_necessary_assets,
                setup_locale,
                setup_volume,
            ),
        )
        .add_systems(OnExit(LevelStates::Setup), cleanup_asset_load_timeout_retry)
        .add_systems(
            Update,
            (
                check_loading_progress,
                update_loading_progress,
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

fn setup_asset_load_timeout_retry(mut commands: Commands) {
    commands.insert_resource(SceneTimer::default());
    commands.insert_resource(RetryCounter::default());
}

fn setup_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, LoadingStateRoot));

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Vw(20.0),
                height: Val::Vh(5.0),
                bottom: Val::Vh(3.0),
                right: Val::Vw(3.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            LoadingStateRoot,
        ))
        .with_children(|parent| {
            // Container for the loading text.
            parent
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(88.0),
                    ..Default::default()
                },))
                .with_children(|parent| {
                    // Spawn the "Now Loading..." text element.
                    let font = asset_server.load(FONT_PATH_NOTOSANS_BOLD);
                    parent.spawn((
                        Text::new("Now Loading..."),
                        TextFont::from_font(font).with_font_size(24.0),
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextColor::WHITE,
                        ResizableFont::vertical(1280.0, 24.0),
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            overflow: Overflow::hidden(),
                            ..Default::default()
                        },
                        LoadingText,
                        ZIndex(2),
                    ));
                });

            // Container for the loading progress bar.
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(12.0),
                        border: UiRect::all(Val::Percent(0.25)),
                        ..Default::default()
                    },
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Percent(50.0)),
                ))
                .with_children(|parent| {
                    // The actual loading bar that will be filled.
                    parent.spawn((
                        Node {
                            width: Val::Percent(0.0),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },
                        BorderRadius::all(Val::Percent(50.0)),
                        BackgroundColor(LOADING_BAR_COLOR),
                        LoadingBar,
                        ZIndex(1),
                    ));
                });
        });
}

fn load_necessary_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    load_assets(&mut commands, &asset_server);
}

fn load_assets(commands: &mut Commands, asset_server: &AssetServer) {
    let mut loading_assets = SystemAssets::default();
    let mut localizations = LocalizationAssets::default();

    // --- Locale Loading ---
    // Load localization data for each supported language.
    let handle: Handle<LocalizationData> = asset_server.load(LOCALE_PATH_EN);
    localizations.locale.insert(Locale::En, handle.clone());
    loading_assets.handles.push(handle.into());

    let handle: Handle<LocalizationData> = asset_server.load(LOCALE_PATH_JA);
    localizations.locale.insert(Locale::Ja, handle.clone());
    loading_assets.handles.push(handle.into());

    let handle: Handle<LocalizationData> = asset_server.load(LOCALE_PATH_KO);
    localizations.locale.insert(Locale::Ko, handle.clone());
    loading_assets.handles.push(handle.into());

    // --- Font Loading ---
    let font: Handle<Font> = asset_server.load(FONT_PATH_NOTOSANS_BOLD);
    loading_assets.handles.push(font.into());

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

// --- CLEANUP SYSTEMS ---

fn cleanup_asset_load_timeout_retry(mut commands: Commands) {
    commands.remove_resource::<SceneTimer>();
    commands.remove_resource::<RetryCounter>();
}

// --- UPDATE SYSTEMS ---

fn check_loading_progress(
    asset_server: Res<AssetServer>,
    loading_assets: Res<SystemAssets>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    let all_loaded = loading_assets
        .handles
        .iter()
        .all(|h| asset_server.is_loaded_with_dependencies(h.id()));

    if all_loaded {
        // TODO
    }
}

fn update_loading_progress(
    asset_server: Res<AssetServer>,
    loading_assets: Res<SystemAssets>,
    mut query: Query<&mut Node, With<LoadingBar>>,
) {
    let Ok(mut node) = query.single_mut() else {
        return;
    };
    let loaded_count = loading_assets
        .handles
        .iter()
        .filter(|h| asset_server.is_loaded_with_dependencies(h.id()))
        .count();

    let total_count = loading_assets.handles.len();
    let progress = if total_count > 0 {
        loaded_count as f32 / total_count as f32
    } else {
        1.0
    };

    node.width = Val::Percent(progress * 100.0);
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
            commands.insert_resource(ErrorMessage(
                "Asset load request timed out.\nPlease refresh your browser.".to_string(),
            ));
            next_state.set(LevelStates::Error);
        } else {
            load_assets(&mut commands, &asset_server);
        }
    }
}
