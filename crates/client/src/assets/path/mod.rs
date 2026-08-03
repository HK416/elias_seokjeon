mod hero;

use const_format::concatcp;
use hero::*;
use protocol::NUM_HEROS;

pub trait HeroVoiceSet {
    fn all(&self) -> &[&'static str];

    fn call_player(&self) -> &[&'static str];

    fn defeat(&self) -> &[&'static str];

    fn ducth_rub_end(&self) -> &[&'static str];

    fn greeting(&self) -> &[&'static str];

    fn hit(&self) -> &[&'static str];

    fn shout(&self) -> &[&'static str];

    fn touch_1(&self) -> &[&'static str];

    fn touch_2(&self) -> &[&'static str];

    fn victory(&self) -> &[&'static str];
}

#[rustfmt::skip] pub const QUERY: &str = "?";
#[rustfmt::skip] pub const VERSION: &str = concat!("v=", env!("CARGO_PKG_VERSION_PATCH"));

#[rustfmt::skip] pub const CONFIG_PATH: &str = concatcp!("config.json", QUERY, VERSION);
#[rustfmt::skip] pub const LOCALE_PATH_EN: &str = concatcp!("locale/en.json", QUERY, VERSION);
#[rustfmt::skip] pub const LOCALE_PATH_JA: &str = concatcp!("locale/ja.json", QUERY, VERSION);
#[rustfmt::skip] pub const LOCALE_PATH_KO: &str = concatcp!("locale/ko.json", QUERY, VERSION);

#[rustfmt::skip] pub const FONT_PATH: &str = concatcp!("fonts/NotoSans-Bold.otf", QUERY, VERSION);

#[rustfmt::skip] pub const BGM_PATH_BACKGROUND: &str = concatcp!("sounds/BGM_PatchGame.sound", QUERY, VERSION);
#[rustfmt::skip] pub const BGM_PATH_INGAME_DEFEAT: &str = concatcp!("sounds/SFX_InGame_Defeat.sound", QUERY, VERSION);
#[rustfmt::skip] pub const BGM_PATH_INGAME_VICTORY: &str = concatcp!("sounds/SFX_InGame_Victory.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_BOXING_BELL: &str = concatcp!("sounds/SFX_Boxing_Bell.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_CHEER: &str = concatcp!("sounds/SFX_Cheer.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_COMMON_BUTTON_UP: &str = concatcp!("sounds/SFX_Common_ButtonUp.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_COMMON_BUTTON_DOWN: &str = concatcp!("sounds/SFX_Common_ButtonDown.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_COMMON_BUTTON_TOUCH: &str = concatcp!("sounds/SFX_Common_ButtonTouch.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_COMMON_POPUP_CLOSE: &str = concatcp!("sounds/SFX_Common_PopupClose.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_COMMON_POPUP_BUTTON_TOUCH: &str = concatcp!("sounds/SFX_Common_PopupButtonTouch.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_COMMON_POPUP_TOAST_MESSAGE: &str = concatcp!("sounds/SFX_Common_PopupToastMessage.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_COMMON_PULL_CHEEK: &str = concatcp!("sounds/SFX_Common_PullCheek.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_COMMON_PULL_CHEEK_END: &str = concatcp!("sounds/SFX_Common_PullCheekEnd.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_COMMON_RUBBING: &str = concatcp!("sounds/SFX_Common_Rubbing.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_COMMON_RUBBING_END: &str = concatcp!("sounds/SFX_Common_RubbingEnd.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_EMOTICON_HIT: &str = concatcp!("sounds/SFX_Emoticon_Hit.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_FIRE_FAIL: &str = concatcp!("sounds/SFX_FireFail.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_INGAME_TIME_OVER: &str = concatcp!("sounds/SFX_InGame_TimeOver.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_POPUP_BOBBLE: &str = concatcp!("sounds/SFX_PopupBobble.sound", QUERY, VERSION);
#[rustfmt::skip] pub const SFX_PATH_SWING: &str = concatcp!("sounds/SFX_Swing.sound", QUERY, VERSION);

#[rustfmt::skip] pub const IMG_PATH_BACKGROUND: &str = concatcp!("textures/Background.texture", QUERY, VERSION);

#[rustfmt::skip] pub const IMG_PATH_BG_FAIRY_0: &str = concatcp!("textures/BG_Fairy_0.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_BG_FAIRY_1: &str = concatcp!("textures/BG_Fairy_1.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_BG_FAIRY_2: &str = concatcp!("textures/BG_Fairy_2.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_BG_FAIRY_3: &str = concatcp!("textures/BG_Fairy_3.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_BG_FAIRY_4: &str = concatcp!("textures/BG_Fairy_4.texture", QUERY, VERSION);
pub const IMG_PATH_BG_FAIRY: [&str; 5] = [
    IMG_PATH_BG_FAIRY_0,
    IMG_PATH_BG_FAIRY_1,
    IMG_PATH_BG_FAIRY_2,
    IMG_PATH_BG_FAIRY_3,
    IMG_PATH_BG_FAIRY_4,
];

#[rustfmt::skip] pub const IMG_PATH_GAME_RESULT_DEFEAT_ICON: &str = concatcp!("textures/Game_Result_Defeat_Icon.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_GAME_RESULT_DEFEAT_TEXT: &str = concatcp!("textures/Game_Result_Defeat_Text.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_GAME_RESULT_VICTORY_ICON: &str = concatcp!("textures/Game_Result_Victory_Icon.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_GAME_RESULT_VICTORY_TEXT: &str = concatcp!("textures/Game_Result_Victory_Text.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_GAME_RESULT_DRAW_TEXT: &str = concatcp!("textures/Game_Result_Draw_Text.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_GUIDE_GESTURE: &str = concatcp!("textures/Guide_Gesture.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_HEALTH_HEART: &str = concatcp!("textures/Health_Heart.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_INGAME_TIME_ICON: &str = concatcp!("textures/Ingame_Time_Icon.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_LABEL_DECO_0: &str = concatcp!("textures/Label_Deco_0.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_LABEL_DECO_1: &str = concatcp!("textures/Label_Deco_1.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_LABEL_DECO_2: &str = concatcp!("textures/Label_Deco_2.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_LOADING_DECO: &str = concatcp!("textures/Loading_Deco.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_PATTERN_0: &str = concatcp!("textures/Pattern_0.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_PVP_INGAME_VS: &str = concatcp!("textures/PVP_Ingame_VS.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_PROJECTILE: &str = concatcp!("textures/Projectile.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_RED_DOT: &str = concatcp!("textures/Red_Dot.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_WIND_INDICATOR_ARROW: &str = concatcp!("textures/Wind_Indicator_Arrow.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_WIND_INDICATOR_DECO: &str = concatcp!("textures/Wind_Indicator_Deco.texture", QUERY, VERSION);

#[rustfmt::skip] pub const IMG_PATH_ID_PANEL: &str = concatcp!("textures/ID_Panel.texture", QUERY, VERSION);
#[rustfmt::skip] pub const ATLAS_PATH_ID_PANEL: &str = concatcp!("textures/ID_Panel.atlas", QUERY, VERSION);

#[rustfmt::skip] pub const IMG_PATH_FX_FIRECARTOON: &str = concatcp!("textures/FX_Firecartoon.texture", QUERY, VERSION);
#[rustfmt::skip] pub const ATLAS_PATH_FX_FIRECARTOON: &str = concatcp!("textures/FX_Firecartoon.atlas", QUERY, VERSION);

#[rustfmt::skip] pub const IMG_PATH_LOADING_MINIMI: &str = concatcp!("textures/Loading_minimi.texture", QUERY, VERSION);
#[rustfmt::skip] pub const ATLAS_PATH_LOADING_MINIMI: &str = concatcp!("textures/Loading_minimi.atlas", QUERY, VERSION);

pub const HERO_VOICE_SETS: [&dyn HeroVoiceSet; NUM_HEROS] = [
    &alice::HeroVoice,
    &amelia::HeroVoice,
    &ashur::HeroVoice,
    &aya::HeroVoice,
    &belita::HeroVoice,
    &beni::HeroVoice,
    &bigwood::HeroVoice,
    &butter::HeroVoice,
    &canna::HeroVoice,
    &chloe::HeroVoice,
    &daya::HeroVoice,
    &diana::HeroVoice,
    &elena::HeroVoice,
    &epica::HeroVoice,
    &erpin::HeroVoice,
    &espi::HeroVoice,
    &festa::HeroVoice,
    &fricle::HeroVoice,
    &gabia::HeroVoice,
    &hilde::HeroVoice,
    &ifrit::HeroVoice,
    // &jade::HeroVoice,
    &jubee::HeroVoice,
    &kidian::HeroVoice,
    &kommy::HeroVoice,
    &leets::HeroVoice,
    &levi::HeroVoice,
    &maestro_mk2::HeroVoice,
    &marie::HeroVoice,
    &mayo::HeroVoice,
    // &meluna::HeroVoice,
    &naia::HeroVoice,
    &ner::HeroVoice,
    &posher::HeroVoice,
    &rim::HeroVoice,
    &rohne::HeroVoice,
    &rude::HeroVoice,
    &rufo::HeroVoice,
    &selline::HeroVoice,
    &shady::HeroVoice,
    &silphir::HeroVoice,
    &sist::HeroVoice,
    &speaki::HeroVoice,
    &sylla::HeroVoice,
    &tig::HeroVoice,
    &ui::HeroVoice,
    // &velvet::HeroVoice,
    &vivi::HeroVoice,
    &xion::HeroVoice,
];

pub const MODEL_PATH_HEROS: [&str; NUM_HEROS] = [
    alice::MODEL_PATH,
    amelia::MODEL_PATH,
    ashur::MODEL_PATH,
    aya::MODEL_PATH,
    belita::MODEL_PATH,
    beni::MODEL_PATH,
    bigwood::MODEL_PATH,
    butter::MODEL_PATH,
    canna::MODEL_PATH,
    chloe::MODEL_PATH,
    daya::MODEL_PATH,
    diana::MODEL_PATH,
    elena::MODEL_PATH,
    epica::MODEL_PATH,
    erpin::MODEL_PATH,
    espi::MODEL_PATH,
    festa::MODEL_PATH,
    fricle::MODEL_PATH,
    gabia::MODEL_PATH,
    hilde::MODEL_PATH,
    ifrit::MODEL_PATH,
    // jade::MODEL_PATH,
    jubee::MODEL_PATH,
    kidian::MODEL_PATH,
    kommy::MODEL_PATH,
    leets::MODEL_PATH,
    levi::MODEL_PATH,
    maestro_mk2::MODEL_PATH,
    marie::MODEL_PATH,
    mayo::MODEL_PATH,
    // meluna::MODEL_PATH,
    naia::MODEL_PATH,
    ner::MODEL_PATH,
    posher::MODEL_PATH,
    rim::MODEL_PATH,
    rohne::MODEL_PATH,
    rude::MODEL_PATH,
    rufo::MODEL_PATH,
    selline::MODEL_PATH,
    shady::MODEL_PATH,
    silphir::MODEL_PATH,
    sist::MODEL_PATH,
    speaki::MODEL_PATH,
    sylla::MODEL_PATH,
    tig::MODEL_PATH,
    ui::MODEL_PATH,
    // velvet::MODEL_PATH,
    vivi::MODEL_PATH,
    xion::MODEL_PATH,
];
