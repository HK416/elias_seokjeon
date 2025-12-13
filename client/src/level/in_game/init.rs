use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy_spine::{SkeletonController, SpineBundle, SpineReadyEvent};
use protocol::{
    COLLIDER_DATA, LEFT_PLAYER_POS_X, LEFT_PLAYER_POS_Y, PROJECTILE_SIZE, RIGHT_PLAYER_POS_X,
    RIGHT_PLAYER_POS_Y,
};

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelStates::InitGame), (debug_label, setup_in_game))
            .add_systems(OnExit(LevelStates::InitGame), cleanup_loading_resource)
            .add_systems(
                Update,
                (
                    update_spawn_progress,
                    observe_entiey_creation,
                    check_loading_progress,
                    play_animation,
                    update_loading_minimi,
                )
                    .run_if(in_state(LevelStates::InitGame)),
            );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            PreUpdate,
            handle_received_packets.run_if(in_state(LevelStates::InitGame)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InitGame");
}

fn setup_in_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_assets: Res<Assets<Image>>,
    player_info: Res<PlayerInfo>,
    other_info: Res<OtherInfo>,
) {
    let mut loading_entities = LoadingEntities::default();
    setup_in_game_entities(
        &mut commands,
        &asset_server,
        &mut loading_entities,
        &player_info,
        &other_info,
    );
    setup_in_game_interface(
        &mut commands,
        &asset_server,
        &image_assets,
        &mut loading_entities,
        &player_info,
        &other_info,
    );

    // --- Resource Insertion ---
    commands.insert_resource(loading_entities);
}

fn setup_in_game_entities(
    commands: &mut Commands,
    asset_server: &AssetServer,
    loading_entities: &mut LoadingEntities,
    player_info: &PlayerInfo,
    other_info: &OtherInfo,
) {
    // --- Spawn Stage ---
    let entity = commands
        .spawn((
            Sprite {
                custom_size: Some(Vec2::new(2880.0, 929.53125)),
                image: asset_server.load(IMG_PATH_BG_FAIRY_4),
                image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
                ..Default::default()
            },
            Transform::from_xyz(0.0, 320.0, 0.4),
            Visibility::Visible,
            SpawnRequest,
        ))
        .id();
    loading_entities.insert(entity);

    let entity = commands
        .spawn((
            Sprite {
                custom_size: Some(Vec2::new(2880.0, 582.188)),
                image: asset_server.load(IMG_PATH_BG_FAIRY_3),
                image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
                ..Default::default()
            },
            Transform::from_xyz(0.0, 910.0, 0.3),
            Visibility::Visible,
            SpawnRequest,
        ))
        .id();
    loading_entities.insert(entity);

    let entity = commands
        .spawn((
            Sprite {
                custom_size: Some(Vec2::new(1920.0, 401.25)),
                image: asset_server.load(IMG_PATH_BG_FAIRY_2),
                image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
                ..Default::default()
            },
            Transform::from_xyz(0.0, 840.0, 0.2),
            Visibility::Visible,
            SpawnRequest,
        ))
        .id();
    loading_entities.insert(entity);

    let entity = commands
        .spawn((
            Sprite {
                custom_size: Some(Vec2::new(1920.0, 356.25)),
                image: asset_server.load(IMG_PATH_BG_FAIRY_1),
                image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
                ..Default::default()
            },
            Transform::from_xyz(0.0, 860.0, 0.1),
            Visibility::Visible,
            SpawnRequest,
        ))
        .id();
    loading_entities.insert(entity);

    let entity = commands
        .spawn((
            Sprite {
                custom_size: Some(Vec2::new(1920.0, 390.0)),
                image: asset_server.load(IMG_PATH_BG_FAIRY_0),
                image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
                ..Default::default()
            },
            Transform::from_xyz(0.0, 960.0, 0.0),
            Visibility::Visible,
            SpawnRequest,
        ))
        .id();
    loading_entities.insert(entity);

    // -- Spawn Spine Player ---
    let ((left, left_channel), (right, right_channel)) = if other_info.left_side {
        (
            (other_info.hero, VoiceChannel::Other),
            (player_info.hero, VoiceChannel::MySelf),
        )
    } else {
        (
            (player_info.hero, VoiceChannel::MySelf),
            (other_info.hero, VoiceChannel::Other),
        )
    };

    let path = MODEL_PATH_HEROS.get(&left).copied().unwrap();
    let left_entity = commands
        .spawn((
            SpineBundle {
                skeleton: asset_server.load(path).into(),
                transform: Transform::from_xyz(LEFT_PLAYER_POS_X, LEFT_PLAYER_POS_Y, 0.5)
                    .with_scale(Vec3::new(-0.3, 0.3, 0.3)),
                visibility: Visibility::Visible,
                ..Default::default()
            },
            Character::from(left),
            left_channel,
        ))
        .id();
    loading_entities.insert(left_entity);

    let path = MODEL_PATH_HEROS.get(&right).copied().unwrap();
    let right_entity = commands
        .spawn((
            SpineBundle {
                skeleton: asset_server.load(path).into(),
                transform: Transform::from_xyz(RIGHT_PLAYER_POS_X, RIGHT_PLAYER_POS_Y, 0.5)
                    .with_scale(Vec3::new(0.3, 0.3, 0.3)),
                visibility: Visibility::Visible,
                ..Default::default()
            },
            Character::from(right),
            right_channel,
        ))
        .id();
    loading_entities.insert(right_entity);

    let entity = commands
        .spawn((
            Collider2d::Circle {
                offset: Vec2::ZERO,
                radius: 120.0,
            },
            Transform::from_xyz(LEFT_THROW_POS_X, LEFT_THROW_POS_Y, 0.5),
            Visibility::Visible,
            LeftPlayerTrigger,
            SpawnRequest,
        ))
        .id();
    loading_entities.insert(entity);

    let entity = commands
        .spawn((
            Collider2d::Circle {
                offset: Vec2::ZERO,
                radius: 120.0,
            },
            Transform::from_xyz(RIGHT_THROW_POS_X, RIGHT_THROW_POS_Y, 0.5),
            Visibility::Visible,
            RightPlayerTrigger,
            SpawnRequest,
        ))
        .id();
    loading_entities.insert(entity);

    let circle = COLLIDER_DATA.get(&left).unwrap();
    let entity = commands
        .spawn((
            Collider2d::Circle {
                offset: circle.center.into(),
                radius: circle.radius,
            },
            Transform::from_xyz(LEFT_PLAYER_POS_X, LEFT_PLAYER_POS_Y, 0.5),
            LeftPlayerHead(left_entity),
            Visibility::Visible,
            SpawnRequest,
        ))
        .id();
    loading_entities.insert(entity);

    let circle = COLLIDER_DATA.get(&right).unwrap();
    let entity = commands
        .spawn((
            Collider2d::Circle {
                offset: Vec2::from(circle.center) * Vec2::new(-1.0, 1.0),
                radius: circle.radius,
            },
            Transform::from_xyz(RIGHT_PLAYER_POS_X, RIGHT_PLAYER_POS_Y, 0.5),
            RightPlayerHead(right_entity),
            Visibility::Visible,
            SpawnRequest,
        ))
        .id();
    loading_entities.insert(entity);

    // --- Spawn Projectile ---
    let entity = commands
        .spawn((
            Sprite {
                image: asset_server.load(IMG_PATH_PROJECTILE),
                custom_size: Some(Vec2::splat(PROJECTILE_SIZE)),
                color: Color::WHITE,
                ..Default::default()
            },
            Collider2d::Circle {
                offset: Vec2::ZERO,
                radius: PROJECTILE_SIZE * 0.5,
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            Projectile::default(),
            Visibility::Hidden,
            SpawnRequest,
        ))
        .id();
    loading_entities.insert(entity);
}

fn setup_in_game_interface(
    commands: &mut Commands,
    asset_server: &AssetServer,
    image_assets: &Assets<Image>,
    loading_entities: &mut LoadingEntities,
    player_info: &PlayerInfo,
    other_info: &OtherInfo,
) {
    // --- Spawn Health Bar ---
    let entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            Visibility::Hidden,
            SpawnRequest,
        ))
        .with_children(|parent| {
            // --- Padding ---
            let entity = parent
                .spawn((
                    Node {
                        height: Val::Percent(5.0),
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .id();
            loading_entities.insert(entity);

            // --- Health Bar ---
            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(12.5),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .with_children(|parent| {
                    let texture = asset_server.load(IMG_PATH_HEALTH_HEART);
                    let image = image_assets.get(texture.id()).unwrap();
                    let ratio = image.aspect_ratio().ratio();

                    // --- Spawn Left Health Bar ---
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(30.0),
                                height: Val::Percent(70.0),
                                border: UiRect::all(Val::VMin(1.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(50.0)),
                            BorderColor::all(BORDER_GREEN_COLOR_0),
                            BackgroundColor(BG_GREEN_COLOR_3),
                            Visibility::Inherited,
                            UiAnimationTarget,
                            SpawnRequest,
                        ))
                        .with_children(|parent| {
                            add_health_heart(
                                &texture,
                                ratio,
                                LeftHealth5,
                                parent,
                                loading_entities,
                            );
                            add_horizontal_space(loading_entities, parent, Val::Percent(1.25));
                            add_health_heart(
                                &texture,
                                ratio,
                                LeftHealth4,
                                parent,
                                loading_entities,
                            );
                            add_horizontal_space(loading_entities, parent, Val::Percent(1.25));
                            add_health_heart(
                                &texture,
                                ratio,
                                LeftHealth3,
                                parent,
                                loading_entities,
                            );
                            add_horizontal_space(loading_entities, parent, Val::Percent(1.25));
                            add_health_heart(
                                &texture,
                                ratio,
                                LeftHealth2,
                                parent,
                                loading_entities,
                            );
                            add_horizontal_space(loading_entities, parent, Val::Percent(1.25));
                            add_health_heart(
                                &texture,
                                ratio,
                                LeftHealth1,
                                parent,
                                loading_entities,
                            );
                        })
                        .id();
                    loading_entities.insert(entity);

                    // --- Spawn Timer ---
                    add_horizontal_space(loading_entities, parent, Val::Percent(3.0));
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(10.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::VMin(1.0)),
                                ..Default::default()
                            },
                            BorderColor::all(BORDER_GREEN_COLOR_0),
                            BorderRadius::all(Val::Percent(30.0)),
                            BackgroundColor(BG_GREEN_COLOR_3),
                            Visibility::Inherited,
                            UiAnimationTarget,
                            SpawnRequest,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Text::new("000"),
                                    TextFont::from(asset_server.load(FONT_PATH)),
                                    ResizableFont::vertical(1280.0, 64.0),
                                    TextColor::BLACK,
                                    Visibility::Inherited,
                                    RemainingTimer,
                                    SpawnRequest,
                                ))
                                .id();
                            loading_entities.insert(entity);
                        })
                        .id();
                    loading_entities.insert(entity);
                    add_horizontal_space(loading_entities, parent, Val::Percent(3.0));

                    // --- Spawn Right Health Bar ---
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(30.0),
                                height: Val::Percent(70.0),
                                border: UiRect::all(Val::VMin(1.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(50.0)),
                            BorderColor::all(BORDER_GREEN_COLOR_0),
                            BackgroundColor(BG_GREEN_COLOR_3),
                            Visibility::Inherited,
                            UiAnimationTarget,
                            SpawnRequest,
                        ))
                        .with_children(|parent| {
                            add_health_heart(
                                &texture,
                                ratio,
                                RightHealth1,
                                parent,
                                loading_entities,
                            );
                            add_horizontal_space(loading_entities, parent, Val::Percent(1.25));
                            add_health_heart(
                                &texture,
                                ratio,
                                RightHealth2,
                                parent,
                                loading_entities,
                            );
                            add_horizontal_space(loading_entities, parent, Val::Percent(1.25));
                            add_health_heart(
                                &texture,
                                ratio,
                                RightHealth3,
                                parent,
                                loading_entities,
                            );
                            add_horizontal_space(loading_entities, parent, Val::Percent(1.25));
                            add_health_heart(
                                &texture,
                                ratio,
                                RightHealth4,
                                parent,
                                loading_entities,
                            );
                            add_horizontal_space(loading_entities, parent, Val::Percent(1.25));
                            add_health_heart(
                                &texture,
                                ratio,
                                RightHealth5,
                                parent,
                                loading_entities,
                            );
                        })
                        .id();
                    loading_entities.insert(entity);
                })
                .id();
            loading_entities.insert(entity);

            // --- Turn Timer ---
            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(40.0),
                        height: Val::Percent(6.0),
                        border: UiRect::all(Val::VMin(0.7)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Percent(50.0)),
                    BorderColor::all(BORDER_GREEN_COLOR_0),
                    BackgroundColor(BG_GREEN_COLOR_1),
                    Visibility::Hidden,
                    SpawnRequest,
                    UiTurnTimer,
                ))
                .with_children(|parent| {
                    let texture = asset_server.load(IMG_PATH_INGAME_TIME_ICON);
                    let image = image_assets.get(&texture).unwrap();
                    let ratio = image.aspect_ratio().ratio();
                    let entity = parent
                        .spawn((
                            ImageNode::new(texture),
                            Node {
                                width: Val::Auto,
                                height: Val::Percent(80.0),
                                aspect_ratio: Some(ratio),
                                ..Default::default()
                            },
                            Visibility::Inherited,
                            SpawnRequest,
                        ))
                        .id();
                    loading_entities.insert(entity);

                    add_horizontal_space(loading_entities, parent, Val::Percent(5.0));

                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(80.0),
                                height: Val::Percent(50.0),
                                justify_content: JustifyContent::Start,
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
                                        height: Val::Percent(100.0),
                                        ..Default::default()
                                    },
                                    BorderRadius::all(Val::Percent(50.0)),
                                    BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                                    Visibility::Inherited,
                                    SpawnRequest,
                                    TurnTimer,
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

    // --- Spawn Wind Indicator ---
    let entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
                ..Default::default()
            },
            Visibility::Hidden,
            UiAnimationTarget,
            SpawnRequest,
        ))
        .with_children(|parent| {
            let texture = asset_server.load(IMG_PATH_WIND_INDICATOR_DECO);
            let image = image_assets.get(&texture).unwrap();
            let ratio = image.aspect_ratio().ratio();
            let entity = parent
                .spawn((
                    ImageNode::new(texture),
                    Node {
                        top: Val::VMin(1.5),
                        width: Val::Percent(30.0),
                        height: Val::Auto,
                        aspect_ratio: Some(ratio),
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .id();
            loading_entities.insert(entity);
        })
        .id();
    loading_entities.insert(entity);

    // --- Spawn Wind Indicator Arrow ---
    let entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
                ..Default::default()
            },
            Visibility::Hidden,
            UiAnimationTarget,
            SpawnRequest,
        ))
        .with_children(|parent| {
            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(15.0),
                        height: Val::Auto,
                        aspect_ratio: Some(1.0),
                        border: UiRect::all(Val::VMin(1.25)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderColor::all(BORDER_GREEN_COLOR_0),
                    BorderRadius::all(Val::Percent(50.0)),
                    BackgroundColor(BG_GREEN_COLOR_3),
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .with_children(|parent| {
                    let texture = asset_server.load(IMG_PATH_WIND_INDICATOR_ARROW);
                    let image = image_assets.get(&texture).unwrap();
                    let ratio = image.aspect_ratio().ratio();
                    let entity = parent
                        .spawn((
                            ImageNode::new(texture),
                            Node {
                                width: Val::Percent(70.0),
                                height: Val::Auto,
                                aspect_ratio: Some(ratio),
                                ..Default::default()
                            },
                            Visibility::Inherited,
                            WindIndicator,
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

    // --- Spawn ID Panel ---
    let entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
                ..Default::default()
            },
            Visibility::Hidden,
            UiAnimationTarget,
            SpawnRequest,
        ))
        .with_children(|parent| {
            let handle = asset_server.load(IMG_PATH_ID_PANEL);
            let atlas = asset_server.load(ATLAS_PATH_ID_PANEL);

            let entity = parent
                .spawn((
                    ImageNode {
                        image: handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: atlas.clone(),
                            index: 0,
                        }),
                        ..Default::default()
                    },
                    Node {
                        width: Val::Auto,
                        height: Val::Percent(10.0),
                        aspect_ratio: Some(60.0 / 106.0),
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .with_children(|parent| {
                    if !other_info.left_side {
                        let entity = parent
                            .spawn((
                                ImageNode::new(asset_server.load(IMG_PATH_RED_DOT)),
                                Node {
                                    top: Val::Px(0.0),
                                    right: Val::Px(0.0),
                                    width: Val::VMin(5.0),
                                    height: Val::VMin(5.0),
                                    ..Default::default()
                                },
                                Visibility::Inherited,
                                SpawnRequest,
                            ))
                            .id();
                        loading_entities.insert(entity);
                    }
                })
                .id();
            loading_entities.insert(entity);

            let entity = parent
                .spawn((
                    ImageNode {
                        image: handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: atlas.clone(),
                            index: 1,
                        }),
                        ..Default::default()
                    },
                    Node {
                        width: Val::Percent(20.0),
                        height: Val::Percent(10.0),
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
                            Text::new(match other_info.left_side {
                                true => &other_info.name,
                                false => &player_info.name,
                            }),
                            TextFont::from(asset_server.load(FONT_PATH)),
                            ResizableFont::vertical(1280.0, 48.0),
                            TextColor::BLACK,
                            Visibility::Inherited,
                            SpawnRequest,
                        ))
                        .id();
                    loading_entities.insert(entity);
                })
                .id();
            loading_entities.insert(entity);

            let entity = parent
                .spawn((
                    ImageNode {
                        image: handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: atlas.clone(),
                            index: 2,
                        }),
                        ..Default::default()
                    },
                    Node {
                        width: Val::Auto,
                        height: Val::Percent(10.0),
                        aspect_ratio: Some(60.0 / 106.0),
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .id();
            loading_entities.insert(entity);

            add_horizontal_space(loading_entities, parent, Val::Percent(30.0));

            let entity = parent
                .spawn((
                    ImageNode {
                        image: handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: atlas.clone(),
                            index: 0,
                        }),
                        ..Default::default()
                    },
                    Node {
                        width: Val::Auto,
                        height: Val::Percent(10.0),
                        aspect_ratio: Some(60.0 / 106.0),
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .with_children(|parent| {
                    if other_info.left_side {
                        let entity = parent
                            .spawn((
                                ImageNode::new(asset_server.load(IMG_PATH_RED_DOT)),
                                Node {
                                    top: Val::Px(0.0),
                                    right: Val::Px(0.0),
                                    width: Val::VMin(5.0),
                                    height: Val::VMin(5.0),
                                    ..Default::default()
                                },
                                Visibility::Inherited,
                                SpawnRequest,
                            ))
                            .id();
                        loading_entities.insert(entity);
                    }
                })
                .id();
            loading_entities.insert(entity);

            let entity = parent
                .spawn((
                    ImageNode {
                        image: handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: atlas.clone(),
                            index: 1,
                        }),
                        ..Default::default()
                    },
                    Node {
                        width: Val::Percent(20.0),
                        height: Val::Percent(10.0),
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
                            Text::new(match other_info.left_side {
                                true => &player_info.name,
                                false => &other_info.name,
                            }),
                            TextFont::from(asset_server.load(FONT_PATH)),
                            ResizableFont::vertical(1280.0, 48.0),
                            TextColor::BLACK,
                            Visibility::Inherited,
                            SpawnRequest,
                        ))
                        .id();
                    loading_entities.insert(entity);
                })
                .id();
            loading_entities.insert(entity);

            let entity = parent
                .spawn((
                    ImageNode {
                        image: handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: atlas.clone(),
                            index: 2,
                        }),
                        ..Default::default()
                    },
                    Node {
                        width: Val::Auto,
                        height: Val::Percent(10.0),
                        aspect_ratio: Some(60.0 / 106.0),
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .id();
            loading_entities.insert(entity);
        })
        .id();
    loading_entities.insert(entity);
}

fn add_health_heart<T: Component>(
    texture: &Handle<Image>,
    ratio: f32,
    tag: T,
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    loading_entities: &mut LoadingEntities,
) {
    let entity = parent
        .spawn((
            Node {
                width: Val::Auto,
                height: Val::Percent(80.0),
                aspect_ratio: Some(ratio),
                ..Default::default()
            },
            ImageNode::new(texture.clone()),
            Visibility::Inherited,
            UiAnimationTarget,
            SpawnRequest,
            tag,
        ))
        .id();
    loading_entities.insert(entity);
}

// --- PREUPDATE SYSTEMS ---

#[cfg(target_arch = "wasm32")]
fn handle_received_packets(
    mut commands: Commands,
    mut next_state: ResMut<NextState<LevelStates>>,
    network: Res<Network>,
) {
    for result in network.try_iter() {
        match result {
            Ok(packet) => match packet {
                Packet::GameLoadTimeout => {
                    commands.insert_resource(ErrorMessage::new(
                        "game_load_timeout",
                        "Failed to enter the game due to a connection timeout.",
                    ));
                    next_state.set(LevelStates::SwitchToTitleMessage);
                }
                _ => { /* empty */ }
            },
            Err(e) => {
                commands.insert_resource(ErrorMessage::from(e));
                next_state.set(LevelStates::Error);
            }
        }
    }
}

// --- UPDATE SYSTEMS ---

#[allow(clippy::type_complexity)]
fn update_spawn_progress(
    loading_assets: Res<LoadingEntities>,
    mut set: ParamSet<(
        Query<&mut Node, With<EnterGameLoadingBar>>,
        Query<&mut Node, With<EnterGameLoadingCursor>>,
    )>,
) {
    let progress = loading_assets.percent();

    if let Ok(mut node) = set.p0().single_mut() {
        node.width = Val::Percent(progress * 100.0);
    }

    if let Ok(mut node) = set.p1().single_mut() {
        node.left = Val::Percent(progress * 100.0);
    }
}

fn observe_entiey_creation(
    mut commands: Commands,
    mut loading_entities: ResMut<LoadingEntities>,
    query: Query<(Entity, Option<&ChildOf>), Added<SpawnRequest>>,
) {
    for (entity, child_of) in query.iter() {
        loading_entities.remove(entity);

        let mut commands = commands.entity(entity);
        commands.remove::<SpawnRequest>();

        commands.insert(InGameLevelEntity);
        if child_of.is_none() {
            commands.insert(InGameLevelRoot);
        }
    }
}

fn check_loading_progress(
    loading_entities: Res<LoadingEntities>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    if loading_entities.is_empty() {
        next_state.set(LevelStates::InitPrepareGame);
    }
}

#[allow(unreachable_patterns)]
fn play_animation(
    mut commands: Commands,
    mut spine_ready_event: MessageReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine>,
) {
    for event in spine_ready_event.read() {
        let mut spine = spine_query.get_mut(event.entity).unwrap();
        let Spine(SkeletonController {
            skeleton,
            animation_state,
            ..
        }) = spine.as_mut();

        skeleton.set_skin_by_name("Normal").unwrap();
        animation_state
            .set_animation_by_name(0, IDLE, true)
            .unwrap();

        commands
            .entity(event.entity)
            .insert((CharacterAnimState::InGame, SpawnRequest));
    }
}

fn update_loading_minimi(
    mut query: Query<(&mut ImageNode, &mut AnimationTimer), With<EnterGameLevelEntity>>,
    time: Res<Time>,
) {
    for (mut image_node, mut timer) in query.iter_mut() {
        timer.tick(time.delta_secs());
        if let Some(atlas) = image_node.texture_atlas.as_mut() {
            atlas.index = timer.frame_index();
        }
    }
}
