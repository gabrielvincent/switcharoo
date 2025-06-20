use crate::global::WindowsSwitchData;
use core_lib::{ClientId, FindByFirst, Warn};
use exec_lib::switch::switch_client;
use exec_lib::{reset_remain_focused, to_client_address};
use gtk::glib;
use gtk::prelude::*;
use tracing::{Level, debug, span, trace};

pub fn close_switch(data: &mut WindowsSwitchData, ids: Option<Option<ClientId>>) {
    let _span = span!(Level::TRACE, "close_switch").entered();

    reset_remain_focused().warn("Failed to reset follow mouse");
    while let Some(child) = data.clients_flow.first_child() {
        data.clients_flow.remove(&child);
    }
    trace!("Hiding window (windows) {:?}", data.window.id());
    data.window.set_visible(false);

    if let Some(client_id) = ids {
        if let Some(client_id) = client_id.or(data.active.client) {
            debug!(
                "Switching to client {}",
                data.hypr_data
                    .clients
                    .find_by_first(&client_id)
                    .map(|c| c.title.clone())
                    .unwrap_or_else(|| "<Unknown>".to_string())
            );
            // we need to do this because the window might still be visible and have KeyboardMode::Exclusive
            glib::idle_add_local(move || {
                switch_client(to_client_address(client_id))
                    .warn(&format!("Failed to execute with id {client_id:?}"));
                glib::ControlFlow::Break
            });
        }
    }
}
