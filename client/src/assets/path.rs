use const_format::concatcp;

#[rustfmt::skip] pub const QUERY: &str = "?";
#[rustfmt::skip] pub const VERSION: &str = concat!("v=", env!("CARGO_PKG_VERSION_PATCH"));

#[rustfmt::skip] pub const FONT_PATH_NOTOSANS_BOLD: &str = concatcp!("fonts/NotoSans-Bold.otf", QUERY, VERSION);
