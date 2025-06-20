use core_lib::transfer::Identifier;
use gtk::{ApplicationWindow, Entry};
use std::collections::HashMap;

#[derive(Debug)]
pub struct LauncherData {
    pub window: ApplicationWindow,
    pub entry: Entry,
    pub results: gtk::Box,
    pub plugin_box: gtk::Box,
    pub sorted_matches: Vec<Identifier>,
    pub static_matches: HashMap<char, Identifier>,
}
