use crate::cache::save_run;
use crate::create::get_matches;
use crate::global::LauncherGlobalData;
use crate::run::run_program;
use crate::LauncherGlobal;
use core_lib::Warn;
use gtk::glib;
use gtk::prelude::{ApplicationWindowExt, EditableExt, WidgetExt};
use std::cell::RefCell;
use std::path::Path;
use std::time::Duration;
use tracing::{span, trace, Level};

pub async fn close_launcher(offset: Option<u8>, global: &LauncherGlobal, data_dir: &Path) {
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
                    data_dir,
                );
                drop(data1);
                matches
            };
            if let Some((_, _match)) = matches.get(offset as usize) {
                show_launch(data, offset);
                run_program(
                    &_match.exec,
                    &_match.exec_path,
                    _match.terminal,
                    &global.default_terminal,
                );
                save_run(&_match.path, data_dir).warn("Failed to cache run");
                glib::timeout_future(Duration::from_millis(global.animate_launch_time_ms)).await;
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
