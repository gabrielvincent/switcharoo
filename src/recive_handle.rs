use crate::receive::Globals;
use core_lib::transfer::{CloseConfig, OpenOverview, OpenSwitch, SwitchConfig, WindowsOverride};
use core_lib::{IdOverride, Warn};
use gtk::prelude::EntryExt;
use tracing::warn;

macro_rules! if_some {
    ($global:expr, $func:path $(, $args:expr)*) => {
        if let Some(_gl) = &$global {
            $func(_gl $(, $args)*)
        }
    };
}
pub fn open_overview(global: &Globals, config: OpenOverview) {
    if let Some(global) = &global.window {
        windows_lib::open_overview(global, config).warn("Failed to open overview window");
    } else {
        warn!("Window overview not active");
    };
    if_some!(&global.launcher, launcher_lib::open_launcher);
}

pub fn open_switch(global: &Globals, config: OpenSwitch) {
    if let Some(global) = &global.window {
        windows_lib::open_switch(global, config).warn("Failed to open switch window");
    } else {
        warn!("Window switch not active");
    }
}

pub fn switch(global: &Globals, config: SwitchConfig) {
    if global.launcher.is_some() {
        let launch = global
            .launcher
            .as_ref()
            .and_then(|l| {
                l.data.as_ref().map(|d| {
                    let b = d.borrow();
                    b.entry.text_length() > 0
                })
            })
            .unwrap_or(false);
        // don't switch selected window if launcher is active
        if !launch {
            if_some!(&global.window, windows_lib::update_overview, config)
        }
    } else {
        if_some!(&global.window, windows_lib::update_overview, config)
    }
}
pub fn exit(global: &Globals) {
    if_some!(global.window, windows_lib::close_overview, None);
    if_some!(global.launcher, launcher_lib::close_launcher, None);
}

pub fn r#type(global: &Globals, text: String) {
    if_some!(global.launcher, launcher_lib::update_launcher, text);
}

pub fn close(global: &Globals, config: CloseConfig) {
    match config {
        CloseConfig::None => {
            let launch = global
                .launcher
                .as_ref()
                .and_then(|l| {
                    l.data.as_ref().map(|d| {
                        let b = d.borrow();
                        b.entry.text_length() > 0
                    })
                })
                .unwrap_or(false);
            if launch {
                // kill overview, close launcher
                if_some!(global.window, windows_lib::close_overview, None);
                if_some!(global.launcher, launcher_lib::close_launcher, Some('0'));
            } else {
                // close overview, kill launcher
                if_some!(global.window, windows_lib::close_overview, Some(None));
                if_some!(global.launcher, launcher_lib::close_launcher, None);
            };
        }
        CloseConfig::Launcher(key) => {
            // kill overview, close launcher
            if_some!(global.window, windows_lib::close_overview, None);
            if_some!(global.launcher, launcher_lib::close_launcher, Some(key));
        }
        CloseConfig::Windows(WindowsOverride::ClientId(client_id)) => {
            // close overview, kill launcher
            if_some!(
                global.window,
                windows_lib::close_overview,
                Some(Some(IdOverride::ClientId(client_id)))
            );
            if_some!(global.launcher, launcher_lib::close_launcher, None);
        }
        CloseConfig::Windows(WindowsOverride::WorkspaceID(workspace_id)) => {
            // close overview, kill launcher
            if_some!(
                global.window,
                windows_lib::close_overview,
                Some(Some(IdOverride::WorkspaceID(workspace_id)))
            );
            if_some!(global.launcher, launcher_lib::close_launcher, None);
        }
    }
}

pub fn restart(global: &Globals) {
    if_some!(&global.window, windows_lib::stop_overview);
    if_some!(&global.launcher, launcher_lib::stop_launcher);
}
