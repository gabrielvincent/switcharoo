use crate::store::util::get_storage_string;
use anyhow::{Context, bail};
use image::ImageReader;
use std::fs::File;
use std::io::Cursor;
use tracing::{debug, trace};

const IMAGE_HEIGHT: u32 = 150;

pub fn compress_and_store_image(pref_mime: &str, pref_data: &Vec<u8>) -> anyhow::Result<()> {
    let (format, ext) = match pref_mime {
        "image/png" => (image::ImageFormat::Png, "png"),
        "image/jpeg" | "image/jpg" => (image::ImageFormat::Jpeg, "jpg"),
        "image/gif" => (image::ImageFormat::Gif, "gif"),
        "image/webp" => (image::ImageFormat::WebP, "webp"),
        _ => bail!("Unsupported image MIME type: {pref_mime}"),
    };
    let now = std::time::SystemTime::now();
    let img2 = ImageReader::with_format(Cursor::new(pref_data), format).decode()?;
    debug!(
        "Loaded image in {:?}, Image size: {}x{}",
        now.elapsed()?,
        img2.width(),
        img2.height()
    );
    let now = std::time::SystemTime::now();
    let nw = (img2.width() as f32 * (IMAGE_HEIGHT as f32 / img2.height() as f32)) as u32;
    let img2 = img2.resize(nw, IMAGE_HEIGHT, image::imageops::FilterType::Nearest);
    trace!(
        "Resized image size: {}x{} in {:?}",
        img2.width(),
        img2.height(),
        now.elapsed()?
    );

    let storage_string =
        get_storage_string().context("Failed to get storage string for clipboard image")?;
    std::fs::create_dir_all("test-data/images").context("Failed to create image directory")?;
    let mut file = File::create(format!("test-data/images/{storage_string}.{ext}"))
        .context("Failed to create clipboard image file")?;
    img2.write_to(&mut file, format)?;
    debug!(
        "Wrote image to test-data/images/{storage_string} ({} bytes)",
        file.metadata().map(|m| m.len()).unwrap_or(0)
    );
    Ok(())
}
