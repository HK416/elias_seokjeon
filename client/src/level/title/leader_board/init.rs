// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::InitLeaderBoard),
            (debug_label, setup_leader_board),
        )
        .add_systems(
            OnExit(LevelStates::InitLeaderBoard),
            cleanup_loading_resource,
        )
        .add_systems(
            Update,
            (
                update_entity_spawn_progress,
                observe_entity_creation,
                check_loading_progress,
            )
                .run_if(in_state(LevelStates::InitLeaderBoard)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InitLeaderBoard");
}

fn setup_leader_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_info: Res<PlayerInfo>,
) {
    let mut loading_entities = LoadingEntities::default();
    setup_leader_board_interface(
        &mut commands,
        &asset_server,
        &mut loading_entities,
        &player_info,
    );

    // --- Resource Insertion ---
    commands.insert_resource(loading_entities);
}

fn setup_leader_board_interface(
    commands: &mut Commands,
    asset_server: &AssetServer,
    loading_entities: &mut LoadingEntities,
    player_info: &PlayerInfo,
) {
    let entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            Visibility::Hidden,
            SpawnRequest,
            ZIndex(3),
        ))
        .with_children(|parent| {
            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(60.0),
                        height: Val::Percent(80.0),
                        border: UiRect::all(Val::VMin(1.25)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Percent(15.0)),
                    BorderColor::all(BORDER_GREEN_COLOR_0),
                    BackgroundColor(BG_GREEN_COLOR_3),
                    Visibility::Inherited,
                    UiAnimationTarget,
                    SpawnRequest,
                ))
                .with_children(|parent| {
                    // --- Title ---
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(90.0),
                                height: Val::Percent(12.0),
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
                                    Node::default(),
                                    Text::new("Ranking"),
                                    TextFont::from(asset_server.load(FONT_PATH)),
                                    TextLayout::new_with_justify(Justify::Center),
                                    TranslatableText("game_rank".into()),
                                    ResizableFont::vertical(1280.0, 42.0),
                                    OriginColor::<TextColor>::fill(Color::BLACK),
                                    TextColor::BLACK,
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .id();
                            loading_entities.insert(entity);
                        })
                        .id();
                    loading_entities.insert(entity);

                    add_vertical_space(loading_entities, parent, Val::Percent(3.0));

                    // --- LeaderBoard ---
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(90.0),
                                height: Val::Percent(67.0),
                                border: UiRect::all(Val::VMin(0.5)),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderColor::all(Color::BLACK),
                            BackgroundColor(Color::WHITE),
                            Visibility::Inherited,
                            SpawnRequest,
                        ))
                        .with_children(|parent| {
                            // --- Title ---
                            let entity = parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(10.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    BackgroundColor(BG_YELLO_COLOR_0),
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .with_children(|parent| {
                                    // --- Rank ---
                                    let entity = parent
                                        .spawn((
                                            Node {
                                                width: Val::Percent(10.0),
                                                height: Val::Percent(100.0),
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
                                                    Node::default(),
                                                    Text::new("Rank"),
                                                    TextFont::from(asset_server.load(FONT_PATH)),
                                                    TextLayout::new_with_justify(Justify::Center),
                                                    TranslatableText("rank".into()),
                                                    ResizableFont::vertical(1280.0, 21.0),
                                                    TextColor::BLACK,
                                                    Visibility::Inherited,
                                                    SpawnRequest,
                                                ))
                                                .id();
                                            loading_entities.insert(entity);
                                        })
                                        .id();
                                    loading_entities.insert(entity);

                                    // --- Uuid ---
                                    let entity = parent
                                        .spawn((
                                            Node {
                                                width: Val::Percent(45.0),
                                                height: Val::Percent(100.0),
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
                                                    Node::default(),
                                                    Text::new("UUID"),
                                                    TextFont::from(asset_server.load(FONT_PATH)),
                                                    TextLayout::new_with_justify(Justify::Center),
                                                    ResizableFont::vertical(1280.0, 21.0),
                                                    TextColor::BLACK,
                                                    Visibility::Inherited,
                                                    SpawnRequest,
                                                ))
                                                .id();
                                            loading_entities.insert(entity);
                                        })
                                        .id();
                                    loading_entities.insert(entity);

                                    // --- Name ---
                                    let entity = parent
                                        .spawn((
                                            Node {
                                                width: Val::Percent(25.0),
                                                height: Val::Percent(100.0),
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
                                                    Node::default(),
                                                    Text::new("Name"),
                                                    TextFont::from(asset_server.load(FONT_PATH)),
                                                    TextLayout::new_with_justify(Justify::Center),
                                                    TranslatableText("name".into()),
                                                    ResizableFont::vertical(1280.0, 21.0),
                                                    TextColor::BLACK,
                                                    Visibility::Inherited,
                                                    SpawnRequest,
                                                ))
                                                .id();
                                            loading_entities.insert(entity);
                                        })
                                        .id();
                                    loading_entities.insert(entity);

                                    // --- Win ---
                                    let entity = parent
                                        .spawn((
                                            Node {
                                                width: Val::Percent(10.0),
                                                height: Val::Percent(100.0),
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
                                                    Node::default(),
                                                    Text::new("Win"),
                                                    TextFont::from(asset_server.load(FONT_PATH)),
                                                    TextLayout::new_with_justify(Justify::Center),
                                                    TranslatableText("win".into()),
                                                    ResizableFont::vertical(1280.0, 21.0),
                                                    TextColor::BLACK,
                                                    Visibility::Inherited,
                                                    SpawnRequest,
                                                ))
                                                .id();
                                            loading_entities.insert(entity);
                                        })
                                        .id();
                                    loading_entities.insert(entity);

                                    // --- Lose ---
                                    let entity = parent
                                        .spawn((
                                            Node {
                                                width: Val::Percent(10.0),
                                                height: Val::Percent(100.0),
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
                                                    Node::default(),
                                                    Text::new("Lose"),
                                                    TextFont::from(asset_server.load(FONT_PATH)),
                                                    TextLayout::new_with_justify(Justify::Center),
                                                    TranslatableText("lose".into()),
                                                    ResizableFont::vertical(1280.0, 21.0),
                                                    TextColor::BLACK,
                                                    Visibility::Inherited,
                                                    SpawnRequest,
                                                ))
                                                .id();
                                            loading_entities.insert(entity);
                                        })
                                        .id();
                                    loading_entities.insert(entity);
                                })
                                .id();
                            loading_entities.insert(entity);

                            // --- Content ---
                            const LINE_COLOR: [Color; 2] =
                                [Color::WHITE, Color::srgb(0.88, 0.88, 0.88)];
                            for i in 0..10 {
                                let entity = parent
                                    .spawn((
                                        Node {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(8.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..Default::default()
                                        },
                                        BackgroundColor(LINE_COLOR[i % 2]),
                                        Visibility::Inherited,
                                        SpawnRequest,
                                    ))
                                    .with_children(|parent| {
                                        // --- Rank ---
                                        let entity = parent
                                            .spawn((
                                                Node {
                                                    width: Val::Percent(10.0),
                                                    height: Val::Percent(100.0),
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
                                                        Node::default(),
                                                        Text::new(format!("{}", i + 1)),
                                                        TextFont::from(
                                                            asset_server.load(FONT_PATH),
                                                        ),
                                                        TextLayout::new_with_justify(
                                                            Justify::Center,
                                                        ),
                                                        ResizableFont::vertical(1280.0, 20.0),
                                                        TextColor::BLACK,
                                                        Visibility::Inherited,
                                                        SpawnRequest,
                                                    ))
                                                    .id();
                                                loading_entities.insert(entity);
                                            })
                                            .id();
                                        loading_entities.insert(entity);

                                        // --- Uuid ---
                                        let entity = parent
                                            .spawn((
                                                Node {
                                                    width: Val::Percent(45.0),
                                                    height: Val::Percent(100.0),
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
                                                        Node::default(),
                                                        Text::new("-"),
                                                        TextFont::from(
                                                            asset_server.load(FONT_PATH),
                                                        ),
                                                        TextLayout::new_with_justify(
                                                            Justify::Center,
                                                        ),
                                                        ResizableFont::vertical(1280.0, 20.0),
                                                        TextColor::BLACK,
                                                        Visibility::Inherited,
                                                        SpawnRequest,
                                                        RankItemUuid,
                                                        RankEntry(i),
                                                    ))
                                                    .id();
                                                loading_entities.insert(entity);
                                            })
                                            .id();
                                        loading_entities.insert(entity);

                                        // --- Name ---
                                        let entity = parent
                                            .spawn((
                                                Node {
                                                    width: Val::Percent(25.0),
                                                    height: Val::Percent(100.0),
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
                                                        Node::default(),
                                                        Text::new("-"),
                                                        TextFont::from(
                                                            asset_server.load(FONT_PATH),
                                                        ),
                                                        TextLayout::new_with_justify(
                                                            Justify::Center,
                                                        ),
                                                        ResizableFont::vertical(1280.0, 20.0),
                                                        TextColor::BLACK,
                                                        Visibility::Inherited,
                                                        SpawnRequest,
                                                        RankItemName,
                                                        RankEntry(i),
                                                    ))
                                                    .id();
                                                loading_entities.insert(entity);
                                            })
                                            .id();
                                        loading_entities.insert(entity);

                                        // --- Win ---
                                        let entity = parent
                                            .spawn((
                                                Node {
                                                    width: Val::Percent(10.0),
                                                    height: Val::Percent(100.0),
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
                                                        Node::default(),
                                                        Text::new("-"),
                                                        TextFont::from(
                                                            asset_server.load(FONT_PATH),
                                                        ),
                                                        TextLayout::new_with_justify(
                                                            Justify::Center,
                                                        ),
                                                        ResizableFont::vertical(1280.0, 20.0),
                                                        TextColor::BLACK,
                                                        Visibility::Inherited,
                                                        SpawnRequest,
                                                        RankItemWins,
                                                        RankEntry(i),
                                                    ))
                                                    .id();
                                                loading_entities.insert(entity);
                                            })
                                            .id();
                                        loading_entities.insert(entity);

                                        // --- Lose ---
                                        let entity = parent
                                            .spawn((
                                                Node {
                                                    width: Val::Percent(10.0),
                                                    height: Val::Percent(100.0),
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
                                                        Node::default(),
                                                        Text::new("-"),
                                                        TextFont::from(
                                                            asset_server.load(FONT_PATH),
                                                        ),
                                                        TextLayout::new_with_justify(
                                                            Justify::Center,
                                                        ),
                                                        ResizableFont::vertical(1280.0, 21.0),
                                                        TextColor::BLACK,
                                                        Visibility::Inherited,
                                                        RankItemLosses,
                                                        SpawnRequest,
                                                        RankEntry(i),
                                                    ))
                                                    .id();
                                                loading_entities.insert(entity);
                                            })
                                            .id();
                                        loading_entities.insert(entity);
                                    })
                                    .id();
                                loading_entities.insert(entity);
                            }

                            // --- My Rank ---
                            let entity = parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(10.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    BackgroundColor(BG_YELLO_COLOR_0),
                                    Visibility::Inherited,
                                    SpawnRequest,
                                    MyRankEntry,
                                ))
                                .with_children(|parent| {
                                    // --- Rank ---
                                    let entity = parent
                                        .spawn((
                                            Node {
                                                width: Val::Percent(10.0),
                                                height: Val::Percent(100.0),
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
                                                    Node::default(),
                                                    Text::new("-"),
                                                    TextFont::from(asset_server.load(FONT_PATH)),
                                                    TextLayout::new_with_justify(Justify::Center),
                                                    ResizableFont::vertical(1280.0, 20.0),
                                                    TextColor::BLACK,
                                                    Visibility::Inherited,
                                                    SpawnRequest,
                                                    RankItemNum,
                                                ))
                                                .id();
                                            loading_entities.insert(entity);
                                        })
                                        .id();
                                    loading_entities.insert(entity);

                                    // --- Uuid ---
                                    let entity = parent
                                        .spawn((
                                            Node {
                                                width: Val::Percent(45.0),
                                                height: Val::Percent(100.0),
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
                                                    Node::default(),
                                                    Text::new(format!("{}", player_info.uuid)),
                                                    TextFont::from(asset_server.load(FONT_PATH)),
                                                    TextLayout::new_with_justify(Justify::Center),
                                                    ResizableFont::vertical(1280.0, 20.0),
                                                    TextColor::BLACK,
                                                    Visibility::Inherited,
                                                    SpawnRequest,
                                                ))
                                                .id();
                                            loading_entities.insert(entity);
                                        })
                                        .id();
                                    loading_entities.insert(entity);

                                    // --- Name ---
                                    let entity = parent
                                        .spawn((
                                            Node {
                                                width: Val::Percent(25.0),
                                                height: Val::Percent(100.0),
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
                                                    Node::default(),
                                                    Text::new(&player_info.name),
                                                    TextFont::from(asset_server.load(FONT_PATH)),
                                                    TextLayout::new_with_justify(Justify::Center),
                                                    ResizableFont::vertical(1280.0, 20.0),
                                                    TextColor::BLACK,
                                                    Visibility::Inherited,
                                                    SpawnRequest,
                                                ))
                                                .id();
                                            loading_entities.insert(entity);
                                        })
                                        .id();
                                    loading_entities.insert(entity);

                                    // --- Win ---
                                    let entity = parent
                                        .spawn((
                                            Node {
                                                width: Val::Percent(10.0),
                                                height: Val::Percent(100.0),
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
                                                    Node::default(),
                                                    Text::new(format!("{}", player_info.win)),
                                                    TextFont::from(asset_server.load(FONT_PATH)),
                                                    TextLayout::new_with_justify(Justify::Center),
                                                    ResizableFont::vertical(1280.0, 20.0),
                                                    TextColor::BLACK,
                                                    Visibility::Inherited,
                                                    SpawnRequest,
                                                    RankItemWins,
                                                ))
                                                .id();
                                            loading_entities.insert(entity);
                                        })
                                        .id();
                                    loading_entities.insert(entity);

                                    // --- Lose ---
                                    let entity = parent
                                        .spawn((
                                            Node {
                                                width: Val::Percent(10.0),
                                                height: Val::Percent(100.0),
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
                                                    Node::default(),
                                                    Text::new(format!("{}", player_info.lose)),
                                                    TextFont::from(asset_server.load(FONT_PATH)),
                                                    TextLayout::new_with_justify(Justify::Center),
                                                    ResizableFont::vertical(1280.0, 21.0),
                                                    TextColor::BLACK,
                                                    Visibility::Inherited,
                                                    RankItemLosses,
                                                    SpawnRequest,
                                                ))
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

                    add_vertical_space(loading_entities, parent, Val::Percent(3.0));

                    // --- Exit Button ---
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(40.0),
                                height: Val::Percent(12.0),
                                border: UiRect::all(Val::VMin(0.8)),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(30.0)),
                            OriginColor::<BackgroundColor>::new(BG_YELLO_COLOR_0),
                            BorderColor::all(BORDER_YELLO_COLOR_0),
                            BackgroundColor(BG_YELLO_COLOR_0),
                            Visibility::Inherited,
                            PNButton::Positive,
                            SpawnRequest,
                            Button,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Node::default(),
                                    Text::new("Back"),
                                    TextFont::from(asset_server.load(FONT_PATH)),
                                    TextLayout::new_with_justify(Justify::Center),
                                    TranslatableText("back".into()),
                                    ResizableFont::vertical(1280.0, 42.0),
                                    OriginColor::<TextColor>::fill(Color::BLACK),
                                    TextColor::BLACK,
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .id();
                            loading_entities.insert(entity);
                        })
                        .id();
                    loading_entities.insert(entity);

                    add_vertical_space(loading_entities, parent, Val::Percent(3.0));
                })
                .id();
            loading_entities.insert(entity);
        })
        .id();
    loading_entities.insert(entity);
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

        commands.insert(LeaderBoardLevelEntity);
        if child_of.is_none() {
            commands.insert(TitleLevelRoot);
        }
    }
}

fn check_loading_progress(
    loading_entities: Res<LoadingEntities>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    if loading_entities.is_empty() {
        next_state.set(LevelStates::InitEnterGame);
    }
}
