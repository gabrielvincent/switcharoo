#![allow(dead_code)]

#[derive(Debug, Copy, Clone)]
pub struct Config {
    pub encryption: Encryption,
    pub compression: Compression,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Encryption {
    #[default]
    None,
    #[cfg(feature = "encrypt_chacha20poly1305")]
    ChaCha20Poly1305,
    #[cfg(feature = "encrypt_aes_gcm")]
    AesGcm,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Compression {
    #[default]
    None,
    #[cfg(feature = "compress_lz4")]
    Lz4,
    #[cfg(feature = "compress_zstd")]
    Zstd(u8),
}
