// Import necessary Bevy modules.
use bevy::prelude::*;
use bevy_spine::{SkeletonController, Spine, SpineBundle, SpineReadyEvent};

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::Error),
            (debug_label, test_spine_spawn, setup_error_screen),
        )
        .add_systems(Update, play_animation);
    }
}

// --- RESOURCES ---

#[derive(Resource)]
pub struct ErrorMessage(pub String);

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: Error");
}

fn test_spine_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpineBundle {
        skeleton: asset_server.load("spine/Butter.model").into(),
        ..default()
    });
}

fn setup_error_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    message: Option<Res<ErrorMessage>>,
) {
    // Debuging code
    commands.spawn(Camera2d::default());

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
                    let font = asset_server.load(FONT_PATH_NOTOSANS_BOLD);
                    let message = message
                        .as_ref()
                        .map(|s| s.0.as_str())
                        .unwrap_or("Unknown error.");

                    parent.spawn((
                        Text::new(message),
                        TextFont::from_font(font),
                        TextLayout::new_with_justify(JustifyText::Center),
                        ResizableFont::vertical(1280.0, 48.0),
                        TextColor::BLACK,
                        Node::default(),
                        Visibility::Inherited,
                    ));
                });
        });
}

// --- UPDATE SYSTEMS ---

pub fn play_animation(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine>,
) {
    for event in spine_ready_event.read() {
        info!("on_spawn!");
        let Ok(mut spine) = spine_query.get_mut(event.entity) else {
            continue;
        };

        let Spine(SkeletonController {
            skeleton,
            animation_state,
            ..
        }) = spine.as_mut();
        let _ = skeleton.set_skin_by_name("Normal");
        skeleton.set_scale(Vec2::splat(0.5));
        let _ = animation_state.set_animation_by_name(0, "Idle_1", true);
    }
}
