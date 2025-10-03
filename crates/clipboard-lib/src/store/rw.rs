use crate::store::util::get_storage_string;
use anyhow::Context;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use tracing::debug;

#[derive(Debug, bincode::Encode)]
pub enum ClipboardDataType {
    Alias(Box<str>),
    Data(Vec<u8>),
}

pub fn store_binary(data: &HashMap<Box<str>, ClipboardDataType>) -> anyhow::Result<()> {
    let storage_string =
        get_storage_string().context("Failed to get storage string for clipboard data")?;
    std::fs::create_dir_all("test-data/data").context("Failed to create data directory")?;
    let mut file = File::create(format!("test-data/data/{storage_string}.bin"))
        .context("Failed to create clipboard data file")?;
    let vec = bincode::encode_to_vec(data, bincode::config::standard())
        .context("Failed to encode clipboard data")?;
    let mut writer = BufWriter::new(&mut file);
    writer
        .write_all(&vec)
        .context("Failed to write clipboard data to file")?;
    writer
        .flush()
        .context("Failed to flush clipboard data to file")?;
    debug!(
        "Wrote clipboard data to clipboard_data.bin ({} bytes)",
        vec.len()
    );
    Ok(())
}
