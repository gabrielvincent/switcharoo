use crate::store::util::get_current_storage_string;
use anyhow::Context;
use image::{ImageEncoder, ImageReader};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Cursor};
use tracing::trace;

use fast_image_resize::images::Image;
use fast_image_resize::{FilterType, IntoImageView, ResizeAlg, ResizeOptions, Resizer};
use image::codecs::png::PngEncoder;

const IMAGE_HEIGHT: u32 = 150;

pub fn compress_and_store_image(pref_data: Vec<u8>) -> anyhow::Result<()> {
    let now = std::time::SystemTime::now();
    let img2 = ImageReader::new(Cursor::new(pref_data))
        .with_guessed_format()?
        .decode()?;
    trace!(
        "Loaded image in {:?}, Image size: {}x{}",
        now.elapsed()?,
        img2.width(),
        img2.height()
    );
    let now = std::time::SystemTime::now();

    #[allow(clippy::cast_sign_loss, clippy::cast_precision_loss)]
    let mut dst_image = Image::new(
        (img2.width() as f32 * (IMAGE_HEIGHT as f32 / img2.height() as f32)) as u32,
        IMAGE_HEIGHT,
        img2.pixel_type()
            .context("Failed to get pixel type for clipboard image")?,
    );

    let mut resizer = Resizer::new();
    resizer.resize(
        &img2,
        &mut dst_image,
        &ResizeOptions::new().resize_alg(ResizeAlg::Convolution(FilterType::Box)),
    )?;
    trace!(
        "Resized image size: {}x{} in {:?}",
        dst_image.width(),
        dst_image.height(),
        now.elapsed()?
    );

    let storage_string =
        get_current_storage_string().context("Failed to get storage string for clipboard image")?;
    fs::create_dir_all("test-data/images").context("Failed to create image directory")?;
    let mut file = File::create(format!("test-data/images/{storage_string}.png"))
        .context("Failed to create clipboard image file")?;
    {
        let mut result_buf = BufWriter::new(&mut file);
        PngEncoder::new(&mut result_buf).write_image(
            dst_image.buffer(),
            dst_image.width(),
            dst_image.height(),
            img2.color().into(),
        )?;
    }
    trace!(
        "Wrote image to test-data/images/{storage_string} ({} bytes)",
        file.metadata().map(|m| m.len()).unwrap_or(0)
    );
    Ok(())
}
