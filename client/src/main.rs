mod assets;
mod level;
mod resizable_font;
mod web;

use std::num::NonZeroU32;

// Import necessary Bevy modules.
use bevy::{
    asset::AssetMetaCheck,
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_spine::SpinePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Elias: Seokjeon".into(),
                    resolution: (1280.0, 720.0).into(),
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
    .add_plugins(level::InnerPlugin)
    .add_plugins(resizable_font::InnerPlugin);

    #[cfg(target_arch = "wasm32")]
    {
        let net = web::NetworkManager::connect("ws://127.0.0.1:8889");
        app.insert_non_send_resource(net);
    }

    app.run();
}
