// Import necessary Bevy modules.
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use super::*;

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
