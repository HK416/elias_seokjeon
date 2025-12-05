// Import necessary Bevy modules.
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use bevy_spine::Spine;

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
        CharacterAnimState::Idle => match character {
            Character::Butter => (BUTTER_TITLE, true),
            Character::Kommy => (KOMMY_TITLE, true),
        },
        CharacterAnimState::PatIdle => (PAT_IDLE, true),
        CharacterAnimState::PatEnd => (PAT_END, false),
        CharacterAnimState::TouchIdle => (TOUCH_IDLE, true),
        CharacterAnimState::TouchEnd => (TOUCH_END, false),
        CharacterAnimState::SmashEnd1 => (SMASH_END_1, false),
        CharacterAnimState::SmashEnd2 => (SMASH_END_2, false),
        CharacterAnimState::InGame => match character {
            Character::Butter => (BUTTER_IDLE, true),
            Character::Kommy => (KOMMY_IDLE, true),
        },
        CharacterAnimState::InGameHit1 => (SMASH_END_1, false),
        CharacterAnimState::InGameHit2 => (SMASH_END_2, false),
    };

    spine
        .animation_state
        .set_animation_by_name(0, animation_name, looping)
        .unwrap();
}

pub fn normalized_wave(t: f32, a: f32, k: f32, omega: f32, phi: f32) -> f32 {
    a * (1.0 - t).powf(k) * (omega * t * TAU + phi).sin()
}
