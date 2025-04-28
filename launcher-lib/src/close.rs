use crate::global::LauncherGlobalData;
use crate::{plugins, LauncherGlobal};
use gtk::prelude::*;
use gtk4_layer_shell::{KeyboardMode, LayerShell};
use std::cell::RefCell;
use tracing::{span, trace, warn, Level};

pub fn close_launcher(global: &LauncherGlobal, char: Option<char>) {
    let _span = span!(Level::TRACE, "close_launcher").entered();

    if let Some(data) = &global.data {
        if let Some(char) = char {
            trace!("Closing launcher with char: {}", char);

            let data1 = data.borrow();
            if let Some(r#match) = match char {
                '0'..'9' => data1
                    .sorted_matches
                    .get(char.to_digit(10).expect("unable to convert char") as usize),
                char => data1.static_matches.get(&char),
            } {
                data1.window.set_keyboard_mode(KeyboardMode::None);
                // show_launch(data, offset);
                plugins::launch(
                    r#match,
                    &data1.entry.text().to_string(),
                    &global.default_terminal,
                    &global.data_dir,
                );
            } else {
                warn!("No match found for char: {}", char);
            }
        }
        let data1 = data.borrow();
        while let Some(child) = data1.results.first_child() {
            data1.results.remove(&child);
        }
        data1.entry.set_text("");
        trace!("Hiding window {:?}", data1.window.id());
        data1.window.set_visible(false);
    }
}

/// no longer used, but would look cool when launching apps
fn show_launch(data: &RefCell<LauncherGlobalData>, offset: u8) {
    let _span = span!(Level::TRACE, "show_launch").entered();

    let data = data.borrow();
    data.entry.set_editable(false);

    let mut i = 0;
    while let Some(child) = data.results.row_at_index(i) {
        if i == offset as i32 {
            child.add_css_class("launch");
        } else {
            child.add_css_class("monochrome");
        }
        i += 1;
    }
}
