use core::fmt;

// Import necessary Bevy modules.
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    platform::collections::HashMap,
    prelude::*,
    tasks::ConditionalSendFuture,
};
use serde::Deserialize;

use super::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Resource)]
pub enum Locale {
    #[default]
    En,
    Ja,
    Ko,
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Locale::En => write!(f, "en"),
            Locale::Ja => write!(f, "ja"),
            Locale::Ko => write!(f, "ko"),
        }
    }
}

#[derive(Default, Resource)]
pub struct LocalizationAssets {
    pub locale: HashMap<Locale, Handle<LocalizationData>>,
}

#[derive(Asset, TypePath, Deserialize)]
pub struct LocalizationData(pub HashMap<String, String>);

#[derive(Default)]
pub struct LocalizationDataLoader;

impl AssetLoader for LocalizationDataLoader {
    type Asset = LocalizationData;
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
            let data: LocalizationData = serde_json::from_slice(&bytes)?;
            Ok(data)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
