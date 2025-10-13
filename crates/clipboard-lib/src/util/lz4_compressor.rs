use lz4_flex::frame::FrameEncoder;
use std::io::Write;
use tracing::warn;

pub struct LZ4CompressWriter<W: Write> {
    encoder: FrameEncoder<W>,
}

impl<W: Write> LZ4CompressWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            encoder: FrameEncoder::new(writer),
        }
    }
}

impl<W: Write> Write for LZ4CompressWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.encoder.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.encoder.flush()
    }
}

impl<W: Write> Drop for LZ4CompressWriter<W> {
    fn drop(&mut self) {
        if let Err(err) = self.encoder.try_finish() {
            warn!("Failed to finish compressor: {err:?}");
        }
    }
}
