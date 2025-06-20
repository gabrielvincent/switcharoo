use crate::plugins::iden_to_str;
use crate::util::DataInWidget;
use crate::{LauncherData, plugins};
use core_lib::transfer::Identifier;
use gtk::prelude::*;
use gtk::{Widget, glib};
use gtk4_layer_shell::{KeyboardMode, LayerShell};
use std::time::{Duration, Instant};
use tracing::{Level, span, trace, warn};

pub fn close_launcher_by_char(data: &mut LauncherData, char: Option<char>) {
    let _span = span!(Level::TRACE, "close_launcher_by_char").entered();
    if let Some(char) = char {
        data.window.set_keyboard_mode(KeyboardMode::None);
        trace!("Closing launcher with char: {}", char);
        if let Some(iden) = match char {
            '0'..='9' => data
                .sorted_matches
                .get(char.to_digit(10).expect("unable to convert char") as usize),
            _ => data.static_matches.get(&char),
        }
        .cloned()
        {
            close_launcher(data, &iden)
        } else {
            warn!("No match found for char: {}", char);
        }
    }
    close_window(&data.entry, &data.window);
}

pub fn close_launcher_by_iden(data: &mut LauncherData, iden: &Identifier) {
    let _span = span!(Level::TRACE, "close_launcher_by_iden").entered();
    data.window.set_keyboard_mode(KeyboardMode::None);
    trace!("Closing launcher with iden: {:?}", iden);

    close_launcher(data, iden);
    close_window(&data.entry, &data.window);
}

fn close_launcher(data: &mut LauncherData, iden: &Identifier) {
    let _span = span!(Level::TRACE, "close_launcher").entered();
    let instant = Instant::now();

    let animate = plugins::launch(
        iden,
        &data.entry.text(),
        &data.config.default_terminal,
        &data.config.data_dir,
    );
    // copy pointer for later close
    let window = data.window.clone();
    let entry = data.entry.clone();
    let results = data.results.clone();
    let plugin_box = data.plugin_box.clone();
    let iden = iden.clone();

    if animate {
        trace!(
            "starting timeout({}ms) animation after {:?} time",
            data.config.animate_launch_ms,
            instant.elapsed()
        );
        show_launch(&results, &plugin_box, &iden);
        entry.set_editable(false);
        glib::timeout_add_local_once(
            Duration::from_millis(data.config.animate_launch_ms),
            move || {
                let _span = _span.clone();
                // close launcher
                close_window(&entry, &window);
                hide_launch(&results, &plugin_box);
                trace!("closed launcher after {:?} time", instant.elapsed());
            },
        );
    }
}

fn close_window(entry: &gtk::Entry, window: &gtk::ApplicationWindow) {
    trace!("Hiding window (launcher) {:?}", window.id());
    window.set_visible(false);
    entry.set_text("");
}

fn show_launch(results: &gtk::Box, plugin_box: &gtk::Box, iden: &Identifier) {
    for child in results.observe_children().into_iter().flatten() {
        if let Some(child) = child.dynamic_cast_ref::<Widget>() {
            // trace!("A Child: {:?}, {:?}", child.get_iden_data(), iden_to_str(iden));
            if let Some(data) = child.get_iden_data() {
                if data == iden_to_str(iden) {
                    child.add_css_class("launch");
                    return;
                }
            }
        }
    }
    for child in plugin_box.observe_children().into_iter().flatten() {
        if let Some(child) = child.dynamic_cast_ref::<Widget>() {
            // trace!("B Child: {:?}, {:?}", child.get_iden_data(), iden_to_str(iden));
            if let Some(data) = child.get_iden_data() {
                if data == iden_to_str(iden) {
                    child.add_css_class("launch");
                    return;
                }
            }
        }
    }
}

fn hide_launch(results: &gtk::Box, plugin_box: &gtk::Box) {
    for child in results.observe_children().into_iter().flatten() {
        if let Some(child) = child.dynamic_cast_ref::<Widget>() {
            child.remove_css_class("launch");
        }
    }
    for child in plugin_box.observe_children().into_iter().flatten() {
        if let Some(child) = child.dynamic_cast_ref::<Widget>() {
            child.remove_css_class("launch");
        }
    }
}
