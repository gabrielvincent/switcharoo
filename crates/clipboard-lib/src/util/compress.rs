use std::io::Write;
use tracing::warn;

pub struct CompressWriter<'a, W: Write> {
    #[cfg(feature = "compress_lz4")]
    encoder: lz4_flex::frame::FrameEncoder<W>,
    #[cfg(feature = "compress_lz4")]
    phantom: std::marker::PhantomData<&'a W>,
    #[cfg(feature = "compress_zstd")]
    encoder: zstd::stream::Encoder<'a, W>,
}

impl<W: Write> CompressWriter<'_, W> {
    pub fn new(writer: W) -> Self {
        Self {
            #[cfg(feature = "compress_lz4")]
            encoder: lz4_flex::frame::FrameEncoder::new(writer),
            #[cfg(feature = "compress_lz4")]
            phantom: std::marker::PhantomData,
            #[cfg(feature = "compress_zstd")]
            encoder: zstd::stream::Encoder::new(writer, 16).expect("Failed to create encoder"),
        }
    }
}

impl<W: Write> Write for CompressWriter<'_, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.encoder.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.encoder.flush()
    }
}

impl<W: Write> Drop for CompressWriter<'_, W> {
    fn drop(&mut self) {
        #[cfg(feature = "compress_zstd")]
        if let Err(err) = self.encoder.do_finish() {
            warn!("Failed to finish compressor: {err:?}");
        }
        #[cfg(feature = "compress_lz4")]
        if let Err(err) = self.encoder.try_finish() {
            warn!("Failed to finish compressor: {err:?}");
        }
    }
}
