// Import necessary Bevy modules.
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    tasks::ConditionalSendFuture,
};
use serde::Deserialize;

use super::*;

#[derive(Asset, TypePath, Deserialize)]
pub struct ConfigData {
    #[allow(dead_code)]
    pub server_url: String,
}

#[derive(Default)]
pub struct ConfigAssetLoader;

impl AssetLoader for ConfigAssetLoader {
    type Asset = ConfigData;
    type Settings = ();
    type Error = LoaderError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            // Read the bytes from the reader.
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let key = reconstruct_key();
            let decrypted_data = decrypt_bytes(&bytes, &key)?;
            let config: ConfigData = serde_json::from_slice(&decrypted_data)?;

            Ok(config)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
