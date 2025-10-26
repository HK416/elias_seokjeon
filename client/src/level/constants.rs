pub use std::f32::consts::{PI, TAU};

// Import necessary Bevy modules.
use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
pub const SYSTEM_VOLUME_KEY: &str = "system_volume";

pub const LOADING_BAR_COLOR: Color = Color::srgb(0.2, 0.8, 0.2);
// --- GREEN ---
#[rustfmt::skip] pub const BG_GREEN_COLOR_0: Color = Color::srgb(176.0 / 255.0, 221.0 / 255.0, 127.0 / 255.0);
#[rustfmt::skip] pub const BORDER_GREEN_COLOR_0: Color = Color::srgb(104.0 / 255.0, 160.0 / 255.0, 76.0 / 255.0);
#[rustfmt::skip] pub const BG_GREEN_COLOR_1: Color = Color::srgb(227.0 / 255.0, 241.0 / 255.0, 179.0 / 255.0);
#[rustfmt::skip] pub const BORDER_GREEN_COLOR_1: Color = Color::srgb(210.0 / 255.0, 233.0 / 255.0, 146.0 / 255.0);
#[rustfmt::skip] pub const BG_GREEN_COLOR_2: Color = Color::srgb(246.0 / 255.0, 251.0 / 255.0, 233.0 / 255.0);
#[rustfmt::skip] pub const BORDER_GREEN_COLOR_2: Color = Color::srgb(168.0 / 255.0, 201.0 / 255.0, 117.0 / 255.0);
#[rustfmt::skip] pub const BG_GREEN_COLOR_3: Color = Color::srgb(204.0 / 255.0, 230.0 / 255.0, 146.0 / 255.0);
// --- YELLO ---
#[rustfmt::skip] pub const BG_YELLO_COLOR_0: Color = Color::srgb(250.0 / 255.0, 224.0 / 255.0, 132.0 / 255.0);
#[rustfmt::skip] pub const BORDER_YELLO_COLOR_0: Color = Color::srgb(232.0 / 255.0, 167.0 / 255.0, 75.0 / 255.0);
// --- RED ---
#[rustfmt::skip] pub const BG_RED_COLOR_0: Color = Color::srgb(227.0 / 255.0, 96.0 / 255.0, 115.0 / 255.0);
#[rustfmt::skip] pub const BORDER_RED_COLOR_0: Color = Color::srgb(221.0 / 255.0, 81.0 / 255.0, 100.0 / 255.0);

pub const PAT_IDLE: &str = "Pat_Idle";
pub const PAT_END: &str = "Pat_End";
pub const TOUCH_IDLE: &str = "Touch_Idle";
pub const TOUCH_END: &str = "Touch_End";
pub const SMASH_END_1: &str = "Smash_End_1";
pub const SMASH_END_2: &str = "Smash_End_2";

pub const BUTTER_TITLE_IDLE: &str = "Idle_1";
pub const KOMMY_TITLE_TAUNT: &str = "Taunt_1";

pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const BALL_MOVE_RANGE: f32 = 30.0;
pub const GRABBED_TIME_THRESHOLD: f32 = 0.25;
pub const BALL_WAVE_DURATION: f32 = 0.5;

pub const UI_POPUP_DURATION: f32 = 0.2;
