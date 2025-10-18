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

#[allow(clippy::too_many_arguments)]
pub fn create_button<F>(
    loading_entities: &mut LoadingEntities,
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    aspect_ratio: f32,
    percent_width: f32,
    percent_height: f32,
    border_color: Color,
    inner_color: Color,
    hoverd_color: Option<Color>,
    pressed_color: Option<Color>,
    shadow: BoxShadow,
    func: F,
) where
    F: FnOnce(&mut EntityCommands<'_>),
{
    let entity = parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderRadius::all(Val::Percent(30.0)),
            BackgroundColor(border_color),
            OriginColor::fill(border_color),
            Visibility::Inherited,
            shadow,
            SpawnRequest,
            ZIndex(5),
        ))
        .with_children(|parent| {
            let t = percent_width / percent_height * aspect_ratio;
            let width = 0.96;
            let height = 1.0 - t * (1.0 - width);
            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(width * 100.0),
                        height: Val::Percent(height * 100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Percent(30.0)),
                    BackgroundColor(inner_color),
                    OriginColor::new(inner_color)
                        .with_hovered(hoverd_color.unwrap_or(inner_color.darker(0.15)))
                        .with_pressed(pressed_color.unwrap_or(inner_color.darker(0.3))),
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .with_children(|parent| {
                    let mut commands =
                        parent.spawn((Node::default(), Visibility::Inherited, SpawnRequest));
                    func(&mut commands);

                    let entity = commands.id();
                    loading_entities.insert(entity);
                })
                .id();
            loading_entities.insert(entity);
        })
        .id();
    loading_entities.insert(entity);
}

pub fn update_button_visual(
    entity: Entity,
    interaction: &Interaction,
    children_query: &Query<&Children>,
    text_color_query: &mut Query<(&mut TextColor, &OriginColor)>,
    button_color_query: &mut Query<(&mut BackgroundColor, &OriginColor)>,
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
            Character::Butter => (BUTTER_TITLE_IDLE, true),
            Character::Kommy => (KOMMY_TITLE_TAUNT, true),
        },
        CharacterAnimState::PatIdle => (PAT_IDLE, true),
        CharacterAnimState::PatEnd => (PAT_END, false),
        CharacterAnimState::TouchIdle => (TOUCH_IDLE, true),
        CharacterAnimState::TouchEnd => (TOUCH_END, false),
        CharacterAnimState::SmashEnd1 => (SMASH_END_1, false),
        CharacterAnimState::SmashEnd2 => (SMASH_END_2, false),
    };

    spine
        .animation_state
        .set_animation_by_name(0, animation_name, looping)
        .unwrap();
}

pub fn normalized_wave(t: f32, a: f32, k: f32, omega: f32, phi: f32) -> f32 {
    a * (1.0 - t).powf(k) * (omega * t * TAU + phi).sin()
}
