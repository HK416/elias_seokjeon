pub use std::f32::consts::{FRAC_PI_2, PI, TAU};

// Import necessary Bevy modules.
use bevy::prelude::*;
use protocol::NUM_HEROS;

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
// --- PURPLE ---
#[rustfmt::skip] pub const BG_PURPLE_COLOR_0: Color = Color::srgb(110.0 / 255.0, 77.0 / 255.0, 135.0 / 255.0);
#[rustfmt::skip] pub const BORDER_PURPLE_COLOR_0: Color = Color::srgb(58.0 / 255.0, 39.0 / 255.0, 78.0 / 255.0);
// --- BROWN ---
#[rustfmt::skip] pub const BG_BROWN_COLOR_0: Color = Color::srgb(246.0 / 255.0, 229.0 / 255.0, 193.0 / 255.0);
#[rustfmt::skip] pub const BORDER_BROWN_COLOR_0: Color = Color::srgb(122.0 / 255.0, 98.0 / 255.0, 96.0 / 255.0);

pub const BALL_BONE_NAME: &str = "Character_Ball_Move";
pub const HEAD_BONE_NAME: &str = "Character_Pat";

pub const IDLE: &str = "Idle_1";
pub const PAT_IDLE: &str = "Pat_Idle";
pub const PAT_END: &str = "Pat_End";
pub const TOUCH_IDLE: &str = "Touch_Idle";
pub const TOUCH_END: &str = "Touch_End";
pub const SMASH_END_1: &str = "Smash_End_1";
pub const SMASH_END_2: &str = "Smash_End_2";
pub const HAPPY_1: &str = "Happy_1";
pub const SAD_1: &str = "Sad_1";

const ALICE_TITLE: &str = "Happy_5";
const AMELIA_TITLE: &str = "Idle_1";
const ASHUR_TITLE: &str = "Idle_1";
const AYA_TITLE: &str = "Idle_1";
const BELITA_TITLE: &str = "Idle_1";
const BENI_TITLE: &str = "Idle_1";
const BIGWOOD_TITLE: &str = "Happy_2";
const BUTTER_TITLE: &str = "Idle_1";
const CANNA_TITLE: &str = "Idle_1";
const CHLOE_TITLE: &str = "Idle_1";
const DAYA_TITLE: &str = "Happy_4";
const DIANA_TITLE: &str = "Idle_1";
const ELENA_TITLE: &str = "Idle_1";
const EPICA_TITLE: &str = "Dance_1";
const ERPIN_TITLE: &str = "Idle_1";
const ESPI_TITLE: &str = "Idle_1";
const FESTA_TITLE: &str = "Rock_1";
const FRICLE_TITLE: &str = "Idle_1";
const GABIA_TITLE: &str = "Idle_1";
const HILDE_TITLE: &str = "Idle_1";
const IFRIT_TITLE: &str = "Sulky_1";
// const JADE_TITLE: &str = "Idle_1";
const JUBEE_TITLE: &str = "Idle_1";
const KIDIAN_TITLE: &str = "Idle_2";
const KOMMY_TITLE: &str = "Taunt_1";
const MAYO_TITLE: &str = "Happy_2";
const ROHNE_TITLE: &str = "Idle_1";
const SPEAKI_TITLE: &str = "Idle_1";
const XION_TITLE: &str = "Ganzi_4";
pub const TITLE_ANIM: [&str; NUM_HEROS] = [
    ALICE_TITLE,
    AMELIA_TITLE,
    ASHUR_TITLE,
    AYA_TITLE,
    BELITA_TITLE,
    BENI_TITLE,
    BIGWOOD_TITLE,
    BUTTER_TITLE,
    CANNA_TITLE,
    CHLOE_TITLE,
    DAYA_TITLE,
    DIANA_TITLE,
    ELENA_TITLE,
    EPICA_TITLE,
    ERPIN_TITLE,
    ESPI_TITLE,
    FESTA_TITLE,
    FRICLE_TITLE,
    GABIA_TITLE,
    HILDE_TITLE,
    IFRIT_TITLE,
    // JADE_TITLE,
    JUBEE_TITLE,
    KIDIAN_TITLE,
    KOMMY_TITLE,
    MAYO_TITLE,
    ROHNE_TITLE,
    SPEAKI_TITLE,
    XION_TITLE,
];

pub const BALL_MOVE_RANGE: f32 = 30.0;
pub const GRABBED_TIME_THRESHOLD: f32 = 0.25;
pub const BALL_WAVE_DURATION: f32 = 0.5;

pub const UI_POPUP_DURATION: f32 = 0.2;

pub const THROW_RANGE: f32 = 300.0;

pub const GUIDE_CYCLE: f32 = 2.0;
pub const GUIDE_LEFT_BEG_VMIN_X: f32 = 10.0;
pub const GUIDE_LEFT_END_VMIN_X: f32 = -10.0;
pub const GUIDE_RIGHT_BEG_VMIN_X: f32 = -10.0;
pub const GUIDE_RIGHT_END_VMIN_X: f32 = 10.0;
pub const GUIDE_BEG_VMIN_Y: f32 = -5.0;
pub const GUIDE_END_VMIN_Y: f32 = 5.0;
