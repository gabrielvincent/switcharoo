use std::io::Write;
use tracing::warn;
use zstd::Encoder;

pub struct ZstdCompressWriter<'a, W: Write> {
    encoder: Encoder<'a, W>,
}

impl<W: Write> ZstdCompressWriter<'_, W> {
    pub fn new(writer: W, mut level: u8) -> Self {
        // use 16 as default compression level
        if level > 22 {
            warn!("Zstd compression level out of range, clamping to 22");
            level = 22;
        }
        Self {
            encoder: Encoder::new(writer, i32::from(level)).expect("Failed to create encoder"),
        }
    }
}

impl<W: Write> Write for ZstdCompressWriter<'_, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.encoder.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.encoder.flush()
    }
}

impl<W: Write> Drop for ZstdCompressWriter<'_, W> {
    fn drop(&mut self) {
        if let Err(err) = self.encoder.do_finish() {
            warn!("Failed to finish compressor: {err:?}");
        }
    }
}
