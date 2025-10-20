// Import necessary Bevy modules.
use bevy::prelude::*;
use bevy_spine::{Spine, SpineEvent};

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

pub fn handle_spine_animation_completed(
    mut spine_events: MessageReader<SpineEvent>,
    mut spine_query: Query<(&mut Spine, &Character, &mut CharacterAnimState)>,
) {
    for event in spine_events.read() {
        if let SpineEvent::Complete { entity, animation } = event
            && let Ok((mut spine, character, mut anim_state)) = spine_query.get_mut(*entity)
            && let Some(track) = spine.animation_state.get_current(0)
        {
            if track.animation().name() != animation {
                continue;
            }

            *anim_state = match *anim_state {
                CharacterAnimState::PatEnd
                | CharacterAnimState::TouchEnd
                | CharacterAnimState::SmashEnd2 => CharacterAnimState::Idle,
                CharacterAnimState::SmashEnd1 => CharacterAnimState::SmashEnd2,
                _ => continue,
            };
            play_character_animation(&mut spine, *character, *anim_state);
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn update_wave_animation(
    mut commands: Commands,
    mut spine_query: Query<(&mut Spine, &CharacterAnimState)>,
    mut wave_anim_query: Query<(
        Entity,
        &TargetSpine,
        &TargetSpineBone,
        &SpineBoneOriginPosition,
        &mut BallWaveAnimation,
    )>,
    time: Res<Time>,
) {
    for (entity, target_spine, target_spine_bone, origin_position, mut wave_anim) in
        wave_anim_query.iter_mut()
    {
        wave_anim.elapsed += time.delta_secs();
        let t = (wave_anim.elapsed / BALL_WAVE_DURATION).min(1.0);
        let delta = normalized_wave(t, 0.5, 1.0, 5.0, PI);

        if let Ok((mut spine, anim_state)) = spine_query.get_mut(target_spine.entity)
            && let Some(mut bone) = spine.skeleton.bone_at_index_mut(target_spine_bone.index)
        {
            if matches!(*anim_state, CharacterAnimState::TouchIdle) {
                bone.set_position(origin_position.local);
                spine.skeleton.update_world_transform();
                commands.entity(entity).remove::<BallWaveAnimation>();
                continue;
            }

            bone.set_position(
                origin_position.local + wave_anim.direction.yx() * delta * wave_anim.power,
            );
            spine.skeleton.update_world_transform();
        }

        if wave_anim.elapsed > BALL_WAVE_DURATION {
            commands.entity(entity).remove::<BallWaveAnimation>();
        }
    }
}
