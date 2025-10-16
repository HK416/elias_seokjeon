// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::InitMatchingCancel),
            (debug_label, setup_matching_cancel),
        )
        .add_systems(
            OnExit(LevelStates::InitMatchingCancel),
            cleanup_loading_resource,
        )
        .add_systems(
            Update,
            (
                update_entity_spawn_progress,
                observe_entity_creation,
                check_loading_progress,
            )
                .run_if(in_state(LevelStates::InitMatchingCancel)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InitMatchingCancel");
}

fn setup_matching_cancel(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut loading_entities = LoadingEntities::default();
    setup_matching_cancel_interface(&mut commands, &asset_server, &mut loading_entities);

    // --- Resource Insertion ---
    commands.insert_resource(loading_entities);
}

fn setup_matching_cancel_interface(
    commands: &mut Commands,
    asset_server: &AssetServer,
    loading_entities: &mut LoadingEntities,
) {
    let entity = commands
        .spawn((
            Node {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.5)),
            UI::InMatchingCancelModal,
            Visibility::Hidden,
            SpawnRequest,
            ZIndex(3),
        ))
        .with_children(|parent| {
            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Percent(30.0)),
                    BackgroundColor(BORDER_GREEN_COLOR_0),
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .with_children(|parent| {
                    let width = 0.97;
                    let height = 1.0 - ASPECT_RATIO * (1.0 - width);
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(width * 100.0),
                                height: Val::Percent(height * 100.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(30.0)),
                            BackgroundColor(BG_GREEN_COLOR_3),
                            Visibility::Inherited,
                            SpawnRequest,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(90.0),
                                        height: Val::Percent(50.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .with_children(|parent| {
                                    let font = asset_server.load(FONT_PATH);
                                    let entity = parent
                                        .spawn((
                                            Node::default(),
                                            Text::new("Are you sure you want to cancel?"),
                                            TextFont::from(font),
                                            TextLayout::new_with_justify(Justify::Center),
                                            TextColor::BLACK,
                                            TranslatableText("cancel_message".into()),
                                            ResizableFont::vertical(1280.0, 64.0),
                                            Visibility::Inherited,
                                            SpawnRequest,
                                        ))
                                        .id();
                                    loading_entities.insert(entity);
                                })
                                .id();
                            loading_entities.insert(entity);

                            add_vertical_space(loading_entities, parent, Val::Percent(10.0));

                            let entity = parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(90.0),
                                        height: Val::Percent(20.0),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .with_children(|parent| {
                                    let percent_width = width * 0.9 * 0.4;
                                    let percent_height = height * 0.2;
                                    let entity = parent
                                        .spawn((
                                            Node {
                                                width: Val::Percent(40.0),
                                                height: Val::Percent(100.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..Default::default()
                                            },
                                            UI::InMatchingCancelYesButton,
                                            Visibility::Inherited,
                                            SpawnRequest,
                                            Button,
                                        ))
                                        .with_children(|parent| {
                                            create_button(
                                                loading_entities,
                                                parent,
                                                ASPECT_RATIO,
                                                percent_width,
                                                percent_height,
                                                BORDER_RED_COLOR_0,
                                                BG_RED_COLOR_0,
                                                None,
                                                None,
                                                BoxShadow::default(),
                                                |commands| {
                                                    let font = asset_server.load(FONT_PATH);
                                                    commands.insert((
                                                        Text::new("Yes"),
                                                        TextFont::from(font),
                                                        TextLayout::new_with_justify(
                                                            Justify::Center,
                                                        ),
                                                        TextColor::WHITE,
                                                        OriginColor::new(Color::WHITE),
                                                        TranslatableText("yes".into()),
                                                        ResizableFont::vertical(1280.0, 36.0),
                                                        Visibility::Inherited,
                                                    ));
                                                },
                                            );
                                        })
                                        .id();
                                    loading_entities.insert(entity);

                                    add_horizontal_space(
                                        loading_entities,
                                        parent,
                                        Val::Percent(5.0),
                                    );

                                    let entity = parent
                                        .spawn((
                                            Node {
                                                width: Val::Percent(40.0),
                                                height: Val::Percent(100.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..Default::default()
                                            },
                                            UI::InMatchingCancelNoButton,
                                            Visibility::Inherited,
                                            SpawnRequest,
                                            Button,
                                        ))
                                        .with_children(|parent| {
                                            create_button(
                                                loading_entities,
                                                parent,
                                                ASPECT_RATIO,
                                                percent_width,
                                                percent_height,
                                                BORDER_YELLO_COLOR_0,
                                                BG_YELLO_COLOR_0,
                                                None,
                                                None,
                                                BoxShadow::default(),
                                                |commands| {
                                                    let font = asset_server.load(FONT_PATH);
                                                    commands.insert((
                                                        Text::new("No"),
                                                        TextFont::from(font),
                                                        TextLayout::new_with_justify(
                                                            Justify::Center,
                                                        ),
                                                        TextColor::BLACK,
                                                        OriginColor::new(Color::BLACK),
                                                        TranslatableText("no".into()),
                                                        ResizableFont::vertical(1280.0, 42.0),
                                                        Visibility::Inherited,
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

        commands.insert(MatchingCancelLevelEntity);
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
        next_state.set(LevelStates::InTitle);
    }
}
