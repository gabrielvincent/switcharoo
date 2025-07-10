use crate::util;
use core_lib::theme_icon_cache::get_all_icons;
use std::path::Path;
use tracing::debug;

pub fn check_class(class: Option<String>) {
    util::init_gtk();
    util::fill_icon_name_map(false);
    let desktop_files = core_lib::collect_desktop_files();
    windows_lib::reload_desktop_map(&desktop_files);
    debug!("prepared desktop files and icon map");

    if let Some(class) = class {
        debug!("searching for {class}");
        check_icon(&class);
    } else {
        debug!("no class provided, iterating over all clients");
        for client in exec_lib::get_clients() {
            let class = client.class;
            debug!("checking {class}");
            check_icon(&class);
        }
    }
}

fn check_icon(class: &str) {
    let in_theme = core_lib::theme_icon_cache::theme_has_icon_name(class);
    println!(
        "Icon ({class}) {} in theme (first choice)",
        if in_theme { "is" } else { "is not" }
    );
    let icon = windows_lib::get_icon_name_by_name_from_desktop_files(class);
    println!(
        "Icon ({class}) {} in desktop files (second choice) {}",
        if icon.is_some() { "is" } else { "is not" },
        if let Some(icon) = icon {
            format!("{:?} [icon: {:?}]", icon.2, icon.0)
        } else {
            "".to_string()
        }
    );
}

pub fn list_icons() {
    util::init_gtk();
    util::fill_icon_name_map(false);
    let icons = get_all_icons();
    for icon in icons.iter() {
        println!("{}", icon);
    }
}

pub fn list_desktop_files() {
    let desktop_files = core_lib::collect_desktop_files();
    for file in desktop_files.iter() {
        println!("{}", file.path().display());
    }
}

pub fn search(text: &str, all: bool, config_path: &Path, data_dir: &Path) {
    let (plugins, max_items) = config_lib::load_and_migrate_config(config_path)
        .ok()
        .and_then(|c| c.windows)
        .and_then(|w| w.overview)
        .map(|o| (o.launcher.plugins, o.launcher.max_items))
        .unwrap_or((
            config_lib::Plugins {
                applications: Default::default(),
                shell: None,
                terminal: None,
                websearch: None,
                calc: None,
                path: None,
            },
            5,
        ));
    launcher_lib::debug::get_matches(&plugins, text, all, max_items, data_dir);
}
