// Import necessary Bevy modules.
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

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

pub fn create_button<F>(
    loading_entities: &mut LoadingEntities,
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    border_color: Color,
    bg_color: Color,
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
            OriginColor(border_color),
            Visibility::Inherited,
            shadow,
            SpawnRequest,
        ))
        .with_children(|parent| {
            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(96.0),
                        height: Val::Percent(86.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Percent(30.0)),
                    BackgroundColor(bg_color),
                    OriginColor(bg_color),
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
            Interaction::None => origin_color.0,
            Interaction::Hovered => origin_color.0.darker(0.15),
            Interaction::Pressed => origin_color.0.darker(0.3),
        };

        *text_color = TextColor(color);
    } else if let Ok((mut background_color, origin_color)) = button_color_query.get_mut(entity) {
        let color = match interaction {
            Interaction::None => origin_color.0,
            Interaction::Hovered => origin_color.0.darker(0.15),
            Interaction::Pressed => origin_color.0.darker(0.3),
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
