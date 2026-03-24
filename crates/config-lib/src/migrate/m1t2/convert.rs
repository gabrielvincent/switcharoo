use crate::migrate::m1t2::{NEXT_CONFIG_VERSION, old_structs};
use crate::migrate::{m2t3, m3t4};

impl From<old_structs::Config> for m2t3::Config {
    fn from(value: old_structs::Config) -> Self {
        Self {
            layerrules: value.layerrules,
            kill_bind: value.kill_bind,
            windows: value.windows.map(old_structs::Windows::into),
            version: Some(NEXT_CONFIG_VERSION),
        }
    }
}

impl From<old_structs::Windows> for m2t3::Windows {
    fn from(value: old_structs::Windows) -> Self {
        Self {
            scale: value.scale,
            items_per_row: value.items_per_row,
            switch: value.switch.map(old_structs::Switch::into),
        }
    }
}

impl From<old_structs::Switch> for m3t4::Switch {
    fn from(value: old_structs::Switch) -> Self {
        Self {
            filter_by: value.filter_by,
            modifier: value.modifier.into(),
            key: "tab".into(),
            switch_workspaces: value.show_workspaces,
            exclude_special_workspaces: "".into(),
        }
    }
}

impl From<old_structs::Modifier> for crate::Modifier {
    fn from(value: old_structs::Modifier) -> Self {
        match value {
            old_structs::Modifier::Alt => Self::Alt,
            old_structs::Modifier::Ctrl => Self::Ctrl,
            old_structs::Modifier::Shift | old_structs::Modifier::Super => Self::Super,
        }
    }
}
