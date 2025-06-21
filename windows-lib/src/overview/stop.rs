use crate::global::WindowsOverviewData;
use core_lib::WarnWithDetails;
use exec_lib::reset_remain_focused;
use gtk::prelude::*;
use tracing::{Level, span, trace};

pub fn stop_overview(data: &WindowsOverviewData) {
    let _span = span!(Level::TRACE, "stop_overview").entered();
    reset_remain_focused().warn("Failed to reset follow mouse");
    for (window, _) in data.window_list.iter() {
        trace!("Closing window {:?}", window.id());
        window.close();
    }
}
