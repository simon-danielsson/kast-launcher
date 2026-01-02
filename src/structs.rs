use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
        #[serde(rename = "Font")]
        pub font: Font,
        #[serde(rename = "Icons")]
        pub icons: Icons,
        #[serde(rename = "Colors")]
        pub colors: Colors,
        #[serde(rename = "Window")]
        pub window: Window,
        #[serde(rename = "Misc")]
        pub misc: Misc,
        #[serde(rename = "App")]
        pub apps: Vec<App>,
}

impl Default for Config {
        fn default() -> Self {
                Self {
                        font: Font::default(),
                        icons: Icons::default(),
                        colors: Colors::default(),
                        window: Window::default(),
                        misc: Misc::default(),
                        apps: vec![],
                }
        }
}

// ==== font ====

#[derive(Clone, Deserialize, Debug)]
pub struct Font {
        #[serde(default)]
        pub path: String,
        #[serde(default = "default_font_size")]
        pub size: f32,
}
fn default_font_size() -> f32 {
        12.0
}
impl Default for Font {
        fn default() -> Self {
                Self {
                        path: "".to_string(),
                        size: default_font_size(),
                }
        }
}

// ==== icons ====

#[derive(Deserialize, Debug)]
pub struct Icons {
        #[serde(default = "default_entry")]
        pub entry: char,
        #[serde(default)]
        pub entry_icon: bool,
}

fn default_entry() -> char {
        ''
}

impl Default for Icons {
        fn default() -> Self {
                Self {
                        entry: default_entry(),
                        entry_icon: false,
                }
        }
}

// ==== colors ====

#[derive(Clone, Deserialize, Debug)]
pub struct Colors {
        #[serde(default)]
        pub text: String,
        #[serde(default)]
        pub text_aux: String,
        #[serde(default)]
        pub accent: String,
        #[serde(default)]
        pub background: String,
}
impl Default for Colors {
        fn default() -> Self {
                Self {
                        text: "#ffffff".to_string(),
                        text_aux: "#888888".to_string(),
                        accent: "#333333".to_string(),
                        background: "1a1a1a".to_string(),
                }
        }
}

// ==== window ====

#[derive(Clone, Deserialize, Debug)]
pub struct Window {
        #[serde(default = "default_width")]
        pub width: u32,
        #[serde(default = "default_height")]
        pub height: u32,
        #[serde(default = "default_cnr_rad")]
        pub elem_cnr_rad: u8,
        #[serde(default = "default_row_height")]
        pub row_height: f32,
}

fn default_row_height() -> f32 {
        30.0
}
fn default_cnr_rad() -> u8 {
        5
}
fn default_height() -> u32 {
        250
}
fn default_width() -> u32 {
        250
}

impl Default for Window {
        fn default() -> Self {
                Self {
                        width: default_width(),
                        height: default_height(),
                        elem_cnr_rad: default_cnr_rad(),
                        row_height: default_row_height(),
                }
        }
}

// ==== misc ====

#[derive(Clone, Deserialize, Debug)]
pub struct Misc {
        #[serde(default)]
        pub search_hint: String,
        #[serde(default = "default_timeout")]
        pub timeout: u64,
}
fn default_timeout() -> u64 {
        20
}
impl Default for Misc {
        fn default() -> Self {
                Self {
                        search_hint: "Search...".to_string(),
                        timeout: default_timeout(),
                }
        }
}

// ==== app ====

#[derive(Clone, PartialEq, Eq, Deserialize, Debug)]
pub struct App {
        #[serde(default)]
        pub name: String,
        #[serde(default = "default_icon")]
        pub icon: char,
        #[serde(default)]
        pub path: String,
        #[serde(default)]
        pub group: String,
}

fn default_icon() -> char {
        'x'
}

impl Default for App {
        fn default() -> Self {
                Self {
                        name: "x".to_string(),
                        icon: default_icon(),
                        path: "x".to_string(),
                        group: "".to_string(),
                }
        }
}
