use std::collections::VecDeque;

// Import necessary Bevy modules.
use bevy::{asset::UntypedAssetId, platform::collections::HashSet, prelude::*};
use protocol::{Hero, MAX_HEALTH_COUNT, RankItem, THROW_END_TIME, uuid::Uuid};

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
pub struct InGameAssets {
    handles: Vec<UntypedHandle>,
}

impl AssetGroup for InGameAssets {
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
pub struct PlayerInfo {
    pub uuid: Uuid,
    pub name: String,
    pub hero: Hero,
    pub win: u16,
    pub lose: u16,
}

#[derive(Resource)]
pub struct OtherInfo {
    pub left_side: bool,
    pub name: String,
    pub hero: Hero,
    pub win: u16,
    pub lose: u16,
}

#[derive(Default, Resource)]
pub struct SelectedSliderCursor(Option<(VolumeSlider, Entity, u64)>);

impl SelectedSliderCursor {
    pub fn take(&mut self) -> Option<(VolumeSlider, Entity, u64)> {
        self.0.take()
    }

    pub fn get(&self) -> Option<(VolumeSlider, Entity, u64)> {
        self.0
    }

    pub fn set(&mut self, slider: VolumeSlider, entity: Entity, id: u64) {
        if self.0.is_none() {
            self.0 = Some((slider, entity, id))
        }
    }
}

#[allow(dead_code)]
pub enum MessageArgs {
    String(String),
    Integer(i32),
}

#[derive(Resource)]
pub struct ErrorMessage {
    pub tag: String,
    pub message: String,
    pub args: Vec<MessageArgs>,
}

impl ErrorMessage {
    pub fn new(tag: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            message: message.into(),
            args: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_args(mut self, args: Vec<MessageArgs>) -> Self {
        self.args = args;
        self
    }
}

#[cfg(target_arch = "wasm32")]
impl From<NetError> for ErrorMessage {
    fn from(e: NetError) -> Self {
        match e {
            NetError::NotFound => {
                ErrorMessage::new("net_not_found", "Failed to connect to the game server.")
            }
            NetError::Closed(code) => ErrorMessage::new(
                "net_closed",
                format!("Disconnected from the server. ({})", code),
            )
            .with_args(vec![MessageArgs::Integer(code as i32)]),
            NetError::Error(message) => ErrorMessage::new(
                "net_error",
                format!("Disconnected from the server. {}", message),
            )
            .with_args(vec![MessageArgs::String(message)]),
        }
    }
}

#[derive(Resource)]
pub struct SyncFlags;

#[derive(Resource)]
pub struct MouseButtonPressed;

#[derive(Resource)]
pub struct TouchPressed {
    pub id: u64,
}

#[derive(Default, Resource)]
pub struct InGameTimer {
    pub miliis: i32,
}

#[derive(Default, Resource)]
pub struct PlayerTimer {
    pub miliis: u16,
}

#[derive(Default, Resource, Clone, Copy, PartialEq, Eq)]
pub enum PlaySide {
    Left(Option<(u8, u8)>),
    Right(Option<(u8, u8)>),
    #[default]
    LeftThrown,
    RightThrown,
}

#[derive(Resource)]
pub struct Wind {
    angle: f32,
    power: f32,
}

impl Wind {
    pub fn new(angle: u8, power: u8) -> Self {
        Self {
            angle: angle as f32 / 255.0 * TAU,
            power: power as f32 / 255.0,
        }
    }

    pub fn get_rotation(&self, offset: f32) -> Rot2 {
        let angle = -(self.angle + offset).to_degrees();
        Rot2::degrees(angle)
    }

    pub fn get_power(&self) -> f32 {
        self.power
    }

    pub fn velocity(&self) -> Vec2 {
        Vec2::new(self.angle.cos(), self.angle.sin()) * self.power
    }
}

impl Default for Wind {
    fn default() -> Self {
        Self {
            angle: 0.0,
            power: 0.0,
        }
    }
}

#[derive(Resource)]
pub struct LeftPlayerHealth(pub usize);

impl Default for LeftPlayerHealth {
    fn default() -> Self {
        Self(MAX_HEALTH_COUNT)
    }
}

#[derive(Resource)]
pub struct RightPlayerHealth(pub usize);

impl Default for RightPlayerHealth {
    fn default() -> Self {
        Self(MAX_HEALTH_COUNT)
    }
}

pub struct Snapshot {
    pub timepoint: i32,
    pub position: Vec2,
    pub velocity: Vec2,
}

#[derive(Resource)]
pub struct ProjectileObject {
    total_remaining_millis: i32,
    remaining_millis: u16,
    snapshots: VecDeque<Snapshot>,
}

impl ProjectileObject {
    const MAX_SNAPSHOTS: usize = 15;
    const BUFFER_SIZE: usize = Self::MAX_SNAPSHOTS + 1;
    const DELAY: i32 = 100;
    const MAX_DELAY: i32 = 250;

    pub fn new(
        total_remaining_millis: i32,
        remaining_millis: u16,
        position: Vec2,
        velocity: Vec2,
    ) -> Self {
        let mut snapshots = VecDeque::with_capacity(Self::BUFFER_SIZE);
        snapshots.push_back(Snapshot {
            timepoint: total_remaining_millis,
            position,
            velocity,
        });

        Self {
            total_remaining_millis,
            remaining_millis,
            snapshots,
        }
    }

    pub fn add_snapshot(
        &mut self,
        total_remaining_millis: i32,
        remaining_millis: u16,
        position: Vec2,
        velocity: Vec2,
    ) {
        self.remaining_millis = remaining_millis;
        let diff_t = self
            .total_remaining_millis
            .saturating_sub(total_remaining_millis);
        if diff_t > Self::MAX_DELAY {
            self.total_remaining_millis = total_remaining_millis;
        }

        self.snapshots.push_back(Snapshot {
            timepoint: total_remaining_millis,
            position,
            velocity,
        });
        if self.snapshots.len() > Self::MAX_SNAPSHOTS {
            self.snapshots.pop_front();
        }
    }

    pub fn front(&self) -> Option<&Snapshot> {
        self.snapshots.front()
    }

    pub fn get(&mut self, elapsed_time: i32) -> (i32, Option<&Snapshot>, Option<&Snapshot>) {
        self.total_remaining_millis -= elapsed_time;
        let timepoint = self.total_remaining_millis + Self::DELAY;
        let mut prev = None;
        let mut next = None;
        for snapshot in self.snapshots.iter() {
            if snapshot.timepoint > timepoint {
                prev = next;
                next = Some(snapshot);
            } else {
                break;
            }
        }

        (timepoint, prev, next)
    }

    pub fn get_alpha(&self) -> f32 {
        self.remaining_millis as f32 / THROW_END_TIME as f32
    }
}

#[derive(Resource)]
pub struct RankingData {
    pub my_rank: Option<u32>,
    pub top_list: Vec<RankItem>,
}

impl RankingData {
    pub fn new(my_rank: Option<u32>, mut top_list: Vec<RankItem>) -> Self {
        top_list.sort_by_key(|i| i.rank);
        Self { my_rank, top_list }
    }
}

#[derive(Resource)]
pub struct GreetingFlag;
