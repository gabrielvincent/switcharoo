use crate::store::util::get_storage_string;
use anyhow::Context;
use lz4_flex::frame::FrameEncoder;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::time::SystemTime;
use tracing::trace;

#[derive(Debug, bincode::Encode)]
pub enum ClipboardDataType {
    Alias(Box<str>),
    Data(Vec<u8>),
}

pub fn store_binary(
    data: &HashMap<Box<str>, ClipboardDataType>,
    compress: bool,
) -> anyhow::Result<()> {
    let storage_string =
        get_storage_string().context("Failed to get storage string for clipboard data")?;
    std::fs::create_dir_all("test-data/data").context("Failed to create data directory")?;
    let name = format!("{storage_string}.{}", if compress { "lz4" } else { "bin" });
    let mut file = File::create(format!("test-data/data/{name}"))
        .context("Failed to create clipboard data file")?;
    let now = SystemTime::now();
    if compress {
        let mut compressor = FrameEncoder::new(&mut file);
        bincode::encode_into_std_write(data, &mut compressor, bincode::config::standard())
            .context("Failed to encode clipboard data")?;
        compressor
            .finish()
            .context("Failed to finish compressing clipboard data")?;
    } else {
        let vec = bincode::encode_to_vec(data, bincode::config::standard())
            .context("Failed to encode clipboard data")?;
        file.write_all(&vec)
            .context("Failed to write clipboard data to file")?;
    }
    trace!(
        "Wrote clipboard data to {} ({} bytes) in {:?}",
        name,
        file.metadata().map(|m| m.len()).unwrap_or(0),
        now.elapsed()?
    );
    Ok(())
}
