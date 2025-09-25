use std::collections::{HashMap, HashSet};
use tracing::{debug, trace};
use wl_clipboard::paste::{Seat, get_all_contents_channel};
use wl_clipboard::utils::is_text;

/// # Panics
pub fn test_clipboard() {
    let rx = get_all_contents_channel(Seat::Unspecified, Some(Box::new(filer_mime))).unwrap();
    loop {
        if let Ok(Ok(data)) = rx.recv() {
            if let Some((mime, data)) = get_prefered_mime(&data) {
                debug!("Prefered MIME type: {mime}");
                if !mime.starts_with("image/") {
                    debug!("Data: {:?}", String::from_utf8_lossy(&data));
                }
            } else {
                debug!("No data available");
            }
            for (mime_type, data) in &data {
                trace!(
                    "Got data of the {} MIME type, size {}",
                    mime_type,
                    data.len()
                );
            }
            let combined_size = data.values().map(Vec::len).sum::<usize>();
            debug!("Combined size: {combined_size} bytes");
        }
    }
}

fn filer_mime(mime_types: &mut HashSet<String>) {
    let count = mime_types.len();

    // remove audio
    mime_types.retain(|mt| !mt.starts_with("audio/"));

    // Retain only one image/ MIME type: prefer jpg, then png, then jpeg, then first found
    let mut image_mime: Option<String> = None;
    for preferred in ["image/jpg", "image/jpeg", "image/png"] {
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

    let mut charsets = HashMap::new();
    mime_types.retain(|mt| {
        mt.find(";charset=").is_none_or(|pos| {
            let (mime, charset) = mt.split_at(pos);
            let charset = &charset[9..];
            charsets
                .entry(mime.to_string())
                .or_insert(vec![])
                .push(charset.to_ascii_lowercase());
            false
        })
    });

    // add all the mimes with charset back in
    for (charset, mimes) in charsets {
        if mimes.contains(&"utf-8".to_string()) {
            mime_types.insert(format!("{charset};charset=utf-8"));
        } else if mimes.contains(&"utf-16".to_string()) {
            mime_types.insert(format!("{charset};charset=utf-16"));
        } else {
            // add back all (idk why no utf-8 or utf-16, so add all)
            for cs in mimes {
                mime_types.insert(format!("{charset};charset={cs}"));
            }
        }
    }

    trace!(
        "Available MIME types: {:#?}, removed {} charset elements",
        mime_types,
        count - mime_types.len()
    );
}

fn get_prefered_mime(mime_types: &HashMap<String, Vec<u8>>) -> Option<(String, Vec<u8>)> {
    // Priority order
    let priority = [
        "image/*",
        "text/plain;charset=utf-8",
        "UTF8_STRING",
        "text/plain",
    ];

    // Find by priority
    for (index, mime) in priority.iter().enumerate() {
        if mime.ends_with("/*") {
            let prefix = &mime[..mime.len() - 1];
            if let Some((mt, data)) = mime_types.iter().find(|(x, _)| x.starts_with(prefix)) {
                trace!("Chosen MIME type: {mt:?} from prio({index}), by prefix {prefix}*");
                return Some((mt.clone(), data.clone()));
            }
        } else if let Some((mt, data)) = mime_types.iter().find(|(x, _)| x == mime) {
            trace!("Chosen MIME type: {mt:?} from prio({index}), by exact match {mime}");
            return Some((mt.clone(), data.clone()));
        }
    }

    // Fallback: first text
    if let Some((mt, data)) = mime_types.iter().find(|(x, _)| is_text(x)) {
        trace!("Chosen MIME type: {mt:?} from text fallback");
        return Some((mt.clone(), data.clone()));
    }
    // choose any
    if let Some((mt, data)) = mime_types.iter().next() {
        trace!("Chosen MIME type: {mt:?} from fallback",);
        return Some((mt.clone(), data.clone()));
    }
    None
}
