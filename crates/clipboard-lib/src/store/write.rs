use crate::config::{Compression, Config, Encryption};
use std::io::Write;

pub fn get_storage_writer<'a, I: Write + 'a>(
    writer: I,
    config: &Config,
    compress: bool,
) -> (Box<dyn Write + 'a>, &'static str) {
    let base: Box<dyn Write> = match config.encryption {
        Encryption::None => Box::new(writer),
        #[cfg(any(feature = "encrypt_chacha20poly1305", feature = "encrypt_aes_gcm"))]
        _ => {
            let config_val = match config.encryption {
                #[cfg(feature = "encrypt_chacha20poly1305")]
                Encryption::ChaCha20Poly1305 => crate::util::crypt::Config::ChaCha20Poly1305,
                #[cfg(feature = "encrypt_aes_gcm")]
                Encryption::AesGcm => crate::util::crypt::Config::AesGcm,
                _ => unreachable!(),
            };
            match crate::util::secret_service::get_hyprshell_key() {
                Ok(key) => Box::new(crate::util::crypt::SecretEncryptWriter::new(
                    writer, key, config_val,
                )),
                Err(err) => {
                    tracing::warn!("Failed to load/generate new encryption key: {err:?}");
                    Box::new(writer)
                }
            }
        }
    };
    match (compress, config.compression) {
        (false, _) | (true, Compression::None) => (base, "raw"),
        #[cfg(feature = "compress_lz4")]
        (true, Compression::Lz4) => (
            Box::new(crate::util::lz4_compressor::LZ4CompressWriter::new(base)),
            "lz4",
        ),
        #[cfg(feature = "compress_brotli")]
        (true, Compression::Brotli(level)) => (
            Box::new(crate::util::brotli_compressor::BrotliCompressWriter::new(
                base, level,
            )),
            "br",
        ),
        #[cfg(feature = "compress_zstd")]
        (true, Compression::Zstd(level)) => (
            Box::new(crate::util::zstd_compressor::ZstdCompressWriter::new(
                base, level,
            )),
            "zstd",
        ),
    }
}
