// Import necessary Bevy modules.
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    tasks::ConditionalSendFuture,
};
use serde::Deserialize;

use crate::collider::Collider2d;

use super::*;

#[derive(Component)]
pub struct ColliderGroupHandle(pub Handle<ColliderGroup>);

impl ColliderGroupHandle {
    pub fn id(&self) -> AssetId<ColliderGroup> {
        self.0.id()
    }
}

#[derive(Asset, TypePath, Deserialize)]
pub struct ColliderGroup {
    pub ball: Collider2d,
    pub head: Collider2d,
}

#[derive(Default)]
pub struct ColliderAssetLoader;

impl AssetLoader for ColliderAssetLoader {
    type Asset = ColliderGroup;
    type Settings = ();
    type Error = LoaderError;

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
            let group: ColliderGroup = serde_json::from_slice(&decrypted_data)?;

            Ok(group)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["collider"]
    }
}
