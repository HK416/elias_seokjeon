use bevy::platform::collections::HashMap;
use const_format::concatcp;
use lazy_static::lazy_static;
use protocol::{Hero, NUM_HEROS};

#[rustfmt::skip] pub const QUERY: &str = "?";
#[rustfmt::skip] pub const VERSION: &str = concat!("v=", env!("CARGO_PKG_VERSION_PATCH"));

#[rustfmt::skip] pub const CONFIG_PATH: &str = concatcp!("config.json", QUERY, VERSION);
#[rustfmt::skip] pub const LOCALE_PATH_EN: &str = concatcp!("locale/en.json", QUERY, VERSION);
#[rustfmt::skip] pub const LOCALE_PATH_JA: &str = concatcp!("locale/ja.json", QUERY, VERSION);
#[rustfmt::skip] pub const LOCALE_PATH_KO: &str = concatcp!("locale/ko.json", QUERY, VERSION);

#[rustfmt::skip] pub const FONT_PATH: &str = concatcp!("fonts/NotoSans-Bold.otf", QUERY, VERSION);

#[rustfmt::skip] pub const MODEL_PATH_BUTTER: &str = concatcp!("models/butter/Butter.model", QUERY, VERSION);
#[rustfmt::skip] pub const MODEL_PATH_KOMMY: &str = concatcp!("models/kommy/Kommy.model", QUERY, VERSION);

#[rustfmt::skip] pub const IMG_PATH_BACKGROUND: &str = concatcp!("textures/Background.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_LABEL_DECO_0: &str = concatcp!("textures/Label_Deco_0.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_LABEL_DECO_1: &str = concatcp!("textures/Label_Deco_1.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_LABEL_DECO_2: &str = concatcp!("textures/Label_Deco_2.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_LOADING_DECO: &str = concatcp!("textures/Loading_Deco.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_PATTERN_0: &str = concatcp!("textures/Pattern_0.texture", QUERY, VERSION);
#[rustfmt::skip] pub const IMG_PATH_PVP_INGAME_VS: &str = concatcp!("textures/PVP_Ingame_VS.texture", QUERY, VERSION);

#[rustfmt::skip] pub const IMG_PATH_FX_FIRECARTOON: &str = concatcp!("textures/FX_Firecartoon.texture", QUERY, VERSION);
#[rustfmt::skip] pub const ATLAS_PATH_FX_FIRECARTOON: &str = concatcp!("textures/FX_Firecartoon.atlas", QUERY, VERSION);

#[rustfmt::skip] pub const IMG_PATH_LOADING_MINIMI: &str = concatcp!("textures/Loading_minimi.texture", QUERY, VERSION);
#[rustfmt::skip] pub const ATLAS_PATH_LOADING_MINIMI: &str = concatcp!("textures/Loading_minimi.atlas", QUERY, VERSION);

lazy_static! {
    pub static ref MODEL_PATH_HEROS: HashMap<Hero, &'static str> = {
        let mut map = HashMap::default();
        map.insert(Hero::Butter, MODEL_PATH_BUTTER);
        map.insert(Hero::Kommy, MODEL_PATH_KOMMY);

        assert_eq!(map.len(), NUM_HEROS);
        map
    };
}
