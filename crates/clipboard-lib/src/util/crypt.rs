#[cfg(feature = "encrypt_aes_gcm")]
use aes_gcm::{AeadCore, Aes256Gcm as Algo, KeyInit, aead::Aead};

#[cfg(feature = "encrypt_chacha20poly1305")]
use chacha20poly1305::{AeadCore, ChaCha20Poly1305 as Algo, KeyInit, aead::Aead};

use anyhow::Context;
use std::io::Write;
use tracing::trace;

pub fn generate_new_cha_cha_key() -> anyhow::Result<Vec<u8>> {
    Algo::generate_key()
        .map_err(|_| anyhow::anyhow!("Failed to generate new encryption key"))
        .map(|k| k.to_vec())
}

fn generate_cypher(key: &[u8]) -> anyhow::Result<Algo> {
    Algo::new_from_slice(key).context("Failed to generate cypher with encryption key")
}

pub struct EncryptWriter<W: Write> {
    key: Vec<u8>,
    buffer: Vec<u8>,
    writer: W,
}

impl<W: Write> EncryptWriter<W> {
    pub fn new(key: &[u8], writer: W) -> Self {
        Self {
            key: key.to_vec(),
            buffer: Vec::new(),
            writer,
        }
    }

    pub fn encrypt(&self, cleartext: &[u8]) -> anyhow::Result<Vec<u8>> {
        trace!("length of cleartext: {}", cleartext.len());
        let cipher = generate_cypher(&self.key).context("Failed to generate cipher")?;
        let nonce =
            Algo::generate_nonce().map_err(|_| anyhow::anyhow!("Failed to generate nonce"))?;
        let obsf = cipher
            .encrypt(&nonce, cleartext)
            .context("Encryption failed")?;
        trace!("length of obsf: {}", obsf.len());
        let mut combined = nonce.to_vec();
        combined.extend_from_slice(&obsf);
        Ok(combined)
    }

    // pub fn decrypt(&self, obsf: &[u8]) -> anyhow::Result<Vec<u8>> {
    //     type NonceSize = <ChaCha20Poly1305 as AeadCore>::NonceSize;
    //     let cipher = generate_cypher(&self.key).context("Failed to generate cipher")?;
    //     let (nonce_bytes, ciphertext) = obsf.split_at(NonceSize::to_usize());
    //     let nonce =
    //         Nonce::<ChaCha20Poly1305>::try_from(nonce_bytes).context("Failed to parse nonce")?;
    //     let out = cipher
    //         .decrypt(&nonce, ciphertext)
    //         .context("Decryption failed")?;
    //     Ok(out)
    // }
}

impl<W: Write> Write for EncryptWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write> Drop for EncryptWriter<W> {
    fn drop(&mut self) {
        if !self.buffer.is_empty() {
            if let Ok(encrypted) = self.encrypt(&self.buffer) {
                trace!("Writing {} bytes encrypted data to writer", encrypted.len());
                let _ = self.writer.write_all(&encrypted);
            }
            self.buffer.clear();
        }
        let _ = self.writer.flush();
    }
}
