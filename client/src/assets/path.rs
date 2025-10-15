use const_format::concatcp;

#[rustfmt::skip] pub const QUERY: &str = "?";
#[rustfmt::skip] pub const VERSION: &str = concat!("v=", env!("CARGO_PKG_VERSION_PATCH"));

#[rustfmt::skip] pub const CONFIG_PATH: &str = concatcp!("config.json", QUERY, VERSION);
#[rustfmt::skip] pub const LOCALE_PATH_EN: &str = concatcp!("locale/en.json", QUERY, VERSION);
#[rustfmt::skip] pub const LOCALE_PATH_JA: &str = concatcp!("locale/ja.json", QUERY, VERSION);
#[rustfmt::skip] pub const LOCALE_PATH_KO: &str = concatcp!("locale/ko.json", QUERY, VERSION);

#[rustfmt::skip] pub const FONT_PATH: &str = concatcp!("fonts/NotoSans-Bold.otf", QUERY, VERSION);

#[rustfmt::skip] pub const MODEL_PATH_BUTTER: &str = concatcp!("models/butter/Butter.model", QUERY, VERSION);
#[rustfmt::skip] pub const MODEL_PATH_KOMMY: &str = concatcp!("models/kommy/Kommy.model", QUERY, VERSION);

#[rustfmt::skip] pub const COLLIDER_PATH_BUTTER: &str = concatcp!("models/butter/Butter.collider", QUERY, VERSION);
#[rustfmt::skip] pub const COLLIDER_PATH_KOMMY: &str = concatcp!("models/kommy/Kommy.collider", QUERY, VERSION);

#[rustfmt::skip] pub const IMG_PATH_BACKGROUND: &str = concatcp!("textures/Background.texture", QUERY, VERSION);
