use crate::global::{WindowsOverviewConfig, WindowsOverviewData, WindowsOverviewMonitorData};
use anyhow::Context;
use core_lib::config::{Overview, Windows};
use core_lib::{HyprlandData, OVERVIEW_NAMESPACE};
use exec_lib::{get_initial_active, get_monitors};
use gtk::gdk::{Display, Monitor};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, FlowBox, Orientation, Overlay, SelectionMode};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use launcher_lib::LauncherData;
use std::collections::HashMap;
use tracing::{Level, debug, span};

pub fn create_windows_overview_window(
    app: &Application,
    overview: &Overview,
    windows: &Windows,
    launcher_data: LauncherData,
) -> anyhow::Result<WindowsOverviewData> {
    let _span = span!(Level::TRACE, "create_windows_overview_window").entered();
    let mut window_list = HashMap::new();

    let monitors = get_monitors();
    if let Ok(display) = Display::default().context("Could not connect to a display") {
        let gtk_monitors = display
            .monitors()
            .iter()
            .filter_map(|m| m.ok())
            .collect::<Vec<Monitor>>();

        for gtk_monitor in gtk_monitors {
            let monitor_name = gtk_monitor.connector().unwrap_or_default();
            if let Some(monitor) = monitors.iter().find(|m| m.name == monitor_name) {
                let workspaces_flow = FlowBox::builder()
                    .selection_mode(SelectionMode::None)
                    .orientation(Orientation::Horizontal)
                    .max_children_per_line(windows.items_per_row as u32)
                    .min_children_per_line(windows.items_per_row as u32)
                    .build();

                let workspaces_flow_overlay = Overlay::builder()
                    .child(&workspaces_flow)
                    .css_classes(["monitor"])
                    .build();

                let window = ApplicationWindow::builder()
                    .css_classes(["window"])
                    .application(app)
                    .child(&workspaces_flow_overlay)
                    .default_height(10)
                    .default_width(10)
                    .build();

                window.init_layer_shell();
                window.set_namespace(Some(OVERVIEW_NAMESPACE));
                window.set_layer(Layer::Top);
                window.set_anchor(Edge::Bottom, true);
                window.set_margin(Edge::Bottom, (monitor.height as f32 / 2.6) as i32);
                window.set_keyboard_mode(KeyboardMode::OnDemand);
                window.set_monitor(Some(&gtk_monitor));
                window.present();
                window.set_visible(false);

                debug!(
                    "Created overview window ({}) for monitor {:?}",
                    window.id(),
                    monitor_name
                );
                window_list.insert(
                    window,
                    WindowsOverviewMonitorData::new(monitor.id, workspaces_flow),
                );
            }
        }
    }
    Ok(WindowsOverviewData {
        config: WindowsOverviewConfig {
            items_per_row: windows.items_per_row,
            scale: windows.scale,
            strip_html_from_workspace_title: overview.strip_html_from_workspace_title,
        },
        window_list,
        active: get_initial_active()?,
        hypr_data: HyprlandData::default(),
        launcher: launcher_data,
    })
}
