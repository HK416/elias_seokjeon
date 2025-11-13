use std::num::NonZeroUsize;

// Import necessary Bevy modules.
use bevy::prelude::*;
use bevy_spine::{SkeletonController, SpineBundle, SpineReadyEvent, SpineSync};

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::InitPrepareGame),
            (debug_label, setup_in_prepare, setup_loading_minimi),
        )
        .add_systems(
            OnExit(LevelStates::InitPrepareGame),
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
                .run_if(in_state(LevelStates::InitPrepareGame)),
        );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            PreUpdate,
            handle_received_packets.run_if(in_state(LevelStates::InitPrepareGame)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InitPrepareGame");
}

fn setup_in_prepare(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_info: Res<PlayerInfo>,
    other_info: Res<OtherInfo>,
) {
    let mut loading_entities = LoadingEntities::default();
    setup_in_prepare_entities(
        &mut commands,
        &asset_server,
        &mut loading_entities,
        &player_info,
        &other_info,
    );
    setup_in_prepare_interface(
        &mut commands,
        &asset_server,
        &mut loading_entities,
        &player_info,
        &other_info,
    );

    // --- Resource Insertion ---
    commands.insert_resource(loading_entities);
}

fn setup_in_prepare_entities(
    commands: &mut Commands,
    asset_server: &AssetServer,
    loading_entities: &mut LoadingEntities,
    player_info: &PlayerInfo,
    other_info: &OtherInfo,
) {
    // Player Hero
    let path = MODEL_PATH_HEROS.get(&player_info.hero).copied().unwrap();
    let entity = commands
        .spawn((
            SpineBundle {
                skeleton: asset_server.load(path).into(),
                transform: Transform::from_xyz(-480.0, 160.0, 1.0)
                    .with_scale(Vec3::new(-1.0, 1.0, 1.0)),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            Character::new(player_info.hero),
            SpineSync,
        ))
        .id();
    loading_entities.insert(entity);

    // Other Hero
    let path = MODEL_PATH_HEROS.get(&other_info.hero).copied().unwrap();
    let entity = commands
        .spawn((
            SpineBundle {
                skeleton: asset_server.load(path).into(),
                transform: Transform::from_xyz(480.0, 160.0, 1.0),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            Character::new(other_info.hero),
            SpineSync,
        ))
        .id();
    loading_entities.insert(entity);
}

fn setup_in_prepare_interface(
    commands: &mut Commands,
    asset_server: &AssetServer,
    loading_entities: &mut LoadingEntities,
    player_info: &PlayerInfo,
    other_info: &OtherInfo,
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
            UI::Root,
            Visibility::Hidden,
            SpawnRequest,
        ))
        .with_children(|parent| {
            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(26.0),
                        height: Val::Percent(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
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
                                height: Val::Percent(30.0),
                                border: UiRect::all(Val::VMin(1.25)),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(30.0)),
                            BorderColor::all(BORDER_GREEN_COLOR_0),
                            BackgroundColor(BG_GREEN_COLOR_0),
                            Visibility::Inherited,
                            SpawnRequest,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(90.0),
                                        height: Val::Percent(26.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    BorderRadius::all(Val::Percent(50.0)),
                                    BackgroundColor(BG_GREEN_COLOR_3),
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .with_children(|parent| {
                                    let entity = parent
                                        .spawn((
                                            Node::default(),
                                            Text::new(format!("{}", player_info.hero)),
                                            TextFont::from(asset_server.load(FONT_PATH)),
                                            TextLayout::new_with_justify(Justify::Center),
                                            TranslatableText(format!("{}", player_info.hero)),
                                            ResizableFont::vertical(1280.0, 32.0),
                                            TextColor::BLACK,
                                            Visibility::Inherited,
                                            SpawnRequest,
                                        ))
                                        .id();
                                    loading_entities.insert(entity);
                                })
                                .id();
                            loading_entities.insert(entity);

                            add_vertical_space(loading_entities, parent, Val::Percent(4.0));

                            let entity = parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(90.0),
                                        height: Val::Percent(60.0),
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
                                            Node::default(),
                                            Text::new(format!("{}pt", player_info.score)),
                                            TextFont::from(asset_server.load(FONT_PATH)),
                                            TextLayout::new_with_justify(Justify::Center),
                                            ResizableFont::vertical(1280.0, 64.0),
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

            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(22.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .with_children(|parent| {
                    let image = asset_server.load(IMG_PATH_FX_FIRECARTOON);
                    let layout = asset_server.load(ATLAS_PATH_FX_FIRECARTOON);
                    let entity = parent
                        .spawn((
                            ImageNode::from_atlas_image(image, TextureAtlas { index: 0, layout }),
                            AnimationTimer::new(1.0, NonZeroUsize::new(10).unwrap(), true),
                            Node {
                                bottom: Val::VMin(-15.0),
                                width: Val::Percent(100.0),
                                height: Val::Auto,
                                aspect_ratio: Some(26.0 / 51.0),
                                position_type: PositionType::Absolute,
                                ..Default::default()
                            },
                            Visibility::Inherited,
                            SpawnRequest,
                        ))
                        .id();
                    loading_entities.insert(entity);

                    let entity = parent
                        .spawn((
                            ImageNode::new(asset_server.load(IMG_PATH_PVP_INGAME_VS)),
                            Node {
                                width: Val::Percent(60.0),
                                height: Val::Auto,
                                aspect_ratio: Some(165.0 / 152.0),
                                position_type: PositionType::Absolute,
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

            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(26.0),
                        height: Val::Percent(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
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
                                height: Val::Percent(30.0),
                                border: UiRect::all(Val::VMin(1.25)),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(30.0)),
                            BorderColor::all(BORDER_GREEN_COLOR_0),
                            BackgroundColor(BG_GREEN_COLOR_0),
                            Visibility::Inherited,
                            SpawnRequest,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(90.0),
                                        height: Val::Percent(24.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    BorderRadius::all(Val::Percent(50.0)),
                                    BackgroundColor(BG_GREEN_COLOR_3),
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .with_children(|parent| {
                                    let entity = parent
                                        .spawn((
                                            Node::default(),
                                            Text::new(format!("{}", other_info.hero)),
                                            TextFont::from(asset_server.load(FONT_PATH)),
                                            TextLayout::new_with_justify(Justify::Center),
                                            TranslatableText(format!("{}", other_info.hero)),
                                            ResizableFont::vertical(1280.0, 32.0),
                                            TextColor::BLACK,
                                            Visibility::Inherited,
                                            SpawnRequest,
                                        ))
                                        .id();
                                    loading_entities.insert(entity);
                                })
                                .id();
                            loading_entities.insert(entity);

                            add_vertical_space(loading_entities, parent, Val::Percent(4.0));

                            let entity = parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(90.0),
                                        height: Val::Percent(60.0),
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
                                            Node::default(),
                                            Text::new(format!("{}pt", other_info.score)),
                                            TextFont::from(asset_server.load(FONT_PATH)),
                                            TextLayout::new_with_justify(Justify::Center),
                                            ResizableFont::vertical(1280.0, 64.0),
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
        loading_entities.remove(entity);

        let mut commands = commands.entity(entity);
        commands.remove::<SpawnRequest>();

        commands.insert(InPrepareLevelEntity);
        if child_of.is_none() {
            commands.insert(InPrepareLevelRoot);
        }
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
            TitleLevelEntity,
            TitleLevelRoot,
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
            TitleLevelEntity,
            TitleLevelRoot,
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
                    .set_animation_by_name(0, BUTTER_TITLE_IDLE, true)
                    .unwrap();
            }
            Character::Kommy => {
                skeleton.set_skin_by_name("Normal").unwrap();
                animation_state
                    .set_animation_by_name(0, KOMMY_TITLE_TAUNT, true)
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
