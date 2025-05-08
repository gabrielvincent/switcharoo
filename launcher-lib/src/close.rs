use crate::plugins::Identifier;
use crate::util::DataInWidget;
use crate::{plugins, LauncherGlobal};
use gtk::prelude::*;
use gtk::{glib, Widget};
use gtk4_layer_shell::{KeyboardMode, LayerShell};
use std::time::Duration;
use tracing::{span, trace, warn, Level};

pub fn close_launcher(global: &LauncherGlobal, char: Option<char>) {
    let _span = span!(Level::TRACE, "close_launcher").entered();

    if let Some(data) = &global.data {
        if let Some(char) = char {
            trace!("Closing launcher with char: {}", char);

            let data1 = data.borrow();
            if let Some(iden) = match char {
                '0'..='9' => data1
                    .sorted_matches
                    .get(char.to_digit(10).expect("unable to convert char") as usize),
                _ => data1.static_matches.get(&char),
            } {
                data1.window.set_keyboard_mode(KeyboardMode::None);
                let animate = plugins::launch(
                    iden,
                    &data1.entry.text(),
                    &global.default_terminal,
                    &global.data_dir,
                );
                // copy pointer for later close
                let window = data1.window.clone();
                let entry = data1.entry.clone();
                let results = data1.results.clone();
                let plugin_box = data1.plugin_box.clone();
                let iden = iden.clone();
                drop(data1);

                if animate {
                    show_launch(results, plugin_box, &iden);
                    entry.set_editable(false);
                    glib::timeout_add_local_once(
                        Duration::from_millis(global.animate_launch_ms),
                        move || {
                            // close launcher
                            close(&entry, &window);
                        },
                    );
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
}

fn show_launch(results: gtk::ListBox, plugin_box: gtk::Box, iden: &Identifier) {
    for child in results.observe_children().into_iter().flatten() {
        if let Some(child) = child.dynamic_cast_ref::<Widget>() {
            // trace!("A Child: {:?}, {:?}", child.get_iden_data(), iden.str());
            if let Some(data) = child.get_iden_data() {
                if data == iden.str() {
                    child.add_css_class("launch");
                    return;
                }
            }
        }
    }
    for child in plugin_box.observe_children().into_iter().flatten() {
        if let Some(child) = child.dynamic_cast_ref::<Widget>() {
            // trace!("B Child: {:?}, {:?}", child.get_iden_data(), iden.str());
            if let Some(data) = child.get_iden_data() {
                if data == iden.str() {
                    child.add_css_class("launch");
                    return;
                }
            }
        }
    }
}
