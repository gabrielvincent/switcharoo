use crate::WindowsGlobal;
use crate::global::{WindowsOverviewData, WindowsSwitchData};
use crate::next::find_next;
use core_lib::transfer::{Direction, SwitchSwitchConfig};
use gtk::prelude::*;
use tracing::{Level, span};

pub fn update_switch(data: &mut WindowsSwitchData, config: SwitchSwitchConfig) {
    let _span = span!(Level::TRACE, "update_switch").entered();

    let active = find_next(
        &if config.reverse {
            Direction::Left
        } else {
            Direction::Right
        },
        false,
        &data.hypr_data,
        data.active,
        0,
    );
    data.active = active;

    for (_, overlay) in data.clients.iter_mut() {
        overlay.remove_css_class("active");
    }
    for (id, overlay) in data.clients.iter_mut() {
        overlay.remove_css_class("active");
        if active.client == Some(*id) {
            overlay.add_css_class("active");
        }
    }
}
