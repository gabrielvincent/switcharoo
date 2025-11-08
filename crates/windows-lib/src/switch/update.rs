use crate::global::WindowsSwitchData;
use crate::next::{find_next_client, find_next_workspace};
use adw::gtk::prelude::*;
use core_lib::transfer::{Direction, SwitchSwitchConfig};
use tracing::debug_span;

pub fn update_switch(data: &mut WindowsSwitchData, config: &SwitchSwitchConfig) {
    let _span = debug_span!("update_switch").entered();

    let dir = if config.reverse {
        Direction::Left
    } else {
        Direction::Right
    };
    let active = if data.config.switch_workspaces {
        find_next_workspace(
            &dir,
            true,
            &data.hypr_data,
            data.active,
            data.config.items_per_row,
        )
    } else {
        find_next_client(
            &dir,
            true,
            &data.hypr_data,
            data.active,
            data.config.items_per_row,
        )
    };
    data.active = active;

    if data.config.switch_workspaces {
        for button in data.clients.values() {
            button.remove_css_class("active");
        }
        for (id, button) in &data.workspaces {
            button.remove_css_class("active");
            if active.workspace == *id {
                button.add_css_class("active");
            }
        }
    } else {
        for button in data.workspaces.values() {
            button.remove_css_class("active");
        }
        for (id, button) in &data.clients {
            button.remove_css_class("active");
            if active.client == Some(*id) {
                button.add_css_class("active");
            }
        }
    }
}
