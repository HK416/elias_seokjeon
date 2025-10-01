use std::{path::Path, sync::Arc};

// Import necessary Bevy modules.
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    tasks::ConditionalSendFuture,
};
use bevy_spine::{Atlas, rusty_spine};

use super::*;

#[derive(Debug, thiserror::Error)]
pub enum AtlasLoaderError {
    #[error("Failed to load asset for the following reason:{0}")]
    IO(#[from] std::io::Error),
    #[error("Spine error: {0}")]
    Spine(#[from] rusty_spine::SpineError),
    #[error("Failed to decrypt asset for the following reason:{0}")]
    Crypt(#[from] anyhow::Error),
}

#[derive(Default)]
pub struct AtlasAssetLoader;

impl AssetLoader for AtlasAssetLoader {
    type Asset = Atlas;
    type Settings = ();
    type Error = AtlasLoaderError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        info!("asset load: {}", &load_context.asset_path());
        Box::pin(async move {
            // Read the bytes from the reader.
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let key = reconstruct_key();
            let decrypted_data = decrypt_bytes(&bytes, &key)?;
            let dir = load_context.path().parent().unwrap_or(Path::new(""));

            Ok(Atlas {
                atlas: Arc::new(rusty_spine::Atlas::new(&decrypted_data, dir)?),
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["s_atlas"]
    }
}
