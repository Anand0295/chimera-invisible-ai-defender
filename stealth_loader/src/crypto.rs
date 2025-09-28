//! Cryptographic utilities for stealth loader
//! 
//! ⚠️ SIMULATION ONLY - Uses simplified crypto for research demonstration

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::{Context, Result};
use rand::RngCore;

pub struct CryptoManager {
    cipher: Aes256Gcm,
}

impl CryptoManager {
    pub fn new(key: &[u8; 32]) -> Self {
        let cipher = Aes256Gcm::new_from_slice(key).expect("Invalid key length");
        Self { cipher }
    }

    pub fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);
        key
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        // Prepend nonce to ciphertext
        let mut result = Vec::with_capacity(12 + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>> {
        if encrypted_data.len() < 12 {
            return Err(anyhow::anyhow!("Invalid encrypted data length"));
        }

        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

        Ok(plaintext)
    }
}

/// Obfuscation utilities for payload protection
pub mod obfuscation {
    use rand::Rng;

    pub fn simple_xor_obfuscate(data: &[u8], key: &[u8]) -> Vec<u8> {
        data.iter()
            .enumerate()
            .map(|(i, &b)| b ^ key[i % key.len()])
            .collect()
    }

    pub fn generate_obfuscation_key(length: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        (0..length).map(|_| rng.gen()).collect()
    }

    /// Add random padding to make analysis harder
    pub fn add_padding(data: &[u8], target_size: usize) -> Vec<u8> {
        if data.len() >= target_size {
            return data.to_vec();
        }

        let mut result = data.to_vec();
        let padding_size = target_size - data.len();
        let mut rng = rand::thread_rng();
        
        for _ in 0..padding_size {
            result.push(rng.gen());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_roundtrip() {
        let key = CryptoManager::generate_key();
        let crypto = CryptoManager::new(&key);
        
        let plaintext = b"Test message for encryption";
        let encrypted = crypto.encrypt(plaintext).unwrap();
        let decrypted = crypto.decrypt(&encrypted).unwrap();
        
        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_obfuscation() {
        let data = b"sensitive data";
        let key = b"key123";
        
        let obfuscated = obfuscation::simple_xor_obfuscate(data, key);
        let deobfuscated = obfuscation::simple_xor_obfuscate(&obfuscated, key);
        
        assert_eq!(data, deobfuscated.as_slice());
    }
}