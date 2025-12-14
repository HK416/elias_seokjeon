// Import necessary Bevy modules.
use bevy::prelude::*;
use bevy_spine::{SpineBundle, SpineSync};
use protocol::Hero;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelStates::InitTitle), (debug_label, setup_title))
            .add_systems(
                OnExit(LevelStates::InitTitle),
                (cleanup_loading_resource, cleanup_loading_screen),
            )
            .add_systems(
                Update,
                (
                    update_entity_spawn_progress,
                    observe_entity_creation,
                    check_loading_progress,
                    play_animation,
                )
                    .run_if(in_state(LevelStates::InitTitle)),
            );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            Update,
            packet_receive_loop.run_if(in_state(LevelStates::InitTitle)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InitTitle");
}

fn setup_title(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_assets: Res<Assets<Image>>,
    player_info: Res<PlayerInfo>,
) {
    let mut loading_entities = LoadingEntities::default();
    setup_title_screen(
        &mut commands,
        &asset_server,
        &image_assets,
        &mut loading_entities,
        player_info.hero,
    );
    setup_title_interface(
        &mut commands,
        &asset_server,
        &mut loading_entities,
        &player_info,
    );

    // --- Resource Insersion ---
    commands.insert_resource(loading_entities);
}

fn setup_title_screen(
    commands: &mut Commands,
    asset_server: &AssetServer,
    image_assets: &Assets<Image>,
    loading_entities: &mut LoadingEntities,
    hero: Hero,
) {
    // --- BACKGROUND ---
    let handle = asset_server.load(IMG_PATH_BACKGROUND);
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
                ..Default::default()
            },
            Transform::from_xyz(0.0, half_height, 0.0),
            Visibility::Hidden,
            TitleBackground,
            SpawnRequest,
        ))
        .id();
    loading_entities.insert(entity);

    // --- MODEL ---
    let path = MODEL_PATH_HEROS.get(&hero).copied().unwrap();
    let entity = commands
        .spawn((
            SpineBundle {
                skeleton: asset_server.load(path).into(),
                transform: Transform::from_xyz(385.0, 160.0, 1.0)
                    .with_scale((1.0, 1.0, 1.0).into()),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            Character::from(hero),
            VoiceChannel::MySelf,
            SpineSync,
        ))
        .id();
    loading_entities.insert(entity);
}

fn setup_title_interface(
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
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            Visibility::Hidden,
            SpawnRequest,
        ))
        .with_children(|parent| {
            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(26.0),
                        height: Val::Percent(80.0),
                        flex_direction: FlexDirection::Column,
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
                                width: Val::Percent(90.0),
                                height: Val::Percent(16.0),
                                border: UiRect::all(Val::VMin(1.25)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(50.0)),
                            BorderColor::all(BORDER_GREEN_COLOR_0),
                            BackgroundColor(BG_GREEN_COLOR_3),
                            Visibility::Inherited,
                            SpawnRequest,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Node {
                                        left: Val::Percent(-14.0),
                                        bottom: Val::Percent(-34.0),
                                        ..Default::default()
                                    },
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .with_children(|parent| {
                                    let texture = asset_server.load(IMG_PATH_LABEL_DECO_0);
                                    let entity = parent
                                        .spawn((
                                            ImageNode::new(texture).with_flip_x(),
                                            Node {
                                                height: Val::VMin(8.0),
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

                            add_horizontal_space(loading_entities, parent, Val::Percent(10.0));

                            let entity = parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
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
                                            Text::new(format!(
                                                "Win:{} Lose:{}",
                                                player_info.win, player_info.lose
                                            )),
                                            TextFont::from(asset_server.load(FONT_PATH)),
                                            TextLayout::new_with_justify(Justify::Center),
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

                            add_horizontal_space(loading_entities, parent, Val::Percent(10.0));

                            let entity = parent
                                .spawn((
                                    Node {
                                        right: Val::Percent(-14.0),
                                        bottom: Val::Percent(-34.0),
                                        ..Default::default()
                                    },
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .with_children(|parent| {
                                    let texture = asset_server.load(IMG_PATH_LABEL_DECO_0);
                                    let entity = parent
                                        .spawn((
                                            ImageNode::new(texture),
                                            Node {
                                                height: Val::VMin(8.0),
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
                        })
                        .id();
                    loading_entities.insert(entity);

                    add_vertical_space(loading_entities, parent, Val::Percent(5.0));

                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(16.0),
                                border: UiRect::all(Val::VMin(1.25)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(30.0)),
                            OriginColor::<BackgroundColor>::new(BG_GREEN_COLOR_0),
                            BorderColor::all(BORDER_GREEN_COLOR_0),
                            BackgroundColor(BG_GREEN_COLOR_0),
                            TitleButton::GameStart,
                            Visibility::Inherited,
                            BoxShadow::new(
                                Color::BLACK.with_alpha(0.8),
                                Val::VMin(1.0),
                                Val::VMin(1.0),
                                Val::VMin(1.0),
                                Val::Px(1.0),
                            ),
                            SpawnRequest,
                            Button,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Node::default(),
                                    Text::new("Game Start"),
                                    TextFont::from(asset_server.load(FONT_PATH)),
                                    TextLayout::new_with_justify(Justify::Center),
                                    ResizableFont::vertical(1280.0, 52.0),
                                    TranslatableText("game_start".into()),
                                    OriginColor::<TextColor>::new(Color::BLACK),
                                    TextColor::BLACK,
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .id();
                            loading_entities.insert(entity);
                        })
                        .id();
                    loading_entities.insert(entity);

                    add_vertical_space(loading_entities, parent, Val::Percent(5.0));

                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(16.0),
                                border: UiRect::all(Val::VMin(1.25)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(30.0)),
                            OriginColor::<BackgroundColor>::new(BG_GREEN_COLOR_0),
                            BorderColor::all(BORDER_GREEN_COLOR_0),
                            BackgroundColor(BG_GREEN_COLOR_0),
                            TitleButton::Option,
                            Visibility::Inherited,
                            BoxShadow::new(
                                Color::BLACK.with_alpha(0.8),
                                Val::VMin(1.0),
                                Val::VMin(1.0),
                                Val::VMin(1.0),
                                Val::Px(1.0),
                            ),
                            SpawnRequest,
                            Button,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Node::default(),
                                    Text::new("Settings"),
                                    TextFont::from(asset_server.load(FONT_PATH)),
                                    TextLayout::new_with_justify(Justify::Center),
                                    ResizableFont::vertical(1280.0, 52.0),
                                    TranslatableText("game_settings".into()),
                                    OriginColor::<TextColor>::new(Color::BLACK),
                                    TextColor::BLACK,
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .id();
                            loading_entities.insert(entity);
                        })
                        .id();
                    loading_entities.insert(entity);

                    add_vertical_space(loading_entities, parent, Val::Percent(5.0));

                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(16.0),
                                border: UiRect::all(Val::VMin(1.25)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(30.0)),
                            OriginColor::<BackgroundColor>::new(BG_GREEN_COLOR_0),
                            BorderColor::all(BORDER_GREEN_COLOR_0),
                            BackgroundColor(BG_GREEN_COLOR_0),
                            TitleButton::Ranking,
                            Visibility::Inherited,
                            BoxShadow::new(
                                Color::BLACK.with_alpha(0.8),
                                Val::VMin(1.0),
                                Val::VMin(1.0),
                                Val::VMin(1.0),
                                Val::Px(1.0),
                            ),
                            SpawnRequest,
                            Button,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Node::default(),
                                    Text::new("Ranking"),
                                    TextFont::from(asset_server.load(FONT_PATH)),
                                    TextLayout::new_with_justify(Justify::Center),
                                    ResizableFont::vertical(1280.0, 52.0),
                                    TranslatableText("game_rank".into()),
                                    OriginColor::<TextColor>::new(Color::BLACK),
                                    TextColor::BLACK,
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .id();
                            loading_entities.insert(entity);
                        })
                        .id();
                    loading_entities.insert(entity);

                    add_vertical_space(loading_entities, parent, Val::Percent(5.0));

                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(16.0),
                                border: UiRect::all(Val::VMin(1.25)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(30.0)),
                            OriginColor::<BackgroundColor>::new(BG_GREEN_COLOR_0),
                            BorderColor::all(BORDER_GREEN_COLOR_0),
                            BackgroundColor(BG_GREEN_COLOR_0),
                            TitleButton::HowToPlay,
                            Visibility::Inherited,
                            BoxShadow::new(
                                Color::BLACK.with_alpha(0.8),
                                Val::VMin(1.0),
                                Val::VMin(1.0),
                                Val::VMin(1.0),
                                Val::Px(1.0),
                            ),
                            SpawnRequest,
                            Button,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Node::default(),
                                    Text::new("How to play"),
                                    TextFont::from(asset_server.load(FONT_PATH)),
                                    TextLayout::new_with_justify(Justify::Center),
                                    ResizableFont::vertical(1280.0, 52.0),
                                    TranslatableText("how_to_play".into()),
                                    OriginColor::<TextColor>::new(Color::BLACK),
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

            add_horizontal_space(loading_entities, parent, Val::Percent(14.0));

            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(26.0),
                        height: Val::Percent(90.0),
                        flex_direction: FlexDirection::Row,
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
                                height: Val::Percent(10.0),
                                border: UiRect::all(Val::VMin(1.25)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(50.0)),
                            BorderColor::all(BORDER_GREEN_COLOR_0),
                            BackgroundColor(BG_GREEN_COLOR_3),
                            Visibility::Inherited,
                            SpawnRequest,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Node {
                                        top: Val::Percent(-74.0),
                                        right: Val::Percent(0.0),
                                        ..Default::default()
                                    },
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .with_children(|parent| {
                                    let texture = asset_server.load(IMG_PATH_LABEL_DECO_1);
                                    let entity = parent
                                        .spawn((
                                            ImageNode::new(texture),
                                            Node {
                                                height: Val::VMin(5.0),
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
                                        width: Val::Percent(100.0),
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
                                            ResizableFont::vertical(1280.0, 36.0),
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
                                    Node {
                                        top: Val::Percent(-74.0),
                                        left: Val::Percent(0.0),
                                        ..Default::default()
                                    },
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .with_children(|parent| {
                                    let texture = asset_server.load(IMG_PATH_LABEL_DECO_2);
                                    let entity = parent
                                        .spawn((
                                            ImageNode::new(texture),
                                            Node {
                                                height: Val::VMin(6.2),
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

        commands.insert(TitleLevelEntity);
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

#[allow(unreachable_patterns)]
fn play_animation(
    mut commands: Commands,
    mut spine_ready_event: MessageReader<SpineReadyEvent>,
    mut spine_query: Query<(&mut Spine, &Character)>,
) {
    for event in spine_ready_event.read() {
        let (mut spine, character) = spine_query.get_mut(event.entity).unwrap();
        info!("Character:{:?}, bones:{:?}", character, event.bones.keys());

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

        let hero: Hero = (*character).into();
        let animation_name = TITLE_ANIM[hero as usize];
        skeleton.set_skin_by_name("Normal").unwrap();
        animation_state
            .set_animation_by_name(0, animation_name, true)
            .unwrap();

        commands
            .entity(event.entity)
            .insert((CharacterAnimState::Idle, SpawnRequest));
    }
}
