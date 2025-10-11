// Import necessary Bevy modules.
use bevy::prelude::*;
use bevy_spine::{SpineBundle, SpineSync};

use crate::assets::collider::{Collider, ColliderHandle};

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelStates::InitTitle), (debug_label, setup_title))
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
                    play_animation,
                )
                    .run_if(in_state(LevelStates::InitTitle)),
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
) {
    let mut loading_entities = LoadingEntities::default();
    setup_title_screen(
        &mut commands,
        &asset_server,
        &image_assets,
        &mut loading_entities,
    );
    setup_title_interface(&mut commands, &asset_server, &mut loading_entities);

    // --- Resource Insersion ---
    commands.insert_resource(loading_entities);
}

fn setup_title_screen(
    commands: &mut Commands,
    asset_server: &AssetServer,
    image_assets: &Assets<Image>,
    loading_entities: &mut LoadingEntities,
) {
    let handle = asset_server.load(IMG_PATH_BACKGROUND);
    let image = image_assets.get(handle.id()).unwrap();
    let aspect_ratio = image.aspect_ratio().ratio();
    let entity = commands
        .spawn((
            Sprite {
                image: asset_server.load(IMG_PATH_BACKGROUND),
                image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
                custom_size: Some((1920.0, 1920.0 / aspect_ratio).into()),
                ..Default::default()
            },
            Transform::from_xyz(0.0, 540.0, 0.0),
            Visibility::Hidden,
            SpawnRequest,
        ))
        .id();
    loading_entities.insert(entity);

    let entity = commands
        .spawn((
            SpineBundle {
                skeleton: asset_server.load(MODEL_PATH_BUTTER).into(),
                transform: Transform::from_xyz(640.0, 0.0, 1.0).with_scale((1.0, 1.0, 1.0).into()),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            SpineSync,
            ColliderHandle(asset_server.load(COLLIDER_PATH_BUTTER)),
            Character::Butter,
        ))
        .id();
    loading_entities.insert(entity);

    let entity = commands
        .spawn((
            SpineBundle {
                skeleton: asset_server.load(MODEL_PATH_KOMMY).into(),
                transform: Transform::from_xyz(-640.0, 0.0, 1.0)
                    .with_scale((-1.0, 1.0, 1.0).into()),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            SpineSync,
            ColliderHandle(asset_server.load(COLLIDER_PATH_KOMMY)),
            Character::Kommy,
        ))
        .id();
    loading_entities.insert(entity);
}

fn setup_title_interface(
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
            Visibility::Inherited,
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
                    add_vertical_space(loading_entities, parent, Val::Percent(20.0));
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(16.0),
                                ..Default::default()
                            },
                            ZIndex(4),
                            UI::InTitleGameStartButton,
                            Visibility::Hidden,
                            SpawnRequest,
                            Button,
                        ))
                        .with_children(|parent| {
                            create_button(
                                loading_entities,
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
                                        TextFont::from(font),
                                        TextLayout::new_with_justify(Justify::Center),
                                        TextColor::BLACK,
                                        OriginColor(Color::BLACK),
                                        TranslatableText("game_start".into()),
                                        ResizableFont::vertical(1280.0, 52.0),
                                    ));
                                },
                            );
                        })
                        .id();
                    loading_entities.insert(entity);

                    add_vertical_space(loading_entities, parent, Val::Percent(5.0));
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(16.0),
                                ..Default::default()
                            },
                            ZIndex(4),
                            UI::InTitleOptionButton,
                            Visibility::Hidden,
                            SpawnRequest,
                            Button,
                        ))
                        .with_children(|parent| {
                            create_button(
                                loading_entities,
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
                                        Text::new("Settings"),
                                        TextFont::from(font),
                                        TextLayout::new_with_justify(Justify::Center),
                                        TextColor::BLACK,
                                        OriginColor(Color::BLACK),
                                        TranslatableText("game_settings".into()),
                                        ResizableFont::vertical(1280.0, 52.0),
                                    ));
                                },
                            );
                        })
                        .id();
                    loading_entities.insert(entity);

                    add_vertical_space(loading_entities, parent, Val::Percent(5.0));
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(16.0),
                                ..Default::default()
                            },
                            ZIndex(4),
                            UI::InTitleHowToPlayButton,
                            Visibility::Hidden,
                            SpawnRequest,
                            Button,
                        ))
                        .with_children(|parent| {
                            create_button(
                                loading_entities,
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
                                        Text::new("How to play"),
                                        TextFont::from(font),
                                        TextLayout::new_with_justify(Justify::Center),
                                        TextColor::BLACK,
                                        OriginColor(Color::BLACK),
                                        TranslatableText("how_to_play".into()),
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
    collider_assets: Res<Assets<Collider>>,
    mut spine_ready_event: MessageReader<SpineReadyEvent>,
    mut spine_query: Query<(&mut Spine, &Character, &ColliderHandle)>,
) {
    for event in spine_ready_event.read() {
        let (mut spine, character, handle) = spine_query.get_mut(event.entity).unwrap();
        let collider = collider_assets.get(handle.id()).unwrap();
        info!("Character:{:?}, bones:{:?}", character, event.bones.keys());

        let bone_entity = event.bones.get(&collider.ball_bone_name).copied().unwrap();
        let (bone, bone_index) = spine
            .skeleton
            .bones()
            .enumerate()
            .find_map(|(i, b)| (b.data().name() == collider.ball_bone_name).then_some((b, i)))
            .unwrap();
        commands.spawn((
            collider.ball_collider,
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

        let bone_entity = event.bones.get(&collider.head_bone_name).copied().unwrap();
        let (bone, bone_index) = spine
            .skeleton
            .bones()
            .enumerate()
            .find_map(|(i, b)| (b.data().name() == collider.head_bone_name).then_some((b, i)))
            .unwrap();
        commands.entity(bone_entity).insert((
            collider.head_collider,
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
            .insert((CharacterAnimState::Idle, SpawnRequest))
            .remove::<ColliderHandle>();
    }
}
