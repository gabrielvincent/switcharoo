#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Config {
    pub encryption: Encryption,
    pub compression: Compression,
    pub image_conv_filter: ConvolutionFilterType,
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
    #[cfg(feature = "compress_brotli")]
    Brotli(u8),
}

#[derive(Debug, Copy, Clone, Default)]
pub enum ConvolutionFilterType {
    Box,
    Bilinear,
    Hamming,
    CatmullRom,
    Mitchell,
    Gaussian,
    #[default]
    Lanczos3,
}

impl From<ConvolutionFilterType> for fast_image_resize::FilterType {
    fn from(value: ConvolutionFilterType) -> Self {
        match value {
            ConvolutionFilterType::Box => Self::Box,
            ConvolutionFilterType::Bilinear => Self::Bilinear,
            ConvolutionFilterType::Hamming => Self::Hamming,
            ConvolutionFilterType::CatmullRom => Self::CatmullRom,
            ConvolutionFilterType::Mitchell => Self::Mitchell,
            ConvolutionFilterType::Gaussian => Self::Gaussian,
            ConvolutionFilterType::Lanczos3 => Self::Lanczos3,
        }
    }
}
