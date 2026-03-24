use crate::migrate::m3t4;
use serde::Deserialize;
use smart_default::SmartDefault;

#[derive(SmartDefault, Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    #[default = true]
    pub layerrules: bool,
    #[default = "ctrl+shift+alt, h"]
    pub kill_bind: String,
    #[default(None)]
    pub windows: Option<Windows>,
    #[allow(dead_code)]
    pub version: Option<u16>,
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct Windows {
    #[default = 8.5]
    pub scale: f64,
    #[default = 5]
    pub items_per_row: u8,
    #[default(None)]
    pub switch: Option<m3t4::Switch>,
}
