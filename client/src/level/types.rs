use std::{marker::PhantomData, num::NonZeroUsize};

// Import necessary Bevy modules.
use bevy::prelude::*;
use protocol::Hero;

#[derive(Component)]
pub struct LoadingStateRoot;

#[derive(Component)]
pub struct LoadingText;

#[derive(Component)]
pub struct LoadingBar;

#[derive(Component)]
pub struct SpawnRequest;

#[derive(Component)]
pub struct OptionLevelEntity;

#[derive(Component)]
pub struct TitleLevelRoot;

#[derive(Component)]
pub struct TitleBackground;

#[derive(Component)]
pub struct TitleLevelEntity;

#[derive(Component)]
pub struct TitleMessageText;

#[derive(Component)]
pub struct TitleMessageLevelEntity;

#[derive(Component)]
pub struct MatchingLevelEntity;

#[derive(Component)]
pub struct MatchingCancelLevelEntity;

#[derive(Component)]
pub struct MatchingStatusMessage;

#[derive(Component)]
pub struct EnterGameLevelEntity;

#[derive(Component)]
pub struct EnterGameLoadingBar;

#[derive(Component)]
pub struct EnterGameLoadingCursor;

#[derive(Component)]
pub struct InPrepareLevelEntity;

#[derive(Component)]
pub struct InGameLevelEntity;

#[derive(Component)]
pub struct InGameLevelRoot;

#[derive(Component)]
pub struct InGameResultLevelEntity;

#[derive(Component)]
pub struct GameResultVictory;

#[derive(Component)]
pub struct GameResultDefeat;

#[derive(Component)]
pub struct GameResultDraw;

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
    Modal,
    PositiveButton,
    NegativeButton,

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

    EnterGameLoadingBackground,
    EnterGameLoadingBar,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub enum Character {
    Butter,
    Kommy,
}

impl Character {
    pub fn new(hero: Hero) -> Self {
        match hero {
            Hero::Butter => Self::Butter,
            Hero::Kommy => Self::Kommy,
        }
    }
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
    InGame,
    InGameHit1,
    InGameHit2,
    Happy,
    Sad,
    Blank,
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
pub struct FadeEffect {
    duration: f32,
    elapsed: f32,
}

impl FadeEffect {
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
pub struct SmoothScale {
    delay: f32,
    duration: f32,
    elapsed: f32,
    start: Vec3,
    end: Vec3,
}

impl SmoothScale {
    pub fn new(duration: f32, start: Vec3, end: Vec3) -> Self {
        assert!(duration > 0.0, "duration must be greater than 0.0");
        Self {
            delay: 0.0,
            duration,
            elapsed: 0.0,
            start,
            end,
        }
    }

    pub fn with_delay(mut self, delay: f32) -> Self {
        self.delay = delay;
        self
    }

    pub fn tick(&mut self, delta: f32) {
        self.elapsed = (self.elapsed + delta).min(self.duration + self.delay);
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration + self.delay
    }

    pub fn scale(&self) -> Vec3 {
        if self.elapsed < self.delay {
            self.start
        } else {
            let t = (self.elapsed - self.delay) / self.duration;
            let t = 3.0 * t.powi(2) - 2.0 * t.powi(3);
            (self.start * (1.0 - t) + self.end * t).max(Vec3::ZERO)
        }
    }
}

#[derive(Component)]
pub struct BackoutScale {
    delay: f32,
    duration: f32,
    elapsed: f32,
    start: Vec3,
    end: Vec3,
}

impl BackoutScale {
    pub fn new(duration: f32, start: Vec3, end: Vec3) -> Self {
        assert!(duration > 0.0, "duration must be greater than 0.0");
        Self {
            delay: 0.0,
            duration,
            elapsed: 0.0,
            start,
            end,
        }
    }

    pub fn with_delay(mut self, delay: f32) -> Self {
        self.delay = delay;
        self
    }

    pub fn tick(&mut self, delta: f32) {
        self.elapsed = (self.elapsed + delta).min(self.duration + self.delay);
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration + self.delay
    }

    pub fn scale(&self) -> Vec3 {
        if self.elapsed < self.delay {
            self.start
        } else {
            let t = (self.elapsed - self.delay) / self.duration;
            let t = 1.0 + 2.70158 * (t - 1.0).powi(3) - 1.70158 * (t - 1.0).powi(2);
            (self.start * (1.0 - t) + self.end * t).max(Vec3::ZERO)
        }
    }
}

#[derive(Component)]
pub struct UiSmoothScale {
    duration: f32,
    elapsed: f32,
    start: Vec2,
    end: Vec2,
}

impl UiSmoothScale {
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
        let t = 3.0 * t.powi(2) - 2.0 * t.powi(3);
        self.start * (1.0 - t) + self.end * t
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

#[derive(Component)]
pub struct BackgroundPattern(pub usize);

#[derive(Component)]
pub struct AnimationTimer {
    num_sheets: NonZeroUsize,
    repeat: bool,
    duration: f32,
    elapsed: f32,
}

impl AnimationTimer {
    pub fn new(duration: f32, num_sheets: NonZeroUsize, repeat: bool) -> Self {
        Self {
            num_sheets,
            repeat,
            duration,
            elapsed: 0.0,
        }
    }

    pub fn tick(&mut self, delta: f32) {
        self.elapsed += delta;
        if self.repeat {
            self.elapsed %= self.duration;
        } else {
            self.elapsed = self.elapsed.min(self.duration);
        }
    }

    pub fn reset(&mut self) {
        self.elapsed = 0.0;
    }

    pub fn frame_index(&self) -> usize {
        let num_sheets = self.num_sheets.get();
        num_sheets.min((self.elapsed / self.duration * num_sheets as f32) as usize)
    }
}

#[derive(Component)]
pub struct UiAnimationTarget;

#[derive(Component)]
pub struct LeftHealth1;

#[derive(Component)]
pub struct LeftHealth2;

#[derive(Component)]
pub struct LeftHealth3;

#[derive(Component)]
pub struct LeftHealth4;

#[derive(Component)]
pub struct LeftHealth5;

#[derive(Component)]
pub struct RightHealth1;

#[derive(Component)]
pub struct RightHealth2;

#[derive(Component)]
pub struct RightHealth3;

#[derive(Component)]
pub struct RightHealth4;

#[derive(Component)]
pub struct RightHealth5;

#[derive(Component)]
pub struct RemainingTimer;

#[derive(Component)]
pub struct UiTurnTimer;

#[derive(Component)]
pub struct TurnTimer;

#[derive(Component)]
pub struct WindIndicator;

#[derive(Component)]
pub struct LeftPlayerTrigger;

#[derive(Component)]
pub struct RightPlayerTrigger;

#[derive(Component)]
pub struct LeftPlayerHead(pub Entity);

#[derive(Component)]
pub struct RightPlayerHead(pub Entity);

#[derive(Component)]
pub struct Projectile {
    pub hit: bool,
}

impl Default for Projectile {
    fn default() -> Self {
        Self { hit: false }
    }
}
