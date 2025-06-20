use crate::config;
use crate::config::Launcher;
use crate::config::migrate::old_structs;

impl From<old_structs::Config> for config::Config {
    fn from(value: old_structs::Config) -> Self {
        Self {
            layerrules: value.layerrules,
            kill_bind: value.kill_bind,
            windows: value
                .windows
                .map(|a| old_structs::Windows::into(a, value.launcher)),
        }
    }
}

impl old_structs::Windows {
    fn into(value: old_structs::Windows, launcher: Option<Launcher>) -> config::Windows {
        config::Windows {
            scale: value.scale,
            items_per_row: value.workspaces_per_row,
            switch: value.switch,
            overview: value.overview.map(|o| {
                old_structs::Overview::into(o, launcher, value.strip_html_from_workspace_title)
            }),
        }
    }
}

impl old_structs::Overview {
    fn into(
        value: old_structs::Overview,
        launcher: Option<Launcher>,
        strip_html_from_workspace_title: bool,
    ) -> config::Overview {
        config::Overview {
            open: value.open,
            other: value.other,
            navigate: value.navigate.into(),
            launcher: launcher.unwrap_or_default(),
            strip_html_from_workspace_title,
        }
    }
}
