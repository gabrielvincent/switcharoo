use crate::store::ClipboardDataType;
use crate::store::compact::compact;
use crate::store::image::compress_and_store_image;
use crate::store::mime::{filer_mimes, get_preferred_mime};
use crate::store::rw::store_binary;
use crate::store::util::get_storage_string;
use anyhow::{Context, bail};
use core_lib::util::get_boot_id;
use image::ImageReader;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Cursor, Write};
use tracing::{debug, trace, warn};
use wl_clipboard::paste::{CallbackData, Seat, get_all_contents_callback};

/// # Panics
pub fn test_clipboard() {
    let handle = get_all_contents_callback(Seat::Unspecified, Box::new(handle_values)).unwrap();
    let _ = handle.join();
}

fn handle_values(val: CallbackData) -> bool {
    let Ok((mut mimes, load)) = val else {
        return false;
    };

    filer_mimes(&mut mimes);

    let pref_mime = get_preferred_mime(&mimes);
    // load data for all mime types
    let mut data = HashMap::new();
    for mime in &mimes {
        data.insert(mime.clone(), load(mime.clone()).unwrap());
    }

    match pref_mime {
        Some(pref_mime) if pref_mime.starts_with("image/") => {
            let pref_data = data.get(&pref_mime).unwrap();
            let _ = compress_and_store_image(&pref_mime, pref_data);
            // TODO
        }
        Some(pref_mime) => {
            let pref_data = data.get(&pref_mime).unwrap();
            debug!("Data: {:?}", String::from_utf8_lossy(&pref_data));
            // TODO
        }
        None => {
            warn!("No preferred MIME type found, available: {mimes:?}");
        }
    }

    let combined_size = data.values().map(Vec::len).sum::<usize>();
    let data = compact(data);
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
    debug!(
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

    if let Err(err) = store_binary(&data) {
        warn!("Failed to store clipboard data: {err}");
    }
    false
}
