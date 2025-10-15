// Import necessary Bevy modules.
use bevy::{asset::UntypedAssetId, platform::collections::HashSet, prelude::*};
use protocol::uuid::Uuid;

use super::*;

pub trait AssetGroup: Resource {
    fn push(&mut self, handle: impl Into<UntypedHandle>);

    fn ids(&self) -> Vec<UntypedAssetId>;

    fn len(&self) -> usize;
}

#[derive(Default, Resource)]
pub struct SystemAssets {
    handles: Vec<UntypedHandle>,
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
pub struct TitleAssets {
    handles: Vec<UntypedHandle>,
}

impl AssetGroup for TitleAssets {
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
pub struct LoadingEntities {
    entities: HashSet<Entity>,
    total: usize,
}

impl LoadingEntities {
    pub fn insert(&mut self, entity: Entity) {
        self.entities.insert(entity);
        self.total += 1;
    }

    pub fn remove(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    pub fn percent(&self) -> f32 {
        if self.total > 0 {
            let remaining = self.total.saturating_sub(self.entities.len());
            remaining as f32 / self.total as f32
        } else {
            1.0
        }
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

#[derive(Resource)]
#[allow(dead_code)]
pub struct PlayerInfo {
    pub uuid: Uuid,
    pub username: String,
}

#[derive(Default, Resource)]
pub struct SelectedSliderCursor(Option<(UI, Entity, u64)>);

impl SelectedSliderCursor {
    pub fn take(&mut self) -> Option<(UI, Entity, u64)> {
        self.0.take()
    }

    pub fn get(&self) -> Option<(UI, Entity, u64)> {
        self.0
    }

    pub fn set(&mut self, ui: UI, entity: Entity, id: u64) {
        if self.0.is_none() {
            self.0 = Some((ui, entity, id))
        }
    }
}
