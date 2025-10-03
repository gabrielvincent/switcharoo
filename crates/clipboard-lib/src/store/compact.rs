use crate::store::ClipboardDataType;
use std::collections::HashMap;
use tracing::trace;

pub fn compact(data: HashMap<String, Vec<u8>>) -> HashMap<Box<str>, ClipboardDataType> {
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
