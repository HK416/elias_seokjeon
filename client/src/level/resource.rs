// Import necessary Bevy modules.
use bevy::{asset::UntypedAssetId, prelude::*};

pub trait AssetGroup: Resource {
    fn push(&mut self, handle: impl Into<UntypedHandle>);

    fn ids(&self) -> Vec<UntypedAssetId>;

    fn len(&self) -> usize;
}

#[derive(Default, Resource)]
pub struct SystemAssets {
    pub handles: Vec<UntypedHandle>,
}

impl AssetGroup for SystemAssets {
    fn push(&mut self, handle: impl Into<UntypedHandle>) {
        self.handles.push(handle.into());
    }

    fn ids(&self) -> Vec<UntypedAssetId> {
        self.handles.iter().map(|h| h.id()).collect()
    }

    fn len(&self) -> usize {
        self.handles.len()
    }
}

#[derive(Default, Resource)]
pub struct RetryCounter(pub u32);

#[derive(Resource)]
pub struct SceneTimer {
    elapsed_time: f32,
}

impl SceneTimer {
    pub fn elapsed_sec(&self) -> f32 {
        self.elapsed_time
    }

    pub fn tick(&mut self, elapsed: f32) {
        self.elapsed_time += elapsed;
    }

    pub fn reset(&mut self) {
        self.elapsed_time = 0.0;
    }
}

impl Default for SceneTimer {
    fn default() -> Self {
        Self { elapsed_time: 0.0 }
    }
}
