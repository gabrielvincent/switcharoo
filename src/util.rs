use core_lib::WarnWithDetails;
use gtk::glib::ControlFlow;
use gtk::{IconTheme, Settings, glib};
use libc::SIGTERM;
use tracing::{info, trace, warn};

pub fn reload_desktop_data() {
    // reload the desktop maps for the launcher and overview
    let desktop_files = core_lib::collect_desktop_files();
    let mime_files = core_lib::collect_mime_files();
    windows_lib::reload_desktop_map(&desktop_files);
    launcher_lib::reload_applications_desktop_map(&desktop_files);
    launcher_lib::reload_search_default_browser(&desktop_files, &mime_files);
    launcher_lib::reload_path_default_file_manager(&desktop_files, &mime_files);
}

pub fn init_gtk() {
    gtk::init().expect("Failed to initialize GTK");
}

pub fn fill_icon_name_map(in_background: bool) {
    let icon_theme = IconTheme::new();
    let gtk_icons = icon_theme
        .icon_names()
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let search_path = icon_theme
        .search_path()
        .into_iter()
        .filter(|path| path.exists())
        .collect::<Vec<_>>();
    trace!("Icon theme search path: {search_path:?}");
    core_lib::theme_icon_cache::init_icon_name_map(gtk_icons, search_path, in_background);
}

pub fn gtk_handle_sigterm() {
    glib::unix_signal_add(SIGTERM, || {
        info!("Received SIGTERM, exiting gracefully");
        exec_lib::reset_remain_focused().warn_details("Failed to reset follow mouse on SIGTERM");
        // Continue with the default SIGTERM handler after cleanup
        unsafe {
            libc::signal(SIGTERM, libc::SIG_DFL);
            libc::raise(SIGTERM);
        }
        ControlFlow::Continue
    });
}

pub fn check_themes() {
    if let Some(settings) = Settings::default() {
        let theme_name = settings.gtk_theme_name();
        let icon_theme_name = settings.gtk_icon_theme_name();
        info!(
            "Using theme: {theme_name:?} and icon theme: {icon_theme_name:?}, please make sure both exist, else weird icon or graphical issues may occur"
        );
    } else {
        warn!("Unable to check default settings for icon theme");
    }
}
