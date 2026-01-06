use crate::migrate::m3t4::{NEXT_CONFIG_VERSION, old_structs};

impl From<old_structs::Config> for crate::Config {
    fn from(value: old_structs::Config) -> Self {
        Self {
            windows: value.windows.map(old_structs::Windows::into),
            version: NEXT_CONFIG_VERSION,
        }
    }
}

impl From<old_structs::Windows> for crate::Windows {
    fn from(value: old_structs::Windows) -> Self {
        Self {
            scale: value.scale,
            items_per_row: value.items_per_row,
            switch: value.switch.map(old_structs::Switch::into),
            switch_2: value.switch_2.map(old_structs::Switch::into),
            overview: value.overview.map(old_structs::Overview::into),
        }
    }
}

impl From<old_structs::Overview> for crate::Overview {
    fn from(value: old_structs::Overview) -> Self {
        Self {
            key: value.key,
            modifier: value.modifier,
            filter_by: value.filter_by,
            launcher: value.launcher.into(),
            exclude_workspaces: value.exclude_special_workspaces,
        }
    }
}

impl From<old_structs::Switch> for crate::Switch {
    fn from(value: old_structs::Switch) -> Self {
        Self {
            key: value.key,
            modifier: value.modifier,
            filter_by: value.filter_by,
            switch_workspaces: value.switch_workspaces,
            exclude_workspaces: value.exclude_special_workspaces,
            kill_key: 'q',
        }
    }
}
