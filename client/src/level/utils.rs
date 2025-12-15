// Import necessary Bevy modules.
use bevy::{audio::PlaybackMode, ecs::relationship::RelatedSpawnerCommands, prelude::*};
use bevy_spine::Spine;

use crate::assets::sound::SystemVolume;

use super::*;

pub fn add_vertical_space(
    loading_entities: &mut LoadingEntities,
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    height: Val,
) {
    let entity = parent
        .spawn((
            Node {
                height,
                ..Default::default()
            },
            SpawnRequest,
        ))
        .id();
    loading_entities.insert(entity);
}

pub fn add_horizontal_space(
    loading_entities: &mut LoadingEntities,
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    width: Val,
) {
    let entity = parent
        .spawn((
            Node {
                width,
                ..Default::default()
            },
            SpawnRequest,
        ))
        .id();
    loading_entities.insert(entity);
}

pub fn update_button_visual(
    entity: Entity,
    interaction: &Interaction,
    children_query: &Query<&Children>,
    text_color_query: &mut Query<(&mut TextColor, &OriginColor<TextColor>)>,
    button_color_query: &mut Query<(&mut BackgroundColor, &OriginColor<BackgroundColor>)>,
) {
    if let Ok((mut text_color, origin_color)) = text_color_query.get_mut(entity) {
        let color = match interaction {
            Interaction::None => origin_color.none,
            Interaction::Hovered => origin_color.hovered,
            Interaction::Pressed => origin_color.pressed,
        };

        *text_color = TextColor(color);
    } else if let Ok((mut background_color, origin_color)) = button_color_query.get_mut(entity) {
        let color = match interaction {
            Interaction::None => origin_color.none,
            Interaction::Hovered => origin_color.hovered,
            Interaction::Pressed => origin_color.pressed,
        };

        *background_color = BackgroundColor(color);
    }

    let Ok(children) = children_query.get(entity) else {
        return;
    };

    for &entity in children {
        update_button_visual(
            entity,
            interaction,
            children_query,
            text_color_query,
            button_color_query,
        );
    }
}

pub fn play_character_animation(
    spine: &mut Spine,
    character: Character,
    anim_state: CharacterAnimState,
) {
    let (animation_name, looping) = match anim_state {
        CharacterAnimState::Idle => {
            let hero: Hero = character.into();
            let animation_name = TITLE_ANIM[hero as usize];
            (animation_name, true)
        }
        CharacterAnimState::PatIdle => (PAT_IDLE, true),
        CharacterAnimState::PatEnd => (PAT_END, false),
        CharacterAnimState::TouchIdle => (TOUCH_IDLE, true),
        CharacterAnimState::TouchEnd => (TOUCH_END, false),
        CharacterAnimState::SmashEnd1 => (SMASH_END_1, false),
        CharacterAnimState::SmashEnd2 => (SMASH_END_2, false),
        CharacterAnimState::InGame => (IDLE, true),
        CharacterAnimState::InGameHit1 => (SMASH_END_1, false),
        CharacterAnimState::InGameHit2 => (SMASH_END_2, false),
        CharacterAnimState::Happy => (HAPPY_1, true),
        CharacterAnimState::Sad => (SAD_1, true),
    };

    spine
        .animation_state
        .set_animation_by_name(0, animation_name, looping)
        .unwrap();
}

pub fn normalized_wave(t: f32, a: f32, k: f32, omega: f32, phi: f32) -> f32 {
    a * (1.0 - t).powf(k) * (omega * t * TAU + phi).sin()
}

#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn play_effect_sound(
    commands: &mut Commands,
    system_volume: &SystemVolume,
    source: Handle<AudioSource>,
) {
    #[cfg(target_arch = "wasm32")]
    commands.spawn((
        WebAudioPlayer::new(source),
        WebPlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: system_volume.get_effect(),
            ..Default::default()
        },
        EffectSound,
    ));
}

#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn play_voice_sound(
    commands: &mut Commands,
    system_volume: &SystemVolume,
    source: Handle<AudioSource>,
    channel: VoiceChannel,
) {
    #[cfg(target_arch = "wasm32")]
    commands.spawn((
        WebAudioPlayer::new(source),
        WebPlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: system_volume.get_voice(),
            ..Default::default()
        },
        VoiceSound { channel },
    ));
}

pub fn cleanup_voices(
    channel: &VoiceChannel,
    commands: &mut Commands,
    voices: &Query<(Entity, &VoiceSound)>,
) {
    for (entity, sound) in voices.iter() {
        if sound.channel.eq(channel) {
            commands.entity(entity).despawn();
        }
    }
}
