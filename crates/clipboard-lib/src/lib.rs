pub mod store;
pub(crate) mod util;

#[cfg(all(feature = "compress_zstd", feature = "compress_lz4"))]
compile_error!("Features `compress_zstd` and `compress_lz4` cannot be enabled at the same time.");

#[cfg(all(feature = "encrypt_chacha20poly1305", feature = "encrypt_aes_gcm"))]
compile_error!(
    "Features `encrypt_chacha20poly1305` and `encrypt_aes_gcm` cannot be enabled at the same time."
);
