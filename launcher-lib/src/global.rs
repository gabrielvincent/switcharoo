use core_lib::config::Plugins;
use core_lib::transfer::Identifier;
use gtk::{ApplicationWindow, Entry};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct LauncherData {
    pub config: LauncherConfig,
    pub window: ApplicationWindow,
    pub entry: Entry,
    pub results: gtk::Box,
    pub plugin_box: gtk::Box,
    pub sorted_matches: Vec<Identifier>,
    pub static_matches: HashMap<char, Identifier>,
}

#[derive(Debug)]
pub struct LauncherConfig {
    pub default_terminal: Option<Box<str>>,
    pub max_items: u8,
    pub show_when_empty: bool,
    pub animate_launch_ms: u64,
    pub width: u32,
    pub data_dir: Box<Path>,
    pub plugins: Plugins,
}
