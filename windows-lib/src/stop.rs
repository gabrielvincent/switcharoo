use crate::WindowsGlobal;
use core_lib::Warn;
use exec_lib::{reset_remain_focused, reset_submap};
use gtk::prelude::*;
use tracing::{Level, span, trace};

pub fn stop_overview(global: &WindowsGlobal) {
    let _span = span!(Level::TRACE, "stop_overview").entered();
    reset_submap().warn("Failed to reset submap");
    reset_remain_focused().warn("Failed to reset follow mouse");
    let data = global.data.borrow();
    for (window, _) in data.monitor_list.iter() {
        trace!("Closing window {:?}", window.id());
        window.close();
    }
    drop(data);
}
