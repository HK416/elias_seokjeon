// Import necessary Bevy modules.
use bevy::prelude::*;
use bevy_spine::{SkeletonController, SpineBundle, SpineReadyEvent, SpineSync};

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::InitGameResult),
            (debug_label, setup_game_result, setup_loading_minimi),
        )
        .add_systems(
            OnExit(LevelStates::InitGameResult),
            (cleanup_loading_resource, cleanup_sync_flags),
        )
        .add_systems(
            Update,
            (
                update_spawn_progress,
                observe_entiey_creation,
                check_loading_progress.run_if(not(resource_exists::<SyncFlags>)),
                play_animation,
                update_loading_minimi,
            )
                .run_if(in_state(LevelStates::InitGameResult)),
        );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            PreUpdate,
            handle_received_packets.run_if(in_state(LevelStates::InitGameResult)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InitGameResult");
}

fn setup_game_result(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_assets: Res<Assets<Image>>,
    player_info: Res<PlayerInfo>,
) {
    let mut loading_entities = LoadingEntities::default();
    setup_game_result_screen(&mut commands, &asset_server, &mut loading_entities);
    setup_game_result_entities(
        &mut commands,
        &asset_server,
        &mut loading_entities,
        &player_info,
    );
    setup_game_victory_interface(
        &mut commands,
        &asset_server,
        &mut loading_entities,
        &image_assets,
    );
    setup_game_defeat_interface(
        &mut commands,
        &asset_server,
        &mut loading_entities,
        &image_assets,
    );
    setup_game_draw_interface(
        &mut commands,
        &asset_server,
        &mut loading_entities,
        &image_assets,
    );

    // --- Resource Insertion ---
    commands.insert_resource(loading_entities);
}

fn setup_game_result_screen(
    commands: &mut Commands,
    asset_server: &AssetServer,
    loading_entities: &mut LoadingEntities,
) {
    const SIZE: Vec2 = Vec2::splat(240.0);
    const NUM_HORIZONTAL: usize = (WND_WIDTH as f32 / SIZE.x).ceil() as usize;
    const NUM_VERTICAL: usize = (WND_HEIGHT as f32 / SIZE.y).ceil() as usize;

    let handle = asset_server.load(IMG_PATH_PATTERN_0);
    let base_x = -0.5 * NUM_HORIZONTAL as f32 * SIZE.x + 0.5 * SIZE.x;
    let base_y = 0.5 * SIZE.y;
    for r in 0..NUM_VERTICAL {
        for c in 0..NUM_HORIZONTAL {
            let x = base_x + c as f32 * SIZE.x;
            let y = base_y + r as f32 * SIZE.y;
            let entity = commands
                .spawn((
                    Sprite {
                        image: handle.clone(),
                        custom_size: Some(SIZE),
                        image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
                        ..Default::default()
                    },
                    Transform::from_xyz(x, y, 1.0).with_scale(Vec3::ZERO),
                    BackgroundPattern(r + c),
                    Visibility::Hidden,
                    SpawnRequest,
                ))
                .id();
            loading_entities.insert(entity);
        }
    }
}

fn setup_game_result_entities(
    commands: &mut Commands,
    asset_server: &AssetServer,
    loading_entities: &mut LoadingEntities,
    player_info: &PlayerInfo,
) {
    // Player Hero
    let path = MODEL_PATH_HEROS.get(&player_info.hero).copied().unwrap();
    let entity = commands
        .spawn((
            SpineBundle {
                skeleton: asset_server.load(path).into(),
                transform: Transform::from_xyz(0.0, 160.0, 1.0),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            Character::from(player_info.hero),
            SpineSync,
        ))
        .id();
    loading_entities.insert(entity);
}

fn setup_game_victory_interface(
    commands: &mut Commands,
    asset_server: &AssetServer,
    loading_entities: &mut LoadingEntities,
    image_assets: &Assets<Image>,
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
            GameResultVictory,
            SpawnRequest,
        ))
        .with_children(|parent| {
            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(40.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .with_children(|parent| {
                    // --- Icon ---
                    let texture = asset_server.load(IMG_PATH_GAME_RESULT_VICTORY_ICON);
                    let image = image_assets.get(texture.id()).unwrap();
                    let ratio = image.aspect_ratio().ratio();
                    let entity = parent
                        .spawn((
                            ImageNode::new(texture),
                            Node {
                                width: Val::Auto,
                                height: Val::Percent(12.5),
                                aspect_ratio: Some(ratio),
                                ..Default::default()
                            },
                            Visibility::Inherited,
                            UiAnimationTarget,
                            SpawnRequest,
                        ))
                        .id();
                    loading_entities.insert(entity);

                    // --- Bar ---
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(50.0),
                                height: Val::Percent(3.0),
                                border: UiRect::all(Val::VMin(0.75)),
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(50.0)),
                            BorderColor::all(BORDER_GREEN_COLOR_0),
                            BackgroundColor(BG_GREEN_COLOR_0),
                            Visibility::Inherited,
                            UiAnimationTarget,
                            SpawnRequest,
                        ))
                        .id();
                    loading_entities.insert(entity);

                    // --- Padding ---
                    add_vertical_space(loading_entities, parent, Val::Percent(60.0));

                    // --- Game Result ---
                    let texture = asset_server.load(IMG_PATH_GAME_RESULT_VICTORY_TEXT);
                    let image = image_assets.get(texture.id()).unwrap();
                    let ratio = image.aspect_ratio().ratio();
                    let entity = parent
                        .spawn((
                            ImageNode::new(texture),
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Auto,
                                aspect_ratio: Some(ratio),
                                ..Default::default()
                            },
                            Visibility::Inherited,
                            UiAnimationTarget,
                            SpawnRequest,
                        ))
                        .id();
                    loading_entities.insert(entity);

                    // --- Information Text ---
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(10.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            Visibility::Inherited,
                            UiAnimationTarget,
                            SpawnRequest,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Text::new("Press Any Key To Continue"),
                                    TextFont::from(asset_server.load(FONT_PATH)),
                                    TextLayout::new_with_justify(Justify::Center),
                                    ResizableFont::vertical(1280.0, 48.0),
                                    TranslatableText("continue".into()),
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
        })
        .id();
    loading_entities.insert(entity);
}

fn setup_game_defeat_interface(
    commands: &mut Commands,
    asset_server: &AssetServer,
    loading_entities: &mut LoadingEntities,
    image_assets: &Assets<Image>,
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
            GameResultDefeat,
            SpawnRequest,
        ))
        .with_children(|parent| {
            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(40.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .with_children(|parent| {
                    // --- Icon ---
                    let texture = asset_server.load(IMG_PATH_GAME_RESULT_DEFEAT_ICON);
                    let image = image_assets.get(texture.id()).unwrap();
                    let ratio = image.aspect_ratio().ratio();
                    let entity = parent
                        .spawn((
                            ImageNode::new(texture),
                            Node {
                                width: Val::Auto,
                                height: Val::Percent(12.5),
                                aspect_ratio: Some(ratio),
                                ..Default::default()
                            },
                            Visibility::Inherited,
                            UiAnimationTarget,
                            SpawnRequest,
                        ))
                        .id();
                    loading_entities.insert(entity);

                    // --- Bar ---
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(50.0),
                                height: Val::Percent(3.0),
                                border: UiRect::all(Val::VMin(0.75)),
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(50.0)),
                            BorderColor::all(BORDER_BROWN_COLOR_0),
                            BackgroundColor(BG_BROWN_COLOR_0),
                            Visibility::Inherited,
                            UiAnimationTarget,
                            SpawnRequest,
                        ))
                        .id();
                    loading_entities.insert(entity);

                    // --- Padding ---
                    add_vertical_space(loading_entities, parent, Val::Percent(60.0));

                    // --- Game Result ---
                    let texture = asset_server.load(IMG_PATH_GAME_RESULT_DEFEAT_TEXT);
                    let image = image_assets.get(texture.id()).unwrap();
                    let ratio = image.aspect_ratio().ratio();
                    let entity = parent
                        .spawn((
                            ImageNode::new(texture),
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Auto,
                                aspect_ratio: Some(ratio),
                                ..Default::default()
                            },
                            Visibility::Inherited,
                            UiAnimationTarget,
                            SpawnRequest,
                        ))
                        .id();
                    loading_entities.insert(entity);

                    // --- Information Text ---
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(10.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            Visibility::Inherited,
                            UiAnimationTarget,
                            SpawnRequest,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Text::new("Press Any Key To Continue"),
                                    TextFont::from(asset_server.load(FONT_PATH)),
                                    TextLayout::new_with_justify(Justify::Center),
                                    ResizableFont::vertical(1280.0, 48.0),
                                    TranslatableText("continue".into()),
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
        })
        .id();
    loading_entities.insert(entity);
}

fn setup_game_draw_interface(
    commands: &mut Commands,
    asset_server: &AssetServer,
    loading_entities: &mut LoadingEntities,
    image_assets: &Assets<Image>,
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
            GameResultDraw,
            SpawnRequest,
        ))
        .with_children(|parent| {
            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(40.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .with_children(|parent| {
                    // --- Icon ---
                    let texture = asset_server.load(IMG_PATH_GAME_RESULT_VICTORY_ICON);
                    let image = image_assets.get(texture.id()).unwrap();
                    let ratio = image.aspect_ratio().ratio();
                    let entity = parent
                        .spawn((
                            ImageNode::new(texture),
                            Node {
                                width: Val::Auto,
                                height: Val::Percent(12.5),
                                aspect_ratio: Some(ratio),
                                ..Default::default()
                            },
                            Visibility::Inherited,
                            UiAnimationTarget,
                            SpawnRequest,
                        ))
                        .id();
                    loading_entities.insert(entity);

                    // --- Bar ---
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(50.0),
                                height: Val::Percent(3.0),
                                border: UiRect::all(Val::VMin(0.75)),
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(50.0)),
                            BorderColor::all(BORDER_PURPLE_COLOR_0),
                            BackgroundColor(BG_PURPLE_COLOR_0),
                            Visibility::Inherited,
                            UiAnimationTarget,
                            SpawnRequest,
                        ))
                        .id();
                    loading_entities.insert(entity);

                    // --- Padding ---
                    add_vertical_space(loading_entities, parent, Val::Percent(60.0));

                    // --- Game Result ---
                    let texture = asset_server.load(IMG_PATH_GAME_RESULT_DRAW_TEXT);
                    let image = image_assets.get(texture.id()).unwrap();
                    let ratio = image.aspect_ratio().ratio();
                    let entity = parent
                        .spawn((
                            ImageNode::new(texture),
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Auto,
                                aspect_ratio: Some(ratio),
                                ..Default::default()
                            },
                            Visibility::Inherited,
                            UiAnimationTarget,
                            SpawnRequest,
                        ))
                        .id();
                    loading_entities.insert(entity);

                    // --- Information Text ---
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(10.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            Visibility::Inherited,
                            UiAnimationTarget,
                            SpawnRequest,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Text::new("Press Any Key To Continue"),
                                    TextFont::from(asset_server.load(FONT_PATH)),
                                    TextLayout::new_with_justify(Justify::Center),
                                    ResizableFont::vertical(1280.0, 48.0),
                                    TranslatableText("continue".into()),
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
        })
        .id();
    loading_entities.insert(entity);
}

fn setup_loading_minimi(
    mut query: Query<(&mut ImageNode, &mut AnimationTimer), With<EnterGameLevelEntity>>,
) {
    for (mut image_node, mut timer) in query.iter_mut() {
        timer.reset();
        if let Some(atlas) = image_node.texture_atlas.as_mut() {
            atlas.index = timer.frame_index();
        }
    }
}

// --- CLEANUP SYSTEMS ---

fn cleanup_sync_flags(mut commands: Commands) {
    commands.remove_resource::<SyncFlags>();
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
                Packet::PrepareInGame => {
                    next_state.set(LevelStates::SwitchToInPrepare);
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
        let mut commands = commands.entity(entity);
        commands.remove::<SpawnRequest>();

        commands.insert(InGameResultLevelEntity);
        if child_of.is_none() {
            commands.insert(InGameLevelRoot);
        }

        loading_entities.remove(entity);
    }
}

fn check_loading_progress(
    mut commands: Commands,
    loading_entities: Res<LoadingEntities>,
    #[cfg(target_arch = "wasm32")] network: Res<Network>,
) {
    if loading_entities.is_empty() {
        #[cfg(target_arch = "wasm32")]
        network.send(&Packet::GameLoadSuccess).unwrap();
        commands.insert_resource(SyncFlags);
    }
}

#[allow(unreachable_patterns)]
fn play_animation(
    mut commands: Commands,
    mut spine_ready_event: MessageReader<SpineReadyEvent>,
    mut spine_query: Query<(&mut Spine, &Character)>,
) {
    for event in spine_ready_event.read() {
        let (mut spine, character) = spine_query.get_mut(event.entity).unwrap();

        let bone_entity = event.bones.get(BALL_BONE_NAME).copied().unwrap();
        let (bone, bone_index) = spine
            .skeleton
            .bones()
            .enumerate()
            .find_map(|(i, b)| (b.data().name() == BALL_BONE_NAME).then_some((b, i)))
            .unwrap();
        commands.spawn((
            Collider2d::Circle {
                offset: (0.0, 0.0).into(),
                radius: 60.0,
            },
            ColliderType::Ball,
            TargetSpine::new(event.entity),
            TargetSpineBone::new(bone_entity, bone_index),
            SpineBoneOriginPosition {
                local: bone.position().into(),
                world: bone.world_position().into(),
            },
            Transform::IDENTITY,
            GlobalTransform::IDENTITY,
            InGameResultLevelEntity,
            InGameLevelRoot,
        ));

        let bone_entity = event.bones.get(HEAD_BONE_NAME).copied().unwrap();
        let (bone, bone_index) = spine
            .skeleton
            .bones()
            .enumerate()
            .find_map(|(i, b)| (b.data().name() == HEAD_BONE_NAME).then_some((b, i)))
            .unwrap();
        commands.entity(bone_entity).insert((
            Collider2d::Circle {
                offset: (0.0, 0.0).into(),
                radius: 80.0,
            },
            ColliderType::Head,
            TargetSpine::new(event.entity),
            TargetSpineBone::new(bone_entity, bone_index),
            SpineBoneOriginPosition {
                local: bone.position().into(),
                world: bone.world_position().into(),
            },
            Transform::IDENTITY,
            GlobalTransform::IDENTITY,
            InGameResultLevelEntity,
            InGameLevelRoot,
        ));

        let Spine(SkeletonController {
            skeleton,
            animation_state,
            ..
        }) = spine.as_mut();

        match character {
            Character::Butter => {
                skeleton.set_skin_by_name("Normal").unwrap();
                animation_state
                    .set_animation_by_name(0, BUTTER_IDLE, true)
                    .unwrap();
            }
            Character::Kommy => {
                skeleton.set_skin_by_name("Normal").unwrap();
                animation_state
                    .set_animation_by_name(0, KOMMY_IDLE, true)
                    .unwrap();
            }
            _ => { /* empty */ }
        }

        commands
            .entity(event.entity)
            .insert((CharacterAnimState::Idle, SpawnRequest));
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
