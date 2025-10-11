// Import necessary Bevy modules.
use bevy::prelude::*;

#[derive(Component)]
pub struct LoadingStateRoot;

#[derive(Component)]
pub struct LoadingText;

#[derive(Component)]
pub struct LoadingBar;

#[derive(Component)]
pub struct SpawnRequest;

#[derive(Component)]
pub struct TitleLevelRoot;

#[derive(Component)]
pub struct TitleLevelEntity;

#[derive(Component)]
pub struct OriginColor(pub Color);

#[derive(Component, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum UI {
    InTitleGameStartButton,
    InTitleOptionButton,
    InTitleHowToPlayButton,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub enum Character {
    Butter,
    Kommy,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub enum CharacterAnimState {
    Idle,
    PatIdle,
    PatEnd,
    TouchIdle,
    TouchEnd,
    SmashEnd1,
    SmashEnd2,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum ColliderType {
    Ball,
    Head,
}

#[derive(Component)]
pub struct TargetSpine {
    pub entity: Entity,
}

impl TargetSpine {
    pub const fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

#[derive(Component)]
pub struct TargetSpineBone {
    pub entity: Entity,
    pub index: usize,
}

impl TargetSpineBone {
    pub const fn new(entity: Entity, index: usize) -> Self {
        Self { entity, index }
    }
}

#[derive(Component)]
pub struct SpineBoneOriginPosition {
    pub local: Vec2,
    pub world: Vec2,
}

#[derive(Component)]
pub struct Grabbed {
    pub elapsed: f32,
}

impl Default for Grabbed {
    fn default() -> Self {
        Self { elapsed: 0.0 }
    }
}

#[derive(Component)]
pub struct BallWaveAnimation {
    pub elapsed: f32,
    pub direction: Vec2,
    pub power: f32,
}
