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
pub struct TitleLevelSub;

#[derive(Component)]
pub struct OriginColor(pub Color);

#[derive(Component, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum UI {
    InTitleGameStartButton,
    InTitleOptionButton,
    InTitleHowToPlayButton,
}

#[derive(Component, PartialEq, Eq)]
pub enum Character {
    Butter,
    Kommy,
}
