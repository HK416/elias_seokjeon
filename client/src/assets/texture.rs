use std::io::Cursor;

use bevy::{
    asset::{AssetLoader, LoadContext, RenderAssetUsages, io::Reader},
    image::ImageSampler,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    tasks::ConditionalSendFuture,
};

use super::*;

/// A loader for texel assets.
#[derive(Default)]
pub struct TexelAssetLoader;

impl AssetLoader for TexelAssetLoader {
    type Asset = Image;
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

            // Decode the image data using the `image` crate and create a Bevy `Image` asset.
            let mut reader = image::ImageReader::new(Cursor::new(decrypted_data));
            reader.set_format(image::ImageFormat::Png);

            let image = reader.decode()?;
            let size = Extent3d {
                width: image.width(),
                height: image.height(),
                depth_or_array_layers: 1,
            };
            let dimension = TextureDimension::D2;
            let data = image.to_rgba8().to_vec();
            let format = TextureFormat::Rgba8UnormSrgb;
            let asset_usage = RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD;

            let mut image_asset = Image::new(size, dimension, data, format, asset_usage);
            // Set the sampler to `Nearest` for a pixelated look, which is common for sprites.
            // This prevents blurring when scaling the image.
            image_asset.sampler = ImageSampler::nearest();

            Ok(image_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["texture"]
    }
}
