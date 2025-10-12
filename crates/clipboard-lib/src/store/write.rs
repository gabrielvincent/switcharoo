use std::io::Write;

pub fn get_storage_writer<'a, I: Write + 'a>(
    writer: I,
    _compress: bool,
    _encrypt: bool,
) -> Box<dyn Write + 'a> {
    #[cfg(any(feature = "encrypt_chacha20poly1305", feature = "encrypt_aes_gcm"))]
    let base: Box<dyn Write> = if _encrypt {
        match crate::util::secret_service::get_hyprshell_key() {
            Ok(key) => Box::new(crate::util::crypt::EncryptWriter::new(
                &key,
                Box::new(writer),
            )),
            Err(e) => {
                tracing::warn!("Failed to get hyprshell key, unable to encrypt: {e:?}");
                Box::new(writer)
            }
        }
    } else {
        Box::new(writer)
    };
    #[cfg(not(any(feature = "encrypt_chacha20poly1305", feature = "encrypt_aes_gcm")))]
    let base: Box<dyn Write> = Box::new(writer);

    #[cfg(any(feature = "compress_lz4", feature = "compress_zstd"))]
    {
        if _compress {
            return Box::new(crate::util::compress::CompressWriter::new(base));
        }
    }

    base
}
