use core_lib::config::Launcher;
use gtk::{ApplicationWindow, Entry, ListBox};
use std::cell::RefCell;

#[derive(Debug)]
pub struct LauncherGlobal {
    pub default_terminal: Option<String>,
    pub animate_launch_time_ms: u64,
    pub show_execs: bool,
    pub max_items: u8,
    pub width: u32,
    pub run_cache_weeks: u8,
    pub show_shell: bool,
    pub data: Option<RefCell<LauncherGlobalData>>,
}

impl LauncherGlobal {
    pub fn new(config: &Launcher) -> Self {
        Self {
            default_terminal: config.default_terminal.clone(),
            animate_launch_time_ms: config.animate_launch_time_ms,
            show_execs: config.show_execs,
            max_items: config.max_items,
            width: config.width,
            run_cache_weeks: config.run_cache_weeks,
            show_shell: config.shell_commands,
            data: None,
        }
    }
}

#[derive(Debug)]
pub struct LauncherGlobalData {
    pub window: ApplicationWindow,
    pub entry: Entry,
    pub results: ListBox,
}
