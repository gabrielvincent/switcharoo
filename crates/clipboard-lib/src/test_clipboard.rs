use anyhow::Context;
use bincode;
use image::ImageReader;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufWriter, Cursor, Read, Write};
use std::sync::OnceLock;
use tracing::{debug, trace};
use wl_clipboard::paste::{Data, Seat, get_all_contents_callback};
use wl_clipboard::utils::is_text;

/// # Panics
pub fn test_clipboard() {
    let handle = get_all_contents_callback(Seat::Unspecified, Box::new(handle_values)).unwrap();
    handle.join();
}

fn handle_values(val: Data) -> bool {
    let Ok((mut mimes, load)) = val else {
        return false;
    };
    let Some(pref_mime) = get_preferred_mime(&mimes) else {
        return false;
    };
    debug!("Preferred MIME type: {pref_mime}");
    let pref_data = load(pref_mime.clone()).unwrap();
    filer_mime(&mut mimes);
    let mut data = HashMap::new();
    for mime in mimes {
        data.insert(mime.clone(), load(mime).unwrap());
    }

    let now_millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let get_boot_id = get_boot_id();

    if !pref_mime.starts_with("image/") {
        debug!("Data: {:?}", String::from_utf8_lossy(&pref_data));
    } else {
        let (format, ext) = match &*pref_mime {
            "image/png" => (image::ImageFormat::Png, "png"),
            "image/jpeg" | "image/jpg" => (image::ImageFormat::Jpeg, "jpg"),
            "image/gif" => (image::ImageFormat::Gif, "gif"),
            "image/webp" => (image::ImageFormat::WebP, "webp"),
            _ => (image::ImageFormat::Png, "png"), // fallback
        };
        let now = std::time::SystemTime::now();
        let img2 = ImageReader::with_format(Cursor::new(pref_data), format)
            .decode()
            .unwrap();
        debug!(
            "Loaded image in {:?}, Image size: {}x{}",
            now.elapsed().unwrap(),
            img2.width(),
            img2.height()
        );
        let now = std::time::SystemTime::now();
        let nw = (img2.width() as f32 * (200.0 / img2.height() as f32)) as u32;
        let img2 = img2.resize(nw, 200, image::imageops::FilterType::Nearest);
        trace!(
            "Resized image size: {}x{} in {:?}",
            img2.width(),
            img2.height(),
            now.elapsed().unwrap()
        );
        if let Ok(mut file) =
            File::create(format!("test-data/images/{now_millis}-{get_boot_id}.{ext}"))
        {
            img2.write_to(&mut file, format).unwrap();
            debug!(
                "Wrote image to test-data/images/{now_millis}-{get_boot_id} ({} bytes)",
                file.metadata().map(|m| m.len()).unwrap_or(0)
            );
        }
    }

    let combined_size = data.values().map(Vec::len).sum::<usize>();

    // don't try to compress anything with image, would date longer, and the image would still be the vast majority of size
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
    if let Ok(mut file) = File::create(format!("test-data/data/{now_millis}-{get_boot_id}.bin")) {
        let vec = bincode::encode_to_vec(&data, bincode::config::standard()).unwrap();
        let mut writer = BufWriter::new(&mut file);
        writer.write_all(&vec).unwrap();
        writer.flush().unwrap();
        debug!(
            "Wrote clipboard data to clipboard_data.bin ({} bytes)",
            vec.len()
        );
    }
    false
}

#[derive(Debug, bincode::Encode)]
enum ClipboardDataType {
    Alias(Box<str>),
    Data(Vec<u8>),
}

fn compact(data: HashMap<String, Vec<u8>>) -> HashMap<Box<str>, ClipboardDataType> {
    let time = std::time::Instant::now();
    let mut dedupted = 0u16;
    let mut map: HashMap<Box<str>, ClipboardDataType> = HashMap::new();
    'outer: for (mime, data) in data {
        // don't try to dedup images, they are usually large and unique nad only one image/* is kept
        // changes that any image/* can be deduped with any other mime type is very rare, so it's not worth the effort
        if !mime.starts_with("image/") {
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
    map
}

fn get_boot_id() -> String {
    static BOOT_ID: OnceLock<anyhow::Result<String>> = OnceLock::new();
    BOOT_ID
        .get_or_init(load_boot_id)
        .as_deref()
        .unwrap_or("unknown")
        .to_string()
}
fn load_boot_id() -> anyhow::Result<String> {
    let mut file = File::open("/proc/sys/kernel/random/boot_id")
        .context("Failed to open /proc/sys/kernel/random/boot_id")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("Failed to read boot_id")?;
    let contents = contents.trim().to_string();
    trace!("Boot ID: {contents}");
    Ok(contents)
}

fn filer_mime(mime_types: &mut HashSet<String>) {
    let count = mime_types.len();

    // remove audio
    mime_types.retain(|mt| !mt.starts_with("audio/"));

    // Retain only one image/ MIME type.
    let mut image_mime: Option<String> = None;
    for preferred in [
        "image/jpg",
        "image/jpeg",
        "image/png",
        "image/webp",
        "image/gif",
    ] {
        if mime_types.contains(&preferred.to_string()) {
            image_mime = Some(preferred.to_string());
            break;
        }
    }
    if image_mime.is_none() {
        if let Some(mt) = mime_types.iter().find(|mt| mt.starts_with("image/")) {
            image_mime = Some(mt.clone());
        }
    }
    mime_types.retain(|mt| {
        if mt.starts_with("image/") {
            image_mime.as_ref() == Some(mt)
        } else {
            true
        }
    });

    trace!(
        "Available MIME types: {:#?}, removed {} charset elements",
        mime_types,
        count - mime_types.len()
    );
}

fn get_preferred_mime(mime_types: &HashSet<String>) -> Option<String> {
    // Priority order
    let priority = [
        "image/webp",
        "image/jpg",
        "image/jpeg",
        "image/png",
        "image/gif",
        "text/plain;charset=utf-8",
        "UTF8_STRING",
        "STRING",
        "TEXT",
        "text/*",
    ];

    // Find by priority
    for (index, mime) in priority.iter().enumerate() {
        if mime.ends_with("/*") {
            let prefix = &mime[..mime.len() - 1];
            if let Some(mt) = mime_types.iter().find(|x| x.starts_with(prefix)) {
                trace!("Chosen MIME type: {mt:?} from prio({index}), by prefix {prefix}*");
                return Some(mt.clone());
            }
        } else if let Some(mt) = mime_types.iter().find(|x| x == mime) {
            trace!("Chosen MIME type: {mt:?} from prio({index}), by exact match {mime}");
            return Some(mt.clone());
        }
    }

    // Fallback: first text
    if let Some(mt) = mime_types.iter().find(|x| is_text(x)) {
        trace!("Chosen MIME type: {mt:?} from text fallback");
        return Some(mt.clone());
    }
    // choose any
    if let Some(mt) = mime_types.iter().next() {
        trace!("Chosen MIME type: {mt:?} from fallback",);
        return Some(mt.clone());
    }
    None
}
