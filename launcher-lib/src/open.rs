use crate::LauncherData;
use gtk::prelude::*;
use gtk4_layer_shell::{KeyboardMode, LayerShell};
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
}
