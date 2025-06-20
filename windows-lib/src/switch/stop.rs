use crate::global::WindowsSwitchData;
use core_lib::Warn;
use exec_lib::reset_remain_focused;
use gtk::prelude::*;
use tracing::{Level, span, trace};

pub fn stop_switch(data: &WindowsSwitchData) {
    let _span = span!(Level::TRACE, "stop_switch").entered();
    reset_remain_focused().warn("Failed to reset follow mouse");
    trace!("Closing window {:?}", data.window.id());
    data.window.close();
}
