// Import necessary Bevy modules.
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    audio::Volume,
    prelude::*,
    tasks::ConditionalSendFuture,
};
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Resource, Deserialize, Serialize)]
pub struct SystemVolume {
    pub background: u8,
    pub effect: u8,
    pub voice: u8,
}

impl SystemVolume {
    pub fn get_background(&self) -> Volume {
        Volume::Linear(self.background as f32 / 255.0)
    }

    pub fn get_effect(&self) -> Volume {
        Volume::Linear(self.effect as f32 / 255.0)
    }

    pub fn get_voice(&self) -> Volume {
        Volume::Linear(self.voice as f32 / 255.0)
    }
}

impl Default for SystemVolume {
    fn default() -> Self {
        Self {
            background: 204,
            effect: 204,
            voice: 204,
        }
    }
}

#[derive(Default)]
pub struct SoundAssetLoader;

impl AssetLoader for SoundAssetLoader {
    type Asset = AudioSource;
    type Settings = ();
    type Error = LoaderError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let key = reconstruct_key();
            let decrypted_data = decrypt_bytes(&bytes, &key)?;

            Ok(AudioSource {
                bytes: decrypted_data.into(),
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["sound"]
    }
}
