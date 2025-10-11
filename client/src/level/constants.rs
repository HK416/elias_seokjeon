pub use std::f32::consts::{PI, TAU};

// Import necessary Bevy modules.
use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
pub const SYSTEM_VOLUME_KEY: &str = "system_volume";

pub const LOADING_BAR_COLOR: Color = Color::srgb(0.2, 0.8, 0.2);
pub const BTN_BG_COLOR: Color = Color::srgb(176.0 / 255.0, 221.0 / 255.0, 127.0 / 255.0);
pub const BTN_BG_BORDER_COLOR: Color = Color::srgb(104.0 / 255.0, 160.0 / 255.0, 76.0 / 255.0);

pub const PAT_IDLE: &str = "Pat_Idle";
pub const PAT_END: &str = "Pat_End";
pub const TOUCH_IDLE: &str = "Touch_Idle";
pub const TOUCH_END: &str = "Touch_End";
pub const SMASH_END_1: &str = "Smash_End_1";
pub const SMASH_END_2: &str = "Smash_End_2";

pub const BUTTER_TITLE_IDLE: &str = "Idle_1";
pub const KOMMY_TITLE_TAUNT: &str = "Taunt_1";

pub const BALL_MOVE_RANGE: f32 = 30.0;
pub const GRABBED_TIME_THRESHOLD: f32 = 0.25;
pub const BALL_WAVE_DURATION: f32 = 0.5;
