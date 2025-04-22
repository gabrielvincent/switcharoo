use crate::data::save_run;
use crate::global::LauncherGlobalData;
use crate::r#match::get_matches;
use crate::run::run_program;
use crate::LauncherGlobal;
use core_lib::Warn;
use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk4_layer_shell::{KeyboardMode, LayerShell};
use std::cell::RefCell;
use std::path::Path;
use std::time::Duration;
use tracing::{span, trace, Level};

pub fn close_launcher(offset: Option<u8>, global: &LauncherGlobal, data_dir: &Path) {
    let _span = span!(Level::TRACE, "close_launcher").entered();

    if let Some(data) = &global.data {
        if let Some(offset) = offset {
            trace!("Closing launcher with offset: {}", offset);
            let matches = {
                let data1 = data.borrow();
                let matches = get_matches(
                    &data1.entry.text(),
                    global.max_items as usize,
                    global.run_cache_weeks,
                    global.show_shell,
                    data_dir,
                );
                drop(data1);
                matches
            };
            if let Some((_, _match)) = matches.get((offset as usize).min(matches.len() - 1)) {
                let data1 = data.borrow();
                // hyprland autofocuses windows if they are created, however, if any window
                // exists with keyboard mode exclusive, it will not focus the new window
                data1.window.set_keyboard_mode(KeyboardMode::None);
                let results = data1.results.clone();
                let entry = data1.entry.clone();
                let window = data1.window.clone();
                drop(data1);

                show_launch(data, offset);
                run_program(
                    &_match.exec,
                    &_match.exec_path,
                    _match.terminal,
                    &global.default_terminal,
                );
                if let Some(source) = &_match.source {
                    trace!("Saving run: {:?}", source);
                    save_run(source, data_dir).warn("Failed to cache run");
                }

                let time = global.animate_launch_time_ms;
                glib::spawn_future_local(clone!(async move {
                    glib::timeout_future(Duration::from_millis(time)).await;
                    while let Some(child) = results.first_child() {
                        results.remove(&child);
                    }
                    entry.set_text("");
                    trace!("Hiding window {:?}", window.id());
                    window.set_visible(false);
                }));
            } else {
                let data1 = data.borrow();
                while let Some(child) = data1.results.first_child() {
                    data1.results.remove(&child);
                }
                data1.entry.set_text("");
                trace!("Hiding window {:?}", data1.window.id());
                data1.window.set_visible(false);
            }
        } else {
            let data1 = data.borrow();
            while let Some(child) = data1.results.first_child() {
                data1.results.remove(&child);
            }
            data1.entry.set_text("");
            trace!("Hiding window {:?}", data1.window.id());
            data1.window.set_visible(false);
        }
    }
}

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
