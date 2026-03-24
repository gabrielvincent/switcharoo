use serde::Deserialize;
use smart_default::SmartDefault;
use std::fmt::Display;

#[derive(SmartDefault, Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    #[default = true]
    pub(super) layerrules: bool,
    #[default = "ctrl+shift+alt, h"]
    pub(super) kill_bind: String,
    #[default(None)]
    pub(super) windows: Option<Windows>,
    #[allow(dead_code)]
    pub(super) version: u16,
}

#[derive(SmartDefault, Debug, Clone, Deserialize)]
#[serde(default)]
pub(super) struct Windows {
    #[default = 8.5]
    pub(super) scale: f64,
    #[default = 5]
    pub(super) items_per_row: u8,
    #[default(None)]
    pub(super) switch: Option<Switch>,
}

#[derive(SmartDefault, Debug, Clone, Deserialize)]
#[serde(default)]
pub(super) struct Switch {
    #[default(Modifier::Alt)]
    pub modifier: Modifier,
    #[default(Vec::new())]
    pub filter_by: Vec<crate::FilterBy>,
    #[default = false]
    pub show_workspaces: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Modifier {
    Alt,
    Ctrl,
    Super,
    Shift,
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Alt => write!(f, "alt"),
            Self::Ctrl => write!(f, "ctrl"),
            Self::Super => write!(f, "super"),
            Self::Shift => write!(f, "shift"),
        }
    }
}
