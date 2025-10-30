// Import necessary Bevy modules.
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::assets::sound::SystemVolume;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::InitOption),
            (debug_label, setup_option),
        )
        .add_systems(OnExit(LevelStates::InitOption), cleanup_loading_resource)
        .add_systems(
            Update,
            (
                update_entity_spawn_progress,
                observe_entity_creation,
                check_loading_progress,
            )
                .run_if(in_state(LevelStates::InitOption)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InitOption");
}

fn setup_option(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
) {
    let mut loading_entities = LoadingEntities::default();
    setup_option_interface(
        &mut commands,
        &asset_server,
        &mut loading_entities,
        &system_volume,
    );

    // --- Resource Insertion ---
    commands.insert_resource(loading_entities);
}

fn setup_option_interface(
    commands: &mut Commands,
    asset_server: &AssetServer,
    loading_entities: &mut LoadingEntities,
    system_volume: &SystemVolume,
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
            Visibility::Hidden,
            UI::Root,
            SpawnRequest,
            ZIndex(3),
        ))
        .with_children(|parent| {
            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(80.0),
                        height: Val::Percent(80.0),
                        border: UiRect::all(Val::VMin(1.25)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Percent(30.0)),
                    BorderColor::all(BORDER_GREEN_COLOR_1),
                    BackgroundColor(BG_GREEN_COLOR_1),
                    Visibility::Inherited,
                    SpawnRequest,
                    UI::Modal,
                ))
                .with_children(|parent| {
                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(50.0),
                                height: Val::Percent(10.0),
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
                                    Text::new("Settings"),
                                    TextFont::from(asset_server.load(FONT_PATH)),
                                    TextLayout::new_with_justify(Justify::Center),
                                    TranslatableText("game_settings".into()),
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

                    add_vertical_space(loading_entities, parent, Val::Percent(5.0));

                    let percentage = system_volume.get_background().to_linear() * 100.0;
                    add_volume_controller(
                        loading_entities,
                        parent,
                        percentage,
                        UI::BackgroundVolumeSlider,
                        Val::Percent(90.0),
                        Val::Percent(10.0),
                        |commands| {
                            let font = asset_server.load(FONT_PATH);
                            commands.insert((
                                Text::new("BGM"),
                                TextFont::from(font),
                                TextLayout::new_with_justify(Justify::Center),
                                TextColor::BLACK,
                                TranslatableText("background_volume".into()),
                                ResizableFont::vertical(1280.0, 54.0),
                                Visibility::Inherited,
                            ));
                        },
                        |commands| {
                            let font = asset_server.load(FONT_PATH);
                            commands.insert((
                                Text::new(format!("{}", percentage as u32)),
                                TextFont::from(font),
                                TextLayout::new_with_justify(Justify::Center),
                                TextColor::BLACK,
                                ResizableFont::vertical(1280.0, 54.0),
                                Visibility::Inherited,
                                UI::BackgroundVolume,
                            ));
                        },
                    );

                    let percentage = system_volume.get_effect().to_linear() * 100.0;
                    add_volume_controller(
                        loading_entities,
                        parent,
                        percentage,
                        UI::EffectVolumeSlider,
                        Val::Percent(90.0),
                        Val::Percent(10.0),
                        |commands| {
                            let font = asset_server.load(FONT_PATH);
                            commands.insert((
                                Text::new("SFX"),
                                TextFont::from(font),
                                TextLayout::new_with_justify(Justify::Center),
                                TextColor::BLACK,
                                TranslatableText("effect_volume".into()),
                                ResizableFont::vertical(1280.0, 54.0),
                                Visibility::Inherited,
                            ));
                        },
                        |commands| {
                            let font = asset_server.load(FONT_PATH);
                            commands.insert((
                                Text::new(format!("{}", percentage as u32)),
                                TextFont::from(font),
                                TextLayout::new_with_justify(Justify::Center),
                                TextColor::BLACK,
                                ResizableFont::vertical(1280.0, 54.0),
                                Visibility::Inherited,
                                UI::EffectVolume,
                            ));
                        },
                    );

                    let percentage = system_volume.get_voice().to_linear() * 100.0;
                    add_volume_controller(
                        loading_entities,
                        parent,
                        percentage,
                        UI::VoiceVolumeSlider,
                        Val::Percent(90.0),
                        Val::Percent(10.0),
                        |commands| {
                            let font = asset_server.load(FONT_PATH);
                            commands.insert((
                                Text::new("Voice"),
                                TextFont::from(font),
                                TextLayout::new_with_justify(Justify::Center),
                                TextColor::BLACK,
                                TranslatableText("voice_volume".into()),
                                ResizableFont::vertical(1280.0, 54.0),
                                Visibility::Inherited,
                            ));
                        },
                        |commands| {
                            let font = asset_server.load(FONT_PATH);
                            commands.insert((
                                Text::new(format!("{}", percentage as u32)),
                                TextFont::from(font),
                                TextLayout::new_with_justify(Justify::Center),
                                TextColor::BLACK,
                                ResizableFont::vertical(1280.0, 54.0),
                                Visibility::Inherited,
                                UI::VoiceVolume,
                            ));
                        },
                    );

                    add_vertical_space(loading_entities, parent, Val::Percent(10.0));

                    add_locale_buttons(
                        asset_server,
                        loading_entities,
                        parent,
                        BORDER_GREEN_COLOR_2,
                        BG_GREEN_COLOR_2,
                        Some(BG_GREEN_COLOR_3),
                        Some(BG_GREEN_COLOR_3),
                        Val::Percent(90.0),
                        Val::Percent(15.0),
                    );

                    add_vertical_space(loading_entities, parent, Val::Percent(10.0));

                    let entity = parent
                        .spawn((
                            Node {
                                width: Val::Percent(24.0),
                                height: Val::Percent(12.0),
                                border: UiRect::all(Val::VMin(1.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderRadius::all(Val::Percent(30.0)),
                            OriginColor::<BackgroundColor>::new(BG_YELLO_COLOR_0),
                            BorderColor::all(BORDER_YELLO_COLOR_0),
                            BackgroundColor(BG_YELLO_COLOR_0),
                            UI::PositiveButton,
                            Visibility::Inherited,
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
                                    ResizableFont::vertical(1280.0, 42.0),
                                    TranslatableText("back".into()),
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
        })
        .id();
    loading_entities.insert(entity);
}

#[allow(clippy::too_many_arguments)]
fn add_volume_controller<LabelFn, VolumeFn>(
    loading_entities: &mut LoadingEntities,
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    percentage: f32,
    slider_handle: UI,
    width: Val,
    height: Val,
    label_func: LabelFn,
    volume_func: VolumeFn,
) where
    LabelFn: FnOnce(&mut EntityCommands<'_>),
    VolumeFn: FnOnce(&mut EntityCommands<'_>),
{
    let entity = parent
        .spawn((
            Node {
                width,
                height,
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
                        width: Val::Percent(25.0),
                        height: Val::Percent(90.0),
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
                    let mut commands =
                        parent.spawn((Node::default(), Visibility::Inherited, SpawnRequest));
                    label_func(&mut commands);
                    loading_entities.insert(commands.id());
                })
                .id();
            loading_entities.insert(entity);

            add_horizontal_space(loading_entities, parent, Val::Percent(2.5));

            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(40.0),
                        height: Val::Percent(15.0),
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
                            Node {
                                width: Val::Percent(100.0),
                                align_content: AlignContent::Center,
                                ..Default::default()
                            },
                            Visibility::Inherited,
                            SpawnRequest,
                        ))
                        .with_children(|parent| {
                            let entity = parent
                                .spawn((
                                    Node {
                                        left: Val::Percent(percentage),
                                        ..Default::default()
                                    },
                                    Visibility::Inherited,
                                    SpawnRequest,
                                ))
                                .with_children(|parent| {
                                    let entity = parent
                                        .spawn((
                                            Node {
                                                left: Val::VMin(-2.5),
                                                width: Val::VMin(5.0),
                                                height: Val::VMin(5.0),
                                                border: UiRect::all(Val::VMin(0.5)),
                                                ..Default::default()
                                            },
                                            OriginColor::<BackgroundColor>::new(BG_GREEN_COLOR_0),
                                            BorderColor::all(BORDER_GREEN_COLOR_0),
                                            BackgroundColor(BG_GREEN_COLOR_0),
                                            BorderRadius::all(Val::Percent(50.0)),
                                            Visibility::Inherited,
                                            SpawnRequest,
                                            Button,
                                            slider_handle,
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

            add_horizontal_space(loading_entities, parent, Val::Percent(2.5));

            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(25.0),
                        height: Val::Percent(90.0),
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
                    let mut commands =
                        parent.spawn((Node::default(), Visibility::Inherited, SpawnRequest));
                    volume_func(&mut commands);
                    loading_entities.insert(commands.id());
                })
                .id();
            loading_entities.insert(entity);
        })
        .id();
    loading_entities.insert(entity);
}

#[allow(clippy::too_many_arguments)]
fn add_locale_buttons(
    asset_server: &AssetServer,
    loading_entities: &mut LoadingEntities,
    parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
    border_color: Color,
    inner_color: Color,
    hoverd_color: Option<Color>,
    pressed_color: Option<Color>,
    width: Val,
    height: Val,
) {
    let entity = parent
        .spawn((
            Node {
                width,
                height,
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
                        width: Val::Percent(22.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::VMin(0.8)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Percent(30.0)),
                    OriginColor::<BackgroundColor>::new(inner_color)
                        .with_hovered(hoverd_color.unwrap_or(inner_color.darker(0.15)))
                        .with_pressed(pressed_color.unwrap_or(inner_color.darker(0.3))),
                    BorderColor::all(border_color),
                    BackgroundColor(inner_color),
                    UI::LocaleButtonEn,
                    Visibility::Inherited,
                    SpawnRequest,
                    Button,
                ))
                .with_children(|parent| {
                    let entity = parent
                        .spawn((
                            Node::default(),
                            Text::new("English"),
                            TextFont::from(asset_server.load(FONT_PATH)),
                            TextLayout::new_with_justify(Justify::Center),
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
            add_horizontal_space(loading_entities, parent, Val::Percent(5.0));

            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(22.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::VMin(0.8)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Percent(30.0)),
                    OriginColor::<BackgroundColor>::new(inner_color)
                        .with_hovered(hoverd_color.unwrap_or(inner_color.darker(0.15)))
                        .with_pressed(pressed_color.unwrap_or(inner_color.darker(0.3))),
                    BorderColor::all(border_color),
                    BackgroundColor(inner_color),
                    UI::LocaleButtonJa,
                    Visibility::Inherited,
                    SpawnRequest,
                    Button,
                ))
                .with_children(|parent| {
                    let entity = parent
                        .spawn((
                            Node::default(),
                            Text::new("日本語"),
                            TextFont::from(asset_server.load(FONT_PATH)),
                            TextLayout::new_with_justify(Justify::Center),
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
            add_horizontal_space(loading_entities, parent, Val::Percent(5.0));

            let entity = parent
                .spawn((
                    Node {
                        width: Val::Percent(22.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::VMin(0.8)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Percent(30.0)),
                    OriginColor::<BackgroundColor>::new(inner_color)
                        .with_hovered(hoverd_color.unwrap_or(inner_color.darker(0.15)))
                        .with_pressed(pressed_color.unwrap_or(inner_color.darker(0.3))),
                    BorderColor::all(border_color),
                    BackgroundColor(inner_color),
                    UI::LocaleButtonKo,
                    Visibility::Inherited,
                    SpawnRequest,
                    Button,
                ))
                .with_children(|parent| {
                    let entity = parent
                        .spawn((
                            Node::default(),
                            Text::new("한국어"),
                            TextFont::from(asset_server.load(FONT_PATH)),
                            TextLayout::new_with_justify(Justify::Center),
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

        commands.insert(OptionLevelEntity);
        if child_of.is_none() {
            commands.insert(OptionLevelRoot);
        }
    }
}

fn check_loading_progress(
    loading_entities: Res<LoadingEntities>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    if loading_entities.is_empty() {
        next_state.set(LevelStates::LoadEnterGame);
    }
}
