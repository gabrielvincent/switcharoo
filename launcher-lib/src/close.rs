use crate::global::LauncherGlobalData;
use crate::{plugins, LauncherGlobal};
use gtk::glib;
use gtk::prelude::*;
use gtk4_layer_shell::{KeyboardMode, LayerShell};
use std::cell::RefCell;
use std::time::Duration;
use tracing::{span, trace, warn, Level};

pub fn close_launcher(global: &LauncherGlobal, char: Option<char>) {
    let _span = span!(Level::TRACE, "close_launcher").entered();

    if let Some(data) = &global.data {
        if let Some(char) = char {
            trace!("Closing launcher with char: {}", char);

            let data1 = data.borrow();
            if let Some(r#match) = match char {
                '0'..='9' => data1
                    .sorted_matches
                    .get(char.to_digit(10).expect("unable to convert char") as usize),
                _ => data1.static_matches.get(&char),
            } {
                data1.window.set_keyboard_mode(KeyboardMode::None);
                let animate = plugins::launch(
                    r#match,
                    &data1.entry.text(),
                    &global.default_terminal,
                    &global.data_dir,
                );
                // copy pointer for later close
                let window = data1.window.clone();
                let entry = data1.entry.clone();
                drop(data1);

                if animate {
                    // show_launch(data);
                    entry.set_editable(false);
                    glib::timeout_add_local_once(Duration::from_millis(400), move || {
                        // close launcher
                        close(&entry, &window);
                    });
                    return;
                }
            } else {
                warn!("No match found for char: {}", char);
            }
        }
        // close launcher
        let data1 = data.borrow();
        close(&data1.entry, &data1.window);
    }
}

fn close(entry: &gtk::Entry, window: &gtk::ApplicationWindow) {
    trace!("Hiding window {:?}", window.id());
    window.set_visible(false);
    entry.set_text("");
    // while let Some(child) = data.results.first_child() {
    //     data.results.remove(&child);
    // }
}

/// no longer used, but would look cool when launching apps
#[allow(dead_code)]
fn show_launch(data: &RefCell<LauncherGlobalData>, offset: u8) {
    let _span = span!(Level::TRACE, "show_launch").entered();

    let data = data.borrow();

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
