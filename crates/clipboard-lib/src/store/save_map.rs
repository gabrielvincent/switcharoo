use crate::store::util::get_current_storage_string;
use crate::store::write::get_storage_writer;
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::time::SystemTime;
use tracing::{trace, warn};

#[derive(Debug, bincode::Encode)]
pub enum ClipboardDataType {
    Alias(Box<str>),
    Data(Vec<u8>),
}

pub fn compress_and_store_map(data: HashMap<String, Vec<u8>>) {
    let combined_size = data.values().map(Vec::len).sum::<usize>();
    let (data, contains_image) = deduplicate_clipboard_entries(data, true);
    let compressed_combined_size = data
        .values()
        .filter_map(|dt| {
            if let ClipboardDataType::Data(d) = dt {
                Some(d.len())
            } else {
                None
            }
        })
        .sum::<usize>();
    trace!(
        "Combined size: {} bytes, compressed size {} bytes, storing {} aliased and {} data entries",
        combined_size,
        compressed_combined_size,
        data.values()
            .filter(|dt| matches!(dt, ClipboardDataType::Alias(_)))
            .count(),
        data.values()
            .filter(|dt| matches!(dt, ClipboardDataType::Data(_)))
            .count()
    );

    // dont compress if contains image
    if let Err(err) = store_map(&data, !contains_image) {
        warn!("Failed to store clipboard data: {err}");
    }
}

fn store_map(data: &HashMap<Box<str>, ClipboardDataType>, compress: bool) -> anyhow::Result<()> {
    let storage_string =
        get_current_storage_string().context("Failed to get storage string for clipboard data")?;
    fs::create_dir_all("test-data/data").context("Failed to create data directory")?;
    let name = format!(
        "{storage_string}.{}",
        if compress { "bin.lz4" } else { "bin" }
    );
    let now = SystemTime::now();

    let mut file = File::create(format!("test-data/data/{name}"))
        .context("Failed to create clipboard data file")?;

    {
        let mut write = get_storage_writer(&mut file, compress, true);
        bincode::encode_into_std_write(data, &mut write, bincode::config::standard())
            .context("Failed to encode clipboard data")?;
    }

    trace!(
        "Wrote clipboard data to {} ({} bytes) in {:?}",
        name,
        file.metadata().map(|m| m.len()).unwrap_or(0),
        now.elapsed()?
    );
    Ok(())
}

pub fn deduplicate_clipboard_entries(
    data: HashMap<String, Vec<u8>>,
    dedup: bool,
) -> (HashMap<Box<str>, ClipboardDataType>, bool) {
    let time = std::time::Instant::now();
    let mut image_found = false;
    let mut dedupted = 0u16;
    let mut map: HashMap<Box<str>, ClipboardDataType> = HashMap::new();
    'outer: for (mime, data) in data {
        if mime.starts_with("image/") {
            image_found = true;
        }
        if dedup {
            for (f_mime, f_dt) in map.iter().filter(|(m, _)| !m.starts_with("image/")) {
                if let ClipboardDataType::Data(check_data) = f_dt {
                    if data.eq(check_data) {
                        trace!("Deduped MIME type {mime} to {f_mime}");
                        map.insert(
                            mime.into_boxed_str(),
                            ClipboardDataType::Alias(f_mime.clone()),
                        );
                        dedupted += 1;
                        continue 'outer;
                    }
                }
            }
        }
        map.insert(mime.into_boxed_str(), ClipboardDataType::Data(data));
    }
    trace!(
        "Deduplication took {:?}, dedupted {dedupted} entries",
        time.elapsed()
    );
    (map, image_found)
}
