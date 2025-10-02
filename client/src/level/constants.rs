// Import necessary Bevy modules.
use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
pub const SYSTEM_VOLUME_KEY: &str = "system_volume";

pub const LOADING_BAR_COLOR: Color = Color::srgb(0.2, 0.8, 0.2);
