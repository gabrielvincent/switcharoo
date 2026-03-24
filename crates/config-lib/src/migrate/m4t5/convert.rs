use crate::migrate::m4t5::{NEXT_CONFIG_VERSION, old_structs};
use crate::migrate::m3t4;

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
        }
    }
}

impl From<old_structs::Switch> for crate::Switch {
    fn from(value: old_structs::Switch) -> Self {
        let exclude = if !value.exclude_workspaces.is_empty() {
            value.exclude_workspaces
        } else {
            value.exclude_special_workspaces
        };
        Self {
            key: value.key,
            modifier: value.modifier,
            filter_by: value.filter_by,
            switch_workspaces: value.switch_workspaces,
            exclude_workspaces: exclude,
            show_workspace_number: true,
            kill_key: value.kill_key,
        }
    }
}

// Chaining from m3t4
impl From<m3t4::Config> for old_structs::Config {
    fn from(value: m3t4::Config) -> Self {
        Self {
            version: 4,
            windows: value.windows.map(Into::into),
        }
    }
}

impl From<m3t4::Windows> for old_structs::Windows {
    fn from(value: m3t4::Windows) -> Self {
        Self {
            scale: value.scale,
            items_per_row: value.items_per_row,
            switch: value.switch.map(Into::into),
            switch_2: value.switch_2.map(Into::into),
        }
    }
}

impl From<m3t4::Switch> for old_structs::Switch {
    fn from(value: m3t4::Switch) -> Self {
        Self {
            modifier: value.modifier,
            key: value.key,
            filter_by: value.filter_by,
            switch_workspaces: value.switch_workspaces,
            exclude_special_workspaces: value.exclude_special_workspaces,
            exclude_workspaces: "".into(),
            kill_key: 'q',
        }
    }
}
