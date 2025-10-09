// Import necessary Bevy modules.
use bevy::prelude::*;

use crate::assets::locale::{Locale, LocalizationAssets, LocalizationData};

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::Error),
            (debug_label, setup_error_screen),
        );
    }
}

// --- RESOURCES ---

#[derive(Resource)]
pub struct ErrorMessage {
    pub tag: String,
    pub message: String,
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: Error");
}

fn setup_error_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    message: Option<Res<ErrorMessage>>,
    locale: Res<Locale>,
    local_assets: Res<LocalizationAssets>,
    locale_data: Res<Assets<LocalizationData>>,
) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.8)),
            Visibility::Visible,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(50.0),
                        overflow: Overflow::scroll_y(),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Percent(3.0)),
                    BackgroundColor(Color::WHITE),
                    Visibility::Inherited,
                ))
                .with_children(|parent| {
                    let font = asset_server.load(FONT_PATH);
                    let message = message
                        .as_ref()
                        .map(|m| {
                            if let Some(handle) = local_assets.locale.get(&*locale)
                                && let Some(data) = locale_data.get(handle.id())
                                && let Some(message) = data.0.get(&m.tag)
                            {
                                message.clone()
                            } else {
                                m.message.clone()
                            }
                        })
                        .unwrap_or("Unknown error.".to_string());

                    parent.spawn((
                        Text::new(message),
                        TextFont::from(font),
                        TextLayout::new_with_justify(Justify::Center),
                        ResizableFont::vertical(1280.0, 48.0),
                        TextColor::BLACK,
                        Node::default(),
                        Visibility::Inherited,
                    ));
                });
        });
}
