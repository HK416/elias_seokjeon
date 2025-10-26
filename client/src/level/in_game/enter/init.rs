// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::InitEnterGame),
            (debug_label, setup_enter_game),
        )
        .add_systems(OnExit(LevelStates::InitEnterGame), cleanup_loading_resource)
        .add_systems(
            Update,
            (
                update_entity_spawn_progress,
                observe_entity_creation,
                check_loading_progress,
            )
                .run_if(in_state(LevelStates::InitEnterGame)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InitEnterGame");
}

fn setup_enter_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_assets: Res<Assets<Image>>,
) {
    let mut loading_entities = LoadingEntities::default();
    setup_enter_game_screen(
        &mut commands,
        &asset_server,
        &image_assets,
        &mut loading_entities,
    );
    setup_enter_game_interface(&mut commands, &asset_server, &mut loading_entities);

    // --- Resource Insertion ---
    commands.insert_resource(loading_entities);
}

fn setup_enter_game_screen(
    commands: &mut Commands,
    asset_server: &AssetServer,
    image_assets: &Assets<Image>,
    loading_entities: &mut LoadingEntities,
) {
    let handle = asset_server.load(IMG_PATH_BACKGROUND_BLURED);
    let image = image_assets.get(handle.id()).unwrap();
    let aspect_ratio = image.aspect_ratio().ratio();
    let size = (WND_WIDTH as f32, WND_WIDTH as f32 / aspect_ratio);
    let half_height = WND_HEIGHT as f32 * 0.5;
    let entity = commands
        .spawn((
            Sprite {
                image: handle,
                image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
                custom_size: Some(size.into()),
                color: Color::WHITE.with_alpha(0.0),
                ..Default::default()
            },
            Transform::from_xyz(0.0, half_height, 1.0),
            Visibility::Hidden,
            BluredBackground,
            SpawnRequest,
        ))
        .id();
    loading_entities.insert(entity);
}

fn setup_enter_game_interface(
    commands: &mut Commands,
    asset_server: &AssetServer,
    loading_entities: &mut LoadingEntities,
) {
    let entity = commands
        .spawn((
            Node {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            UI::EnterGameLoadingBar,
            Visibility::Hidden,
            SpawnRequest,
            ZIndex(4),
        ))
        .with_children(|parent| {
            let entity = parent
                .spawn((
                    Node {
                        height: Val::Percent(70.0),
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .id();
            loading_entities.insert(entity);

            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(80.0),
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .with_children(|parent| {
                    let entity = parent
                        .spawn((
                            Node {
                                left: Val::Percent(0.0),
                                ..Default::default()
                            },
                            Visibility::Inherited,
                            SpawnRequest,
                        ))
                        .with_children(|parent| {
                            let texture = asset_server.load(IMG_PATH_LOADING_ICON_DECO);
                            let entity = parent
                                .spawn((
                                    ImageNode::new(texture),
                                    Node {
                                        left: Val::VMin(-5.0),
                                        width: Val::VMin(10.0),
                                        height: Val::VMin(10.0),
                                        ..Default::default()
                                    },
                                    Transform::from_scale(Vec3::splat(0.01)),
                                    Visibility::Inherited,
                                    EnterGameLoadingCursor,
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

    let entity = commands
        .spawn((
            Node {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            Transform::from_scale(Vec3::splat(0.01)),
            UI::EnterGameLoadingBar,
            Visibility::Hidden,
            SpawnRequest,
            ZIndex(3),
        ))
        .with_children(|parent| {
            let entity = parent
                .spawn((
                    Node {
                        height: Val::Percent(70.0),
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .id();
            loading_entities.insert(entity);

            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(80.0),
                        height: Val::Percent(7.5),
                        border: UiRect::all(Val::VMin(2.0)),
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Percent(50.0)),
                    BorderColor::all(BORDER_GREEN_COLOR_0),
                    BackgroundColor(Color::BLACK),
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .with_children(|parent| {
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(0.0),
                                height: Val::Percent(100.0),
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(50.0)),
                            BackgroundColor(LOADING_BAR_COLOR),
                            Visibility::Inherited,
                            EnterGameLoadingBar,
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

        commands.insert(EnterGameLevelEntity);
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
        next_state.set(LevelStates::LoadTitle);
    }
}
