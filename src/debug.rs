use crate::start;
use core_lib::theme_icon_cache::get_all_icons;
use tracing::{debug, info};

pub fn search(class: Option<String>) {
    start::fill_icon_map();
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
    info!(
        "Icon ({class}) {} in theme (first choice)",
        if in_theme { "is" } else { "is not" }
    );
    let icon = windows_lib::get_icon_name_by_name_from_desktop_files(class);
    info!(
        "Icon ({class}) {} in desktop files (second choice)",
        if icon.is_some() { "is" } else { "is not" }
    );
}

pub fn list() {
    start::fill_icon_map();
    let icons = get_all_icons();
    for icon in icons.iter() {
        info!("{} [{}]", icon.0, icon.1.display());
    }
}

pub fn desktop_files() {
    let desktop_files = core_lib::collect_desktop_files();
    for file in desktop_files.iter() {
        info!("{}", file.path().display());
    }
}
