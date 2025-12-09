// Import necessary Bevy modules.
use bevy::prelude::*;

use crate::assets::locale::{Locale, LocalizationAssets, LocalizationData};

use super::*;

// --- CONSTANTS ---
const SCENE_DURATION: f32 = UI_POPUP_DURATION;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::SwitchToTitleMessage),
            (
                debug_label,
                setup_scene_timer,
                hide_enter_game_entities,
                show_in_title_entities,
                show_message_entities,
                setup_message_interfaces,
                setup_message,
            ),
        )
        .add_systems(
            OnExit(LevelStates::SwitchToTitleMessage),
            cleanup_scene_timer,
        )
        .add_systems(
            Update,
            update_scene_timer.run_if(in_state(LevelStates::SwitchToTitleMessage)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: SwitchToTitleMessage");
}

fn setup_scene_timer(mut commands: Commands) {
    commands.insert_resource(SceneTimer::default());
}

fn hide_enter_game_entities(
    mut query: Query<&mut Visibility, (With<EnterGameLevelEntity>, With<TitleLevelRoot>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

#[allow(clippy::type_complexity)]
fn show_in_title_entities(
    mut query: Query<&mut Visibility, (With<TitleBackground>, With<TitleLevelRoot>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn show_message_entities(
    mut query: Query<&mut Visibility, (With<TitleMessageLevelEntity>, With<TitleLevelRoot>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn setup_message_interfaces(
    mut commands: Commands,
    query: Query<Entity, (With<UiAnimationTarget>, With<TitleMessageLevelEntity>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(UiBackOutScale::new(SCENE_DURATION, Vec2::ZERO, Vec2::ONE));
    }
}

fn setup_message(
    mut commands: Commands,
    message: Option<Res<ErrorMessage>>,
    mut query: Query<&mut Text, With<TitleMessageText>>,
    locale_data: Res<Assets<LocalizationData>>,
    locale_assets: Res<LocalizationAssets>,
    locale: Res<Locale>,
) {
    let Ok(mut text) = query.single_mut() else {
        return;
    };
    let message = message
        .as_ref()
        .map(|e| {
            if let Some(handle) = locale_assets.locale.get(&*locale)
                && let Some(data) = locale_data.get(handle.id())
                && let Some(message) = data.0.get(&e.tag)
            {
                let mut buffer = Vec::new();
                let terminator = message.split_terminator("{}");
                let mut args = e.args.iter();
                for word in terminator {
                    buffer.push(word.to_string());
                    if let Some(arg) = args.next() {
                        buffer.push(match arg {
                            MessageArgs::String(s) => s.clone(),
                            MessageArgs::Integer(i) => i.to_string(),
                        });
                    }
                }
                buffer.into_iter().collect()
            } else {
                e.message.clone()
            }
        })
        .unwrap_or("Unknown error".to_string());

    *text = Text::new(message);
    commands.remove_resource::<ErrorMessage>();
}

// --- CLEANUP SYSTEMS ---

fn cleanup_scene_timer(mut commands: Commands) {
    commands.remove_resource::<SceneTimer>();
}

// --- UPDATE SYSTEMS ---

fn update_scene_timer(
    mut next_state: ResMut<NextState<LevelStates>>,
    mut scene_timer: ResMut<SceneTimer>,
    time: Res<Time>,
) {
    scene_timer.tick(time.delta_secs());
    if scene_timer.elapsed_sec() >= SCENE_DURATION {
        next_state.set(LevelStates::InTitleMessage);
    }
}
