use anyhow::Context;
use core_lib::util::get_boot_id;

pub fn get_storage_string() -> anyhow::Result<String> {
    let now_millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .context("Time went backwards")?
        .as_millis();
    let get_boot_id = get_boot_id().clone().context("Failed to get boot_id")?;
    Ok(format!("{}-{}", now_millis, get_boot_id))
}
