mod core;
mod in_game;
mod matching;
mod option;
mod title;

mod constants;
mod resource;
mod system;
mod types;
mod utils;

// Import necessary Bevy modules.
use bevy::prelude::*;
use bevy_spine::{Spine, SpineEvent};
use protocol::Hero;
use rand::seq::IndexedRandom;

use crate::{
    WND_HEIGHT, WND_WIDTH,
    assets::{path::*, sound::SystemVolume},
    collider::*,
    resizable_font::*,
    translatable_text::*,
};

#[cfg(target_arch = "wasm32")]
use crate::web::*;

use self::{constants::*, resource::*, system::*, types::*, utils::*};

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LevelStates>()
            .add_plugins(core::InnerPlugin)
            .add_plugins(in_game::InnerPlugin)
            .add_plugins(matching::InnerPlugin)
            .add_plugins(option::InnerPlugin)
            .add_plugins(title::InnerPlugin)
            .add_systems(
                Update,
                (
                    update_smooth_anim,
                    update_backout_anim,
                    handle_spine_animation_completed,
                    update_wave_animation,
                ),
            );
    }
}

// --- STATES ---

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum LevelStates {
    Error,
    #[default]
    Setup, // -> Connect, Error
    Connect, // -> LoadTitle, Error

    LoadTitle,          // -> InitOption, Error
    InitOption,         // -> InitMatching, Error
    InitMatching,       // -> InitMatchingCancel, Error
    InitMatchingCancel, // -> InitInTitleMessage, Error
    InitInTitleMessage, // -> InitLeaderBoard, Error
    InitLeaderBoard,    // -> InitEnterGame, Error
    InitEnterGame,      // -> InitTitle, Error
    InitTitle,          // -> InTitle, Error
    InTitle,            // -> SwitchToInMatching, SwitchToInOption, Error

    SwitchToInOption, // -> InOption
    InOption,         // -> InTitle, Error

    InTitleMessage,       // -> InTitle, Error
    SwitchToTitleMessage, // -> InTitleMessage

    SwitchToInMatching, // -> InMatching
    InMatching,         // -> SwitchToInMatchingCancel, SwitchToLoadGame, Error

    SwitchToInMatchingCancel, // -> InMatchingCancel
    InMatchingCancel,         // -> InTitle, SwitchToInMatching, Error

    InitPrepareGame,   // -> SwitchToTitleMessage, InitGameResult, Error
    SwitchToInPrepare, // -> InPrepareGame, Error
    InPrepareGame,     // -> SwitchToInGame, Error

    SwitchToLoadGame, // -> LoadGame
    LoadGame,         // -> SwitchToTitleMessage, InitGame, Error
    InitGame,         // -> SwitchToTitleMessage, InitPrepareGame, Error

    SwitchToInGame, // -> InGame, Error
    InGame,         // -> Error

    InitGameResult, // -> SwitchToTitleMessage, SwitchToInPrepare, Error
    SwitchToGameVictory,
    SwitchToGameDefeat,
    SwitchToGameDraw,
    InGameResult,

    SwitchToLeaderBoard,
    LeaderBoard,
}

// --- UPDATE SYSTEMS ---

fn update_smooth_anim(
    mut commands: Commands,
    mut query: Query<(Entity, &mut UiSmoothScale, &mut UiTransform)>,
    time: Res<Time>,
) {
    for (entity, mut back_out, mut transform) in query.iter_mut() {
        back_out.tick(time.delta_secs());
        transform.scale = back_out.scale();

        if back_out.is_finished() {
            commands.entity(entity).remove::<UiSmoothScale>();
        }
    }
}

fn update_backout_anim(
    mut commands: Commands,
    mut query: Query<(Entity, &mut UiBackOutScale, &mut UiTransform)>,
    time: Res<Time>,
) {
    for (entity, mut back_out, mut transform) in query.iter_mut() {
        back_out.tick(time.delta_secs());
        transform.scale = back_out.scale();

        if back_out.is_finished() {
            commands.entity(entity).remove::<UiBackOutScale>();
        }
    }
}

fn handle_spine_animation_completed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    mut spine_events: MessageReader<SpineEvent>,
    mut spine_query: Query<(
        &mut Spine,
        &Character,
        &VoiceChannel,
        &mut CharacterAnimState,
    )>,
    voices: Query<(Entity, &VoiceSound)>,
) {
    for event in spine_events.read() {
        if let SpineEvent::Complete { entity, animation } = event
            && let Ok((mut spine, character, channel, mut anim_state)) =
                spine_query.get_mut(*entity)
            && let Some(track) = spine.animation_state.get_current(0)
        {
            if track.animation().name() != animation {
                continue;
            }

            *anim_state = match *anim_state {
                CharacterAnimState::PatEnd
                | CharacterAnimState::TouchEnd
                | CharacterAnimState::SmashEnd2 => CharacterAnimState::Idle,
                CharacterAnimState::SmashEnd1 => CharacterAnimState::SmashEnd2,
                CharacterAnimState::InGameHit1 => CharacterAnimState::InGameHit2,
                CharacterAnimState::InGameHit2 => CharacterAnimState::InGame,
                _ => continue,
            };

            match *anim_state {
                CharacterAnimState::SmashEnd2 => {
                    cleanup_voices(channel, &mut commands, &voices);
                    let hero: Hero = (*character).into();
                    let path = HERO_VOICE_SETS[hero as usize]
                        .ducth_rub_end()
                        .last()
                        .copied()
                        .unwrap();
                    let source = asset_server.load(path);
                    play_voice_sound(&mut commands, &system_volume, source, *channel);
                }
                _ => { /* empty */ }
            }

            play_character_animation(&mut spine, *character, *anim_state);
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_wave_animation(
    mut commands: Commands,
    mut spine_query: Query<(&mut Spine, &CharacterAnimState)>,
    mut wave_anim_query: Query<(
        Entity,
        &TargetSpine,
        &TargetSpineBone,
        &SpineBoneOriginPosition,
        &mut BallWaveAnimation,
    )>,
    time: Res<Time>,
) {
    for (entity, target_spine, target_spine_bone, origin_position, mut wave_anim) in
        wave_anim_query.iter_mut()
    {
        wave_anim.elapsed += time.delta_secs();
        let t = (wave_anim.elapsed / BALL_WAVE_DURATION).min(1.0);
        let delta = normalized_wave(t, 0.5, 1.0, 5.0, PI);

        if let Ok((mut spine, anim_state)) = spine_query.get_mut(target_spine.entity)
            && let Some(mut bone) = spine.skeleton.bone_at_index_mut(target_spine_bone.index)
        {
            if matches!(*anim_state, CharacterAnimState::TouchIdle) {
                bone.set_position(origin_position.local);
                spine.skeleton.update_world_transform();
                commands.entity(entity).remove::<BallWaveAnimation>();
                continue;
            }

            bone.set_position(
                origin_position.local + wave_anim.direction.yx() * delta * wave_anim.power,
            );
            spine.skeleton.update_world_transform();
        }

        if wave_anim.elapsed > BALL_WAVE_DURATION {
            commands.entity(entity).remove::<BallWaveAnimation>();
        }
    }
}
