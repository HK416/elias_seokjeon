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
            OnEnter(LevelStates::SwitchToInGame),
            (
                debug_label,
                setup_scene_timer,
                setup_enter_game_patterns,
                setup_prepare_entities,
                setup_prepare_interface,
                show_ingame_entities,
                setup_ingame_interface,
                play_popup_bobble_sounds,
            ),
        )
        .add_systems(OnExit(LevelStates::SwitchToInGame), cleanup_scene_timer)
        .add_systems(
            Update,
            (
                update_scene_timer,
                update_enter_game_patterns,
                update_prepare_spines,
            )
                .run_if(in_state(LevelStates::SwitchToInGame)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: SwitchToInGame");
}

fn setup_scene_timer(mut commands: Commands) {
    commands.insert_resource(SceneTimer::default());
}

fn setup_enter_game_patterns(
    mut commands: Commands,
    query: Query<(Entity, &BackgroundPattern), With<EnterGameLevelEntity>>,
) {
    for (entity, pattern) in query.iter() {
        let delay = pattern.0 as f32 * 0.05;
        commands
            .entity(entity)
            .insert(SmoothScale::new(0.1, Vec3::ONE, Vec3::ZERO).with_delay(delay));
    }
}

fn setup_prepare_entities(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Spine), With<InPrepareLevelEntity>>,
) {
    for (entity, mut spine) in query.iter_mut() {
        spine.skeleton.color_mut().set_a(1.0);
        commands
            .entity(entity)
            .insert(FadeEffect::new(SCENE_DURATION));
    }
}

#[allow(clippy::type_complexity)]
fn setup_prepare_interface(
    mut commands: Commands,
    query: Query<Entity, (With<UiAnimationTarget>, With<InPrepareLevelEntity>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(UiSmoothScale::new(SCENE_DURATION, Vec2::ONE, Vec2::ZERO));
    }
}

#[allow(clippy::type_complexity)]
fn show_ingame_entities(
    mut query: Query<&mut Visibility, (With<InGameLevelRoot>, With<InGameLevelEntity>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn setup_ingame_interface(
    mut commands: Commands,
    query: Query<Entity, (With<UiAnimationTarget>, With<InGameLevelEntity>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(UiBackOutScale::new(SCENE_DURATION, Vec2::ZERO, Vec2::ONE));
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
        next_state.set(LevelStates::InGame);
    }
}

fn update_enter_game_patterns(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SmoothScale, &mut Transform), With<EnterGameLevelEntity>>,
    time: Res<Time>,
) {
    for (entity, mut scale, mut transform) in query.iter_mut() {
        scale.tick(time.delta_secs());
        *transform = transform.with_scale(scale.scale());
        if scale.is_finished() {
            commands.entity(entity).remove::<SmoothScale>();
        }
    }
}

fn update_prepare_spines(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadeEffect, &mut Spine), With<InPrepareLevelEntity>>,
    time: Res<Time>,
) {
    for (entity, mut fade_in, mut spine) in query.iter_mut() {
        fade_in.tick(time.delta_secs());
        spine.skeleton.color_mut().set_a(1.0 - fade_in.progress());
        if fade_in.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
