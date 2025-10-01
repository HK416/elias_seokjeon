pub mod path;

mod atlas;
mod skeleton;
mod spine;
mod texture;

use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
use anyhow::anyhow;
use bevy::prelude::*;
use static_assertions::const_assert_eq;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(skeleton::SkelAssetLoader)
            .register_asset_loader(atlas::AtlasAssetLoader)
            .register_asset_loader(spine::ModelAssetLoader)
            .register_asset_loader(texture::TexelAssetLoader);
    }
}

// --- CRYPT KEYS ---

const OBFUSCATED_KEY: &[u8] =
    include_bytes!(concat!(env!("CARGO_WORKSPACE_DIR"), "/assets/key.bin"));
const MASK: &[u8] = include_bytes!(concat!(env!("CARGO_WORKSPACE_DIR"), "/assets/mask.bin"));

const_assert_eq!(OBFUSCATED_KEY.len(), 32);
const_assert_eq!(MASK.len(), 32);

#[inline(never)]
pub fn reconstruct_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    for i in 0..32 {
        key[i] = OBFUSCATED_KEY[i] ^ MASK[i];
    }
    key
}

pub fn decrypt_bytes(encrypted_data: &[u8], key: &[u8]) -> anyhow::Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    if encrypted_data.len() < 12 {
        warn!("Encrypted data is too short to contain a nonce.");
        return Err(anyhow!("Encrypted data is too short to contain a nonce."));
    }

    let nonce = Nonce::from_slice(&encrypted_data[0..12]);
    let ciphertext = &encrypted_data[12..];
    let decrypted_data = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow!("Decryption failed: {}", e))?;
    Ok(decrypted_data)
}
