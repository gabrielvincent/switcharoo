use crate::LauncherData;
use gtk::glib;
use gtk::prelude::*;
use gtk4_layer_shell::{KeyboardMode, LayerShell};
use std::time::Duration;
use tracing::{Level, span, trace};

pub fn open_launcher(data: &LauncherData) {
    let _span = span!(Level::TRACE, "open_launcher").entered();

    trace!("Showing window {:?}", data.window.id());
    data.window.set_monitor(None);
    data.window.set_visible(true);

    trace!("Resetting launcher data");
    data.entry.set_editable(true);
    data.entry.set_text("");

    data.window.set_keyboard_mode(KeyboardMode::OnDemand);
    data.window.grab_focus();
    data.entry.grab_focus();

    let window = data.window.clone();
    let entry = data.entry.clone();
    glib::timeout_add_local_once(Duration::from_millis(100), move || {
        window.grab_focus();
        entry.grab_focus();
    });
}
