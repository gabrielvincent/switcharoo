use anyhow::Context;
use core_lib::util::get_boot_id;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// # Panics if time went backwards or no `boot_id` is available
pub fn create_storage_path(cache_dir: &Path, path: &str, ext: &str) -> anyhow::Result<PathBuf> {
    let now_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("Time went backwards?")?
        .as_millis();
    let get_boot_id = get_boot_id().clone().context("Failed to get boot_id")?;
    let path = cache_dir
        .to_path_buf()
        .join("clipboard")
        .join(path)
        .join(get_boot_id);
    fs::create_dir_all(&path).context("Failed to create storage directory")?;
    Ok(path.join(format!("{now_millis}.{ext}")))
}
