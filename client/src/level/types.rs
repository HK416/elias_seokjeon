use std::marker::PhantomData;

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
pub struct MatchingStatusMessage;

#[derive(Component)]
pub struct EnterGameLevelEntity;

#[derive(Component)]
pub struct BluredBackground;

#[derive(Component)]
pub struct EnterGameLoadingBar;

#[derive(Component)]
pub struct EnterGameLoadingCursor;

#[derive(Component)]
pub struct InGameLevelEntity;

#[derive(Component)]
pub struct InGameLevelRoot;

#[derive(Component)]
pub struct OriginColor<T> {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
    _panthom: PhantomData<T>,
}

impl<T> OriginColor<T> {
    pub fn new(none: Color) -> Self {
        Self {
            none,
            hovered: none.darker(0.15),
            pressed: none.darker(0.3),
            _panthom: PhantomData,
        }
    }

    pub fn fill(none: Color) -> Self {
        Self {
            none,
            hovered: none,
            pressed: none,
            _panthom: PhantomData,
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
    Root,

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

    InOptionBackground,
    InOptionModal,
    InOptionExitButton,

    InMatchingCancelBackground,
    InMatchingCancelModal,
    InMatchingCancelYesButton,
    InMatchingCancelNoButton,

    InMatchingBackground,
    InMatchingModal,
    InMatchingCancelButton,

    EnterGameLoadingBackground,
    EnterGameLoadingBar,
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

#[derive(Component)]
pub struct FadeIn {
    duration: f32,
    elapsed: f32,
}

impl FadeIn {
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            elapsed: 0.0,
        }
    }

    pub fn tick(&mut self, delta: f32) {
        self.elapsed = (self.elapsed + delta).min(self.duration);
    }

    pub fn progress(&self) -> f32 {
        self.elapsed / self.duration
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }
}

#[derive(Component)]
pub struct FadeOut {
    duration: f32,
    elapsed: f32,
}

impl FadeOut {
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            elapsed: 0.0,
        }
    }

    pub fn tick(&mut self, delta: f32) {
        self.elapsed = (self.elapsed + delta).min(self.duration);
    }

    pub fn progress(&self) -> f32 {
        self.elapsed / self.duration
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }
}

#[derive(Component)]
pub struct UiBackOutScale {
    duration: f32,
    elapsed: f32,
    start: Vec2,
    end: Vec2,
}

impl UiBackOutScale {
    pub fn new(duration: f32, start: Vec2, end: Vec2) -> Self {
        assert!(duration > 0.0, "duration must be greater than 0.0");
        Self {
            duration,
            elapsed: 0.0,
            start,
            end,
        }
    }

    pub fn tick(&mut self, delta: f32) {
        self.elapsed = (self.elapsed + delta).min(self.duration);
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }

    pub fn scale(&self) -> Vec2 {
        let t = self.elapsed / self.duration;
        let t = 1.0 + 2.70158 * (t - 1.0).powi(3) - 1.70158 * (t - 1.0).powi(2);
        (self.start * (1.0 - t) + self.end * t).max(Vec2::ZERO)
    }
}
