use crate::migrate::m2t3::NEXT_CONFIG_VERSION;
use crate::migrate::m3t4;

impl From<crate::migrate::m2t3::old_structs::Config> for m3t4::Config {
    fn from(value: crate::migrate::m2t3::old_structs::Config) -> Self {
        Self {
            windows: value.windows.map(crate::migrate::m2t3::old_structs::Windows::into),
            version: NEXT_CONFIG_VERSION,
        }
    }
}

impl From<crate::migrate::m2t3::old_structs::Windows> for m3t4::Windows {
    fn from(value: crate::migrate::m2t3::old_structs::Windows) -> Self {
        Self {
            scale: value.scale,
            items_per_row: value.items_per_row,
            switch: value.switch,
            switch_2: None,
        }
    }
}
