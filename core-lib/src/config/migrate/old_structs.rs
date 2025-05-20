use crate::config;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(SmartDefault, Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    #[default = true]
    pub layerrules: bool,
    #[default(None)]
    pub launcher: Option<Launcher>,
    #[default(Some(Windows::default()))]
    pub windows: Option<Windows>,
}

#[derive(SmartDefault, Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Windows {
    #[default = 6.5]
    pub size_factor: f64,
    #[default = 8.5]
    pub scale: f64,
    #[default = 5]
    pub workspaces_per_row: u8,
    #[default = true]
    pub strip_html_from_workspace_title: bool,
    pub overview: Option<config::Overview>,
    pub switch: Option<config::Switch>,
}

#[derive(SmartDefault, Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Launcher {
    #[default(None)]
    pub default_terminal: Option<Box<str>>,
    #[default = 650]
    pub width: u32,
    #[default = 5]
    pub max_items: u8,
    #[default = 250]
    pub animate_launch_ms: u64,
    #[default(vec![
        Plugin::Applications(Default::default()),
        Plugin::Calc(),
        Plugin::Terminal(),
        Plugin::WebSearch(Default::default()),
    ])]
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Plugin {
    Applications(config::ApplicationsPluginOptions),
    Terminal(),
    Shell(),
    WebSearch(Vec<SearchEngine>),
    Calc(),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchEngine {
    pub url: String,
    pub name: String,
    pub key: String,
}
