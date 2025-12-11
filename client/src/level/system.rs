// Import necessary Bevy modules.
use bevy::{audio::PlaybackMode, prelude::*};

use crate::assets::sound::SystemVolume;

use super::*;

// --- SETUP SYSTEMS ---

pub fn setup_timeout_retry(mut commands: Commands) {
    commands.insert_resource(SceneTimer::default());
    commands.insert_resource(RetryCounter::default());
}

pub fn setup_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, LoadingStateRoot));

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Vw(20.0),
                height: Val::Vh(5.0),
                bottom: Val::Vh(3.0),
                right: Val::Vw(3.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            LoadingStateRoot,
        ))
        .with_children(|parent| {
            // Container for the loading text.
            parent
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(88.0),
                    ..Default::default()
                },))
                .with_children(|parent| {
                    // Spawn the "Now Loading..." text element.
                    let font = asset_server.load(FONT_PATH);
                    parent.spawn((
                        Text::new("Now Loading..."),
                        TextFont::from(font).with_font_size(24.0),
                        TextLayout::new_with_justify(Justify::Center),
                        TextColor::WHITE,
                        ResizableFont::vertical(1280.0, 24.0),
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            overflow: Overflow::hidden(),
                            ..Default::default()
                        },
                        LoadingText,
                        ZIndex(2),
                    ));
                });

            // Container for the loading progress bar.
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(12.0),
                        border: UiRect::all(Val::Percent(0.25)),
                        ..Default::default()
                    },
                    BorderColor::all(Color::WHITE),
                    BorderRadius::all(Val::Percent(50.0)),
                ))
                .with_children(|parent| {
                    // The actual loading bar that will be filled.
                    parent.spawn((
                        Node {
                            width: Val::Percent(0.0),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },
                        BorderRadius::all(Val::Percent(50.0)),
                        BackgroundColor(LOADING_BAR_COLOR),
                        LoadingBar,
                        ZIndex(1),
                    ));
                });
        });
}

// --- CLEANUP SYSTEMS ---

pub fn cleanup_timeout_retry(mut commands: Commands) {
    commands.remove_resource::<SceneTimer>();
    commands.remove_resource::<RetryCounter>();
}

pub fn cleanup_loading_screen(
    mut commands: Commands,
    query: Query<Entity, With<LoadingStateRoot>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_loading_resource(mut commands: Commands) {
    commands.remove_resource::<LoadingEntities>();
}

// --- UPDATE SYSTEMS ---

#[cfg(target_arch = "wasm32")]
pub fn packet_receive_loop(
    mut commands: Commands,
    mut next_state: ResMut<NextState<LevelStates>>,
    network: Res<Network>,
) {
    for result in network.receiver.try_iter() {
        if let Err(e) = result {
            commands.insert_resource(ErrorMessage::from(e));
            next_state.set(LevelStates::Error);
            return;
        }
    }
}

pub fn update_asset_loading_progress<T: AssetGroup>(
    asset_server: Res<AssetServer>,
    loading_assets: Res<T>,
    mut query: Query<&mut Node, With<LoadingBar>>,
) {
    let Ok(mut node) = query.single_mut() else {
        return;
    };
    let loaded_count = loading_assets
        .ids()
        .iter()
        .filter(|&&id| asset_server.is_loaded_with_dependencies(id))
        .count();

    let total_count = loading_assets.len();
    let progress = if total_count > 0 {
        loaded_count as f32 / total_count as f32
    } else {
        1.0
    };

    node.width = Val::Percent(progress * 100.0);
}

pub fn update_entity_spawn_progress(
    loading_assets: Res<LoadingEntities>,
    mut query: Query<&mut Node, With<LoadingBar>>,
) {
    let Ok(mut node) = query.single_mut() else {
        return;
    };

    let progress = loading_assets.percent();
    node.width = Val::Percent(progress * 100.0);
}

pub fn play_popup_sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
) {
    commands.spawn((
        AudioPlayer::new(asset_server.load(SFX_PATH_COMMON_POPUP_TOAST_MESSAGE)),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: system_volume.get_effect(),
            ..Default::default()
        },
        EffectSound,
    ));
}

pub fn play_popup_bobble_sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
) {
    commands.spawn((
        AudioPlayer::new(asset_server.load(SFX_PATH_POPUP_BOBBLE)),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: system_volume.get_effect(),
            ..Default::default()
        },
        EffectSound,
    ));
}

pub fn play_popup_close_sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
) {
    commands.spawn((
        AudioPlayer::new(asset_server.load(SFX_PATH_COMMON_POPUP_CLOSE)),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: system_volume.get_effect(),
            ..Default::default()
        },
        EffectSound,
    ));
}

pub fn play_in_game_defeat_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    query: Query<Entity, With<BackgroundSound>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    commands.spawn((
        AudioPlayer::new(asset_server.load(BGM_PATH_INGAME_DEFEAT)),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: system_volume.get_effect(),
            ..Default::default()
        },
        BackgroundSound,
    ));
}

pub fn play_in_game_victory_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    query: Query<Entity, With<BackgroundSound>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    commands.spawn((
        AudioPlayer::new(asset_server.load(BGM_PATH_INGAME_VICTORY)),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: system_volume.get_effect(),
            ..Default::default()
        },
        BackgroundSound,
    ));
}

pub fn update_grabbed_timer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    mut spine_query: Query<(&mut Spine, &Character, &mut CharacterAnimState)>,
    mut grabbed_query: Query<(&TargetSpine, &ColliderType, &mut Grabbed)>,
    time: Res<Time>,
) {
    for (target_spine, ty, mut grabbed) in grabbed_query.iter_mut() {
        grabbed.elapsed += time.delta_secs();
        if matches!(ty, ColliderType::Head)
            && grabbed.elapsed > GRABBED_TIME_THRESHOLD
            && let Ok((mut spine, character, mut anim_state)) =
                spine_query.get_mut(target_spine.entity)
            && !matches!(*anim_state, CharacterAnimState::PatIdle)
        {
            let source = asset_server.load(SFX_PATH_COMMON_RUBBING);
            play_effect_sound(&mut commands, &system_volume, source);

            *anim_state = CharacterAnimState::PatIdle;
            play_character_animation(&mut spine, *character, *anim_state);
        }
    }
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn added_grabbed_component(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    bone_query: Query<&GlobalTransform>,
    mut spine_query: Query<(&mut Spine, &Character, &mut CharacterAnimState)>,
    mut grabbed_query: Query<
        (
            &TargetSpine,
            &TargetSpineBone,
            &ColliderType,
            &mut SpineBoneOriginPosition,
        ),
        Added<Grabbed>,
    >,
) {
    for (target_spine, target_spine_bone, ty, mut origin_position) in grabbed_query.iter_mut() {
        if let Ok((mut spine, character, mut anim_state)) = spine_query.get_mut(target_spine.entity)
            && let Ok(transform) = bone_query.get(target_spine_bone.entity)
        {
            match ty {
                ColliderType::Ball => {
                    let source = asset_server.load(SFX_PATH_COMMON_PULL_CHEEK);
                    play_effect_sound(&mut commands, &system_volume, source);

                    origin_position.world = transform.translation().xy();
                    *anim_state = CharacterAnimState::TouchIdle;
                    play_character_animation(&mut spine, *character, *anim_state);
                }
                _ => { /* empty */ }
            };
        }
    }
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn removed_grabbed_component(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut entities: RemovedComponents<Grabbed>,
    mut spine_query: Query<(&mut Spine, &Character, &mut CharacterAnimState)>,
    grabbed_query: Query<(Entity, &TargetSpine, &TargetSpineBone, &ColliderType)>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    for entity in entities.read() {
        if let Ok((entity, target_spine, target_spine_bone, ty)) = grabbed_query.get(entity)
            && let Ok((mut spine, character, mut anim_state)) =
                spine_query.get_mut(target_spine.entity)
        {
            match ty {
                ColliderType::Ball => {
                    let source = asset_server.load(SFX_PATH_COMMON_PULL_CHEEK_END);
                    play_effect_sound(&mut commands, &system_volume, source);

                    *anim_state = CharacterAnimState::TouchEnd;
                    play_character_animation(&mut spine, *character, *anim_state);

                    if let Some(cursor_viewport_position) = window.cursor_position()
                        && let Ok(point) =
                            camera.viewport_to_world_2d(camera_transform, cursor_viewport_position)
                        && let Some(bone) = spine.skeleton.bone_at_index(target_spine_bone.index)
                    {
                        let w_bone_position: Vec2 = bone.world_position().into();
                        let distance = point - w_bone_position;
                        let length = distance.length();
                        if length > f32::EPSILON {
                            commands.entity(entity).insert(BallWaveAnimation {
                                elapsed: 0.0,
                                direction: distance / length,
                                power: length.min(BALL_MOVE_RANGE * 0.5),
                            });
                        }
                    }
                }
                ColliderType::Head => {
                    if matches!(*anim_state, CharacterAnimState::PatIdle) {
                        let source = asset_server.load(SFX_PATH_COMMON_RUBBING_END);
                        play_effect_sound(&mut commands, &system_volume, source);

                        *anim_state = CharacterAnimState::PatEnd;
                        play_character_animation(&mut spine, *character, *anim_state);
                    } else {
                        let source = asset_server.load(SFX_PATH_EMOTICON_HIT);
                        play_effect_sound(&mut commands, &system_volume, source);

                        *anim_state = CharacterAnimState::SmashEnd1;
                        play_character_animation(&mut spine, *character, *anim_state);
                    }
                }
            }
        }
    }
}

pub fn update_spine_bone_position(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut spine_query: Query<(&mut Spine, &GlobalTransform)>,
    mut grabbed_query: Query<
        (
            &TargetSpine,
            &TargetSpineBone,
            &SpineBoneOriginPosition,
            &ColliderType,
        ),
        With<Grabbed>,
    >,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    for (target_spine, target_spine_bone, origin_position, ty) in grabbed_query.iter_mut() {
        if matches!(ty, ColliderType::Ball)
            && let Ok((mut spine, transform)) = spine_query.get_mut(target_spine.entity)
            && let Some(mut bone) = spine.skeleton.bone_at_index_mut(target_spine_bone.index)
            && let Some(cursor_viewport_position) = window.cursor_position()
            && let Ok(point) =
                camera.viewport_to_world_2d(camera_transform, cursor_viewport_position)
        {
            let w_bone_position = origin_position.world;
            let distance = point - w_bone_position;
            let length = distance.length();
            let offset = vec2(1.0, -transform.scale().x);
            if length > f32::EPSILON {
                bone.set_position(
                    origin_position.local
                        + distance.yx() * offset / length * length.min(BALL_MOVE_RANGE),
                );
            } else {
                bone.set_position(w_bone_position);
            }
        }
    }
}
