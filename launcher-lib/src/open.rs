use crate::LauncherGlobal;
use gtk::glib;
use gtk::prelude::*;
use gtk4_layer_shell::{KeyboardMode, LayerShell};
use std::time::Duration;
use tracing::{span, trace, Level};

pub fn open_launcher(global: &LauncherGlobal) {
    let _span = span!(Level::TRACE, "open_launcher").entered();

    if let Some(data) = &global.data {
        let data = data.borrow();
        trace!("Showing window {:?}", data.window.id());
        data.window.set_monitor(None);
        data.window.set_visible(true);

        trace!("Resetting launcher data");
        data.entry.set_editable(true);
        data.entry.set_text("");

        data.window.set_keyboard_mode(KeyboardMode::OnDemand);
        data.entry.grab_focus();

        // focus the entry after a short delay to ensure that the window underneath is not focused
        let entry = data.entry.clone();
        let window = data.window.clone();
        glib::timeout_add_local_once(Duration::from_millis(50), move || {
            window.set_keyboard_mode(KeyboardMode::OnDemand);
            entry.grab_focus();
        });
    }
}
