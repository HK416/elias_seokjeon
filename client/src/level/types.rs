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
pub struct LeaderBoardLevelEntity;

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

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub enum PNButton {
    Positive,
    Negative,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub enum LocaleButton {
    En,
    Ja,
    Ko,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub enum VolumeSlider {
    Background,
    Effect,
    Voice,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub enum VolumeLevelTextId {
    Background,
    Effect,
    Voice,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub enum TitleButton {
    GameStart,
    Option,
    Ranking,
    HowToPlay,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub enum Character {
    Aya,
    BigWood,
    Butter,
    Erpin,
    Kidian,
    Kommy,
    Mayo,
    Rohne,
    Speaki,
    Xion,
}

impl Character {
    pub fn trans(&self, distance: Vec2, scale_x: f32) -> Vec2 {
        match self {
            Character::Aya => distance.xy() * Vec2::new(scale_x, 1.0),
            Character::BigWood => distance.yx() * Vec2::new(1.0, -scale_x),
            Character::Butter => distance.yx() * Vec2::new(1.0, -scale_x),
            Character::Kommy => distance.yx() * Vec2::new(1.0, -scale_x),
            Character::Erpin => distance.xy() * Vec2::new(scale_x, 1.0),
            Character::Kidian => distance.xy() * Vec2::new(scale_x, 1.0),
            Character::Mayo => distance.yx() * Vec2::new(1.0, -scale_x),
            Character::Rohne => distance.yx() * Vec2::new(1.0, -scale_x),
            Character::Speaki => distance.xy() * Vec2::new(scale_x, 1.0),
            Character::Xion => distance.xy() * Vec2::new(scale_x, 1.0),
        }
    }
}

impl From<Hero> for Character {
    fn from(hero: Hero) -> Self {
        match hero {
            Hero::Aya => Self::Aya,
            Hero::BigWood => Self::BigWood,
            Hero::Butter => Self::Butter,
            Hero::Kommy => Self::Kommy,
            Hero::Erpin => Self::Erpin,
            Hero::Kidian => Self::Kidian,
            Hero::Mayo => Self::Mayo,
            Hero::Rohne => Self::Rohne,
            Hero::Speaki => Self::Speaki,
            Hero::Xion => Self::Xion,
        }
    }
}

impl Into<Hero> for Character {
    fn into(self) -> Hero {
        match self {
            Self::Aya => Hero::Aya,
            Self::BigWood => Hero::BigWood,
            Self::Butter => Hero::Butter,
            Self::Kommy => Hero::Kommy,
            Self::Erpin => Hero::Erpin,
            Self::Kidian => Hero::Kidian,
            Self::Mayo => Hero::Mayo,
            Self::Rohne => Hero::Rohne,
            Self::Speaki => Hero::Speaki,
            Self::Xion => Hero::Xion,
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
    pub touch_id: u64,
}

impl Grabbed {
    pub fn new(touch_id: u64) -> Self {
        Self {
            touch_id,
            ..Default::default()
        }
    }
}

impl Default for Grabbed {
    fn default() -> Self {
        Self {
            elapsed: 0.0,
            touch_id: 0,
        }
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

impl Default for BackoutScale {
    fn default() -> Self {
        Self {
            delay: 0.0,
            duration: 0.0,
            elapsed: 0.0,
            start: Vec3::ZERO,
            end: Vec3::ZERO,
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

impl Default for UiBackOutScale {
    fn default() -> Self {
        Self {
            duration: 0.0,
            elapsed: 0.0,
            start: Vec2::ZERO,
            end: Vec2::ZERO,
        }
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

#[derive(Default, Component)]
pub struct Projectile {
    pub hit: bool,
}

#[derive(Component)]
pub struct BackgroundSound;

#[derive(Component)]
pub struct EffectSound;

#[derive(Default, Component)]
pub struct VoiceSound {
    pub channel: VoiceChannel,
}

#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
pub enum VoiceChannel {
    #[default]
    MySelf,
    Other,
}

#[derive(Component)]
pub struct RankEntry(pub usize);

#[derive(Component)]
pub struct MyRankEntry;

#[derive(Component)]
pub struct RankItemNum;

#[derive(Component)]
pub struct RankItemUuid;

#[derive(Component)]
pub struct RankItemName;

#[derive(Component)]
pub struct RankItemWins;

#[derive(Component)]
pub struct RankItemLosses;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GuideGestureStatus {
    Moved,
    FadeOut,
}

#[derive(Component)]
pub struct GuideGestureTimer {
    status: GuideGestureStatus,
    fade_out_duration: f32,
    moving_duration: f32,
    elapsed: f32,
}

impl GuideGestureTimer {
    pub fn new(duration: f32) -> Self {
        assert!(duration > 0.0);
        Self {
            status: GuideGestureStatus::Moved,
            fade_out_duration: duration * 0.3,
            moving_duration: duration * 0.7,
            elapsed: 0.0,
        }
    }

    pub fn tick(&mut self, delta: f32) {
        // Update the elapsed time
        self.elapsed += delta;

        loop {
            let duration = self.duration();
            if self.elapsed < duration {
                break;
            }

            self.elapsed -= duration;
            self.status = match self.status {
                GuideGestureStatus::Moved => GuideGestureStatus::FadeOut,
                GuideGestureStatus::FadeOut => GuideGestureStatus::Moved,
            };
        }
    }

    pub fn duration(&self) -> f32 {
        match self.status {
            GuideGestureStatus::Moved => self.moving_duration,
            GuideGestureStatus::FadeOut => self.fade_out_duration,
        }
    }

    pub fn percent(&self) -> (GuideGestureStatus, f32) {
        (self.status, self.elapsed / self.duration())
    }

    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.status = GuideGestureStatus::Moved;
    }
}
