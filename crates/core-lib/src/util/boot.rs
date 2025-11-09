use anyhow::Context;
use std::fs::File;
use std::io::Read;
use std::sync::OnceLock;
use tracing::{instrument, warn};

pub fn get_boot_id() -> &'static Option<String> {
    static BOOT_ID: OnceLock<Option<String>> = OnceLock::new();
    BOOT_ID.get_or_init(|| {
        load_boot_id().map_or_else(
            |e| {
                warn!("Failed to load boot ID: {e}");
                None
            },
            Some,
        )
    })
}

#[instrument(level = "debug", ret(level = "trace"))]
fn load_boot_id() -> anyhow::Result<String> {
    let mut file = File::open("/proc/sys/kernel/random/boot_id")
        .context("Failed to open /proc/sys/kernel/random/boot_id")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("Failed to read boot_id")?;
    Ok(contents.trim().to_string())
}
