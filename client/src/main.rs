mod assets;
mod collider;
mod level;
mod resizable_font;
mod translatable_text;
mod web;

use std::num::NonZeroU32;

// Import necessary Bevy modules.
use bevy::{
    asset::AssetMetaCheck,
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_spine::SpinePlugin;

const WND_WIDTH: u32 = 1920;
const WND_HEIGHT: u32 = 1080;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Elias: Seokjeon".into(),
                        resolution: (WND_WIDTH, WND_HEIGHT).into(),
                        resizable: true,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        desired_maximum_frame_latency: Some(NonZeroU32::new(3).unwrap()),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..Default::default()
                })
                .set(LogPlugin {
                    level: if cfg!(feature = "no-debuging-log") {
                        Level::WARN
                    } else {
                        Level::INFO
                    },
                    ..Default::default()
                }),
        )
        .add_plugins(SpinePlugin)
        .add_plugins(assets::InnerPlugin)
        .add_plugins(collider::InnerPlugin)
        .add_plugins(level::InnerPlugin)
        .add_plugins(resizable_font::InnerPlugin)
        .add_plugins(translatable_text::InnerPlugin)
        .run();
}
