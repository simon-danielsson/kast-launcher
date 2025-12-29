pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONF_FILE_DEF: &str = include_str!("static/configdef.toml");
pub const FALLBACK_FONT_BYTES: &[u8] = include_bytes!("static/0xProtoNerdFontMono.ttf");
pub const CONF_NOT_FOUND_TEXT: &str = "No config file was found. A new '.kast' with default settings has been created in your home directory.\n\n(Press 'x' to close this message)";
pub const CONF_NOT_FOUND_ICON: char = '';
