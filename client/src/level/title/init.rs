// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::InitTitle),
            (debug_label, setup_title_screen),
        )
        .add_systems(
            OnExit(LevelStates::InitTitle),
            (remove_resource, cleanup_loading_screen),
        )
        .add_systems(
            Update,
            (
                observe_entity_creation,
                update_entity_spawn_progress,
                check_loading_progress,
            )
                .run_if(in_state(LevelStates::InitTitle)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InitTitle");
}

fn setup_title_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut loading_entities = LoadingEntities::default();

    let texture = asset_server.load(IMG_PATH_BACKGROUND);
    let entity = commands
        .spawn((
            ImageNode::new(texture),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                aspect_ratio: Some(1593.0 / 1019.0),
                ..Default::default()
            },
            ZIndex(5),
            Visibility::Hidden,
            SpawnRequest,
        ))
        .with_children(|parent| {
            let entity = parent
                .spawn((
                    Node {
                        width: Val::Vw(100.0),
                        height: Val::Vh(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .with_children(|parent| {
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(26.0),
                                height: Val::Percent(80.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            Visibility::Inherited,
                            SpawnRequest,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(16.0),
                                        ..Default::default()
                                    },
                                    ZIndex(4),
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .with_children(|parent| {
                                    create_button(
                                        &mut loading_entities,
                                        parent,
                                        BTN_BG_BORDER_COLOR,
                                        BTN_BG_COLOR,
                                        BoxShadow::new(
                                            Color::BLACK.with_alpha(0.8),
                                            Val::Percent(2.0),
                                            Val::Percent(10.0),
                                            Val::Percent(5.0),
                                            Val::Px(1.0),
                                        ),
                                        |commands| {
                                            let font = asset_server.load(FONT_PATH);
                                            commands.insert((
                                                Text::new("Game Start"),
                                                TextFont::from_font(font),
                                                TextLayout::new_with_justify(JustifyText::Center),
                                                TextColor::BLACK,
                                                TranslatableText("game_start".into()),
                                                ResizableFont::vertical(1280.0, 52.0),
                                            ));
                                        },
                                    );
                                })
                                .id();
                            loading_entities.insert(entity);
                        })
                        .id();
                    loading_entities.insert(entity);
                })
                .id();
            loading_entities.insert(entity);
        })
        .id();
    loading_entities.insert(entity);

    // --- Resource Insersion ---
    commands.insert_resource(loading_entities);
}

// --- CLEANUP SYSTEMS ---

fn remove_resource(mut commands: Commands) {
    commands.remove_resource::<LoadingEntities>();
}

// --- UPDATE SYSTEMS ---

fn observe_entity_creation(
    mut commands: Commands,
    mut loading_entities: ResMut<LoadingEntities>,
    query: Query<(Entity, Option<&ChildOf>), Added<SpawnRequest>>,
) {
    for (entity, child_of) in query.iter() {
        loading_entities.remove(entity);

        let mut commands = commands.entity(entity);
        commands.remove::<SpawnRequest>();

        if child_of.is_some() {
            commands.insert(TitleLevelSub);
        } else {
            commands.insert(TitleLevelRoot);
        }
    }
}

fn check_loading_progress(
    loading_entities: Res<LoadingEntities>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    if loading_entities.is_empty() {
        next_state.set(LevelStates::InTitle);
    }
}
