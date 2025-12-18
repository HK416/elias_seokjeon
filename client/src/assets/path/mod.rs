mod sound;

use bevy::platform::collections::HashMap;
use const_format::concatcp;
use lazy_static::lazy_static;
use protocol::{Hero, NUM_HEROS};
use sound::*;

pub trait HeroVoiceSet {
    fn all(&self) -> &[&'static str];

    fn defeat(&self) -> &[&'static str];

    fn ducth_rub_end(&self) -> &[&'static str];

    fn hit(&self) -> &[&'static str];

    fn shout(&self) -> &[&'static str];

    fn touch_1(&self) -> &[&'static str];

    fn touch_2(&self) -> &[&'static str];

    fn victory(&self) -> &[&'static str];
}

pub const HERO_VOICE_SETS: [&dyn HeroVoiceSet; NUM_HEROS] = [
    &aya::HeroVoice,
    &bigwood::HeroVoice,
    &butter::HeroVoice,
    &erpin::HeroVoice,
    &kidian::HeroVoice,
    &kommy::HeroVoice,
    &mayo::HeroVoice,
    &rohne::HeroVoice,
    &speaki::HeroVoice,
    &xion::HeroVoice,
];

#[rustfmt::skip] pub const QUERY: &str = "?";
#[rustfmt::skip] pub const VERSION: &str = concat!("v=", env!("CARGO_PKG_VERSION_PATCH"));

#[rustfmt::skip] pub const CONFIG_PATH: &str = concatcp!("config.json", QUERY, VERSION);
#[rustfmt::skip] pub const LOCALE_PATH_EN: &str = concatcp!("locale/en.json", QUERY, VERSION);
#[rustfmt::skip] pub const LOCALE_PATH_JA: &str = concatcp!("locale/ja.json", QUERY, VERSION);
#[rustfmt::skip] pub const LOCALE_PATH_KO: &str = concatcp!("locale/ko.json", QUERY, VERSION);

#[rustfmt::skip] pub const FONT_PATH: &str = concatcp!("fonts/NotoSans-Bold.otf", QUERY, VERSION);

#[rustfmt::skip] pub const MODEL_PATH_AYA: &str = concatcp!("models/aya/Aya.model", QUERY, VERSION);
#[rustfmt::skip] pub const MODEL_PATH_BIGWOOD: &str = concatcp!("models/bigwood/BigWood.model", QUERY, VERSION);
#[rustfmt::skip] pub const MODEL_PATH_BUTTER: &str = concatcp!("models/butter/Butter.model", QUERY, VERSION);
#[rustfmt::skip] pub const MODEL_PATH_ERPIN: &str = concatcp!("models/erpin/Erpin.model", QUERY, VERSION);
#[rustfmt::skip] pub const MODEL_PATH_KIDIAN: &str = concatcp!("models/kidian/Kidian.model", QUERY, VERSION);
#[rustfmt::skip] pub const MODEL_PATH_KOMMY: &str = concatcp!("models/kommy/Kommy.model", QUERY, VERSION);
#[rustfmt::skip] pub const MODEL_PATH_MAYO: &str = concatcp!("models/mayo/Mayo.model", QUERY, VERSION);
#[rustfmt::skip] pub const MODEL_PATH_ROHNE: &str = concatcp!("models/rohne/Rohne.model", QUERY, VERSION);
#[rustfmt::skip] pub const MODEL_PATH_SPEAKI: &str = concatcp!("models/speaki/Speaki.model", QUERY, VERSION);
#[rustfmt::skip] pub const MODEL_PATH_XION: &str = concatcp!("models/xion/xXionx.model", QUERY, VERSION);

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
#[rustfmt::skip] pub const VOC_PATH_ERPIN: &str = concatcp!("sounds/Erpin.sound", QUERY, VERSION);

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

lazy_static! {
    pub static ref MODEL_PATH_HEROS: HashMap<Hero, &'static str> = {
        let map = HashMap::from_iter([
            (Hero::Aya, MODEL_PATH_AYA),
            (Hero::BigWood, MODEL_PATH_BIGWOOD),
            (Hero::Butter, MODEL_PATH_BUTTER),
            (Hero::Erpin, MODEL_PATH_ERPIN),
            (Hero::Kidian, MODEL_PATH_KIDIAN),
            (Hero::Kommy, MODEL_PATH_KOMMY),
            (Hero::Mayo, MODEL_PATH_MAYO),
            (Hero::Rohne, MODEL_PATH_ROHNE),
            (Hero::Speaki, MODEL_PATH_SPEAKI),
            (Hero::Xion, MODEL_PATH_XION),
        ]);

        assert_eq!(map.len(), NUM_HEROS);
        map
    };
}
