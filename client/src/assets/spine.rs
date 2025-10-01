use std::path::Path;

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    tasks::ConditionalSendFuture,
};
use bevy_spine::SkeletonData;
use serde::Deserialize;

use super::*;

#[derive(Deserialize)]
pub struct SpineModel {
    pub skel: String,
    pub atlas: String,
}

/// An error that can occur when loading a texel data.
#[derive(Debug, thiserror::Error)]
pub enum SpineLoaderError {
    /// An I/O error occurred.
    #[error("Failed to load asset for the following reason:{0}")]
    IO(#[from] std::io::Error),
    /// A JSON deserialization error occurred.
    #[error("Failed to decode asset for the following reason:{0}")]
    Json(#[from] serde_json::Error),
    #[error("Failed to decrypt asset for the following reason:{0}")]
    Crypt(#[from] anyhow::Error),
}

#[derive(Default)]
pub struct ModelAssetLoader;

impl AssetLoader for ModelAssetLoader {
    type Asset = SkeletonData;
    type Settings = ();
    type Error = SpineLoaderError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        info!("asset load: {}", &load_context.asset_path());
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let key = reconstruct_key();
            let decrypted_data = decrypt_bytes(&bytes, &key)?;
            let model: SpineModel = serde_json::from_slice(&decrypted_data)?;

            let dir = load_context
                .path()
                .parent()
                .unwrap_or(Path::new(""))
                .to_path_buf();

            let mut path = dir.clone();
            path.push(model.skel);
            let h_binary = load_context.load(path);

            let mut path = dir.clone();
            path.push(model.atlas);
            let h_atlas = load_context.load(path);

            Ok(SkeletonData::new_from_binary(h_binary, h_atlas))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["model"]
    }
}
