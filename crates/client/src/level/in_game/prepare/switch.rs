// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- CONSTANTS ---
const SCENE_DURATION: f32 = 0.7;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::SwitchToInPrepare),
            (
                debug_label,
                setup_scene_timer,
                setup_enter_game_entities,
                show_prepare_entities,
                setup_prepare_interfaces,
                setup_prepare_spines,
                setup_pvp_vs_fire_effect,
            ),
        )
        .add_systems(OnExit(LevelStates::SwitchToInPrepare), cleanup_scene_timer)
        .add_systems(
            Update,
            (
                update_scene_timer,
                update_prepare_spines,
                update_prepare_interface,
            )
                .run_if(in_state(LevelStates::SwitchToInPrepare)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: SwitchToInPrepare");
}

fn setup_scene_timer(mut commands: Commands) {
    commands.insert_resource(SceneTimer::default());
}

fn setup_enter_game_entities(
    mut commands: Commands,
    query: Query<Entity, (With<UiAnimationTarget>, With<EnterGameLevelEntity>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(UiSmoothScale::new(SCENE_DURATION, Vec2::ONE, Vec2::ZERO));
    }
}

fn show_prepare_entities(
    mut query: Query<&mut Visibility, (With<InPrepareLevelEntity>, With<InGameLevelRoot>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn setup_prepare_interfaces(
    mut commands: Commands,
    query: Query<Entity, (With<UiAnimationTarget>, With<InPrepareLevelEntity>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(UiBackOutScale::new(SCENE_DURATION, Vec2::ZERO, Vec2::ONE));
    }
}

#[allow(clippy::type_complexity)]
fn setup_prepare_spines(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Spine), With<InPrepareLevelEntity>>,
) {
    for (entity, mut spine) in query.iter_mut() {
        spine.skeleton.color_mut().set_a(0.0);
        commands
            .entity(entity)
            .insert(FadeEffect::new(SCENE_DURATION));
    }
}

fn setup_pvp_vs_fire_effect(
    mut query: Query<(&mut ImageNode, &mut AnimationTimer), With<InPrepareLevelEntity>>,
) {
    for (mut image_node, mut timer) in query.iter_mut() {
        timer.reset();
        if let Some(atlas) = image_node.texture_atlas.as_mut() {
            atlas.index = timer.frame_index();
        }
    }
}

// --- CLEANUP SYSTEMS ---

fn cleanup_scene_timer(mut commands: Commands) {
    commands.remove_resource::<SceneTimer>();
}

// --- UPDATE SYSTEMS ---

fn update_scene_timer(
    mut next_state: ResMut<NextState<LevelStates>>,
    mut scene_timer: ResMut<SceneTimer>,
    time: Res<Time>,
) {
    scene_timer.tick(time.delta_secs());
    if scene_timer.elapsed_sec() >= SCENE_DURATION {
        next_state.set(LevelStates::InPrepareGame);
    }
}

fn update_prepare_spines(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadeEffect, &mut Spine)>,
    time: Res<Time>,
) {
    for (entity, mut fade_in, mut spine) in query.iter_mut() {
        fade_in.tick(time.delta_secs());
        spine.skeleton.color_mut().set_a(fade_in.progress());
        if fade_in.is_finished() {
            commands.entity(entity).remove::<FadeEffect>();
        }
    }
}

fn update_prepare_interface(
    mut query: Query<(&mut ImageNode, &mut AnimationTimer), With<InPrepareLevelEntity>>,
    time: Res<Time>,
) {
    for (mut image_node, mut timer) in query.iter_mut() {
        timer.tick(time.delta_secs());
        if let Some(atlas) = image_node.texture_atlas.as_mut() {
            atlas.index = timer.frame_index();
        }
    }
}
