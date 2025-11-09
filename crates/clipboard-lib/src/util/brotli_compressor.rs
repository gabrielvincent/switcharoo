use brotli::CompressorWriter;

use brotli::enc::BrotliEncoderParams;
use std::io::Write;
use tracing::warn;

pub struct BrotliCompressWriter<W: Write> {
    encoder: CompressorWriter<W>,
}

impl<W: Write> BrotliCompressWriter<W> {
    pub fn new(writer: W, mut level: u8) -> Self {
        // use 6 as default compression level
        if level > 11 {
            warn!("Brotli compression level out of range, clamping to 11");
            level = 11;
        }
        let params = BrotliEncoderParams::default();
        Self {
            #[allow(clippy::cast_sign_loss)]
            encoder: CompressorWriter::new(writer, 4096, u32::from(level), params.lgwin as u32),
        }
    }
}

impl<W: Write> Write for BrotliCompressWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.encoder.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.encoder.flush()
    }
}
