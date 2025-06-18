use crate::config;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(SmartDefault, Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    #[default = true]
    pub layerrules: bool,
    #[default = "ctrl+shift+alt, h"]
    pub kill_bind: String,
    #[default(None)]
    pub launcher: Option<config::Launcher>,
    #[default(Some(Windows::default()))]
    pub windows: Option<Windows>,
}

#[derive(SmartDefault, Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Windows {
    #[default = 8.5]
    pub scale: f64,
    #[default = 5]
    pub workspaces_per_row: u8,
    #[default = true]
    pub strip_html_from_workspace_title: bool,
    pub overview: Option<Overview>,
    pub switch: Option<config::Switch>,
}

#[derive(SmartDefault, Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Overview {
    pub open: config::OpenOverview,
    pub navigate: config::Navigate,
    pub other: config::OtherOverview,
}
