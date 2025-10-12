use crate::store::util::get_current_storage_string;
use crate::store::write::get_storage_writer;
use anyhow::Context;
use std::fs;
use std::fs::File;
use std::io::Write;
use tracing::trace;

pub fn store_text(text: &str) -> anyhow::Result<()> {
    let compress = text.len() > 100;
    let storage_string =
        get_current_storage_string().context("Failed to get storage string for clipboard image")?;
    fs::create_dir_all("test-data/text").context("Failed to create text directory")?;
    let mut file = File::create(format!(
        "test-data/text/{storage_string}.{}",
        if compress { "txt.lz4" } else { "txt" }
    ))
    .context("Failed to create clipboard image file")?;
    {
        let mut write = get_storage_writer(&mut file, compress, true);
        write
            .write_all(text.as_bytes())
            .context("Failed to write text to clipboard")?;
    }
    trace!(
        "Wrote text to test-data/text/{storage_string} ({} bytes)",
        file.metadata().map(|m| m.len()).unwrap_or(0)
    );
    Ok(())
}
