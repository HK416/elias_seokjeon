// Import necessary Bevy modules.
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    tasks::ConditionalSendFuture,
};
use bevy_spine::SkeletonBinary;

use super::*;

#[derive(Debug, thiserror::Error)]
pub enum SkelLoaderError {
    #[error("Failed to load asset for the following reason:{0}")]
    IO(#[from] std::io::Error),
    #[error("Failed to decrypt asset for the following reason:{0}")]
    Crypt(#[from] anyhow::Error),
}

#[derive(Default)]
pub struct SkelAssetLoader;

impl AssetLoader for SkelAssetLoader {
    type Asset = SkeletonBinary;
    type Settings = ();
    type Error = SkelLoaderError;

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

            Ok(SkeletonBinary {
                binary: decrypted_data.to_vec(),
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["s_skel"]
    }
}
