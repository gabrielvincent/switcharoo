use crate::LauncherData;
use gtk::prelude::*;
use tracing::{Level, span, trace};

pub fn stop_launcher(data: &LauncherData) {
    let _span = span!(Level::TRACE, "stop_launcher").entered();

    trace!("Closing window {:?}", data.window.id());
    data.window.close();
}
