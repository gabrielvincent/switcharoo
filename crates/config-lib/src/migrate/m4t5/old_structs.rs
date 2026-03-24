use serde::Deserialize;
use smart_default::SmartDefault;

#[derive(SmartDefault, Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct Config {
    #[default(4)]
    pub version: u16,
    #[default(None)]
    pub windows: Option<Windows>,
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct Windows {
    #[default = 8.5]
    pub scale: f64,
    #[default = 5]
    pub items_per_row: u8,
    #[default(None)]
    pub switch: Option<Switch>,
    #[default(None)]
    pub switch_2: Option<Switch>,
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
#[allow(clippy::struct_field_names)]
pub struct Switch {
    #[default(crate::Modifier::Alt)]
    pub modifier: crate::Modifier,
    #[default = "Tab"]
    pub key: Box<str>,
    #[default(vec![crate::FilterBy::CurrentMonitor])]
    pub filter_by: Vec<crate::FilterBy>,
    #[default = false]
    pub switch_workspaces: bool,
    #[default = ""]
    pub exclude_workspaces: Box<str>,
    #[default = ""]
    pub exclude_special_workspaces: Box<str>,
    #[default = 'q']
    pub kill_key: char,
}
