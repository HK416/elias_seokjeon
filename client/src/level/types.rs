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
pub struct OptionLevelRoot;

#[derive(Component)]
pub struct OptionLevelEntity;

#[derive(Component)]
pub struct TitleLevelRoot;

#[derive(Component)]
pub struct TitleLevelEntity;

#[derive(Component)]
pub struct MatchingLevelEntity;

#[derive(Component)]
pub struct MatchingCancelLevelEntity;

#[derive(Component)]
pub struct OriginColor {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

impl OriginColor {
    pub fn new(none: Color) -> Self {
        Self {
            none,
            hovered: none.darker(0.15),
            pressed: none.darker(0.3),
        }
    }

    pub fn fill(none: Color) -> Self {
        Self {
            none,
            hovered: none,
            pressed: none,
        }
    }

    pub fn with_hovered(mut self, hovered: Color) -> Self {
        self.hovered = hovered;
        self
    }

    pub fn with_pressed(mut self, pressed: Color) -> Self {
        self.pressed = pressed;
        self
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum UI {
    BackgroundVolumeSlider,
    BackgroundVolume,
    EffectVolumeSlider,
    EffectVolume,
    VoiceVolumeSlider,
    VoiceVolume,

    LocaleButtonEn,
    LocaleButtonJa,
    LocaleButtonKo,

    InTitleGameStartButton,
    InTitleOptionButton,
    InTitleHowToPlayButton,

    InOptionModal,
    InOptionExitButton,

    InMatchingCancelModal,
    InMatchingCancelYesButton,
    InMatchingCancelNoButton,

    InMatchingModal,
    InMatchingCancelButton,
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
