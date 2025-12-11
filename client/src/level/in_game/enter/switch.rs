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
            OnEnter(LevelStates::SwitchToLoadGame),
            (
                debug_label,
                setup_scene_timer,
                show_enter_game_interfaces,
                setup_enter_game_patterns,
                setup_enter_game_interfaces,
                play_popup_bobble_sounds,
            ),
        )
        .add_systems(OnExit(LevelStates::SwitchToLoadGame), cleanup_scene_timer)
        .add_systems(
            Update,
            (update_scene_timer, update_enter_game_patterns)
                .run_if(in_state(LevelStates::SwitchToLoadGame)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: SwitchToLoadGame");
}

fn setup_scene_timer(mut commands: Commands) {
    commands.insert_resource(SceneTimer::default());
}

fn show_enter_game_interfaces(
    mut query: Query<&mut Visibility, (With<TitleLevelRoot>, With<EnterGameLevelEntity>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn setup_enter_game_patterns(
    mut commands: Commands,
    query: Query<(Entity, &BackgroundPattern), With<EnterGameLevelEntity>>,
) {
    for (entity, pattern) in query.iter() {
        let delay = pattern.0 as f32 * 0.05;
        commands
            .entity(entity)
            .insert(BackoutScale::new(0.1, Vec3::ZERO, Vec3::ONE).with_delay(delay));
    }
}

fn setup_enter_game_interfaces(
    mut commands: Commands,
    query: Query<Entity, (With<UiAnimationTarget>, With<EnterGameLevelEntity>)>,
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
        next_state.set(LevelStates::LoadGame);
    }
}

fn update_enter_game_patterns(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BackoutScale, &mut Transform), With<EnterGameLevelEntity>>,
    time: Res<Time>,
) {
    for (entity, mut scale, mut transform) in query.iter_mut() {
        scale.tick(time.delta_secs());
        *transform = transform.with_scale(scale.scale());
        if scale.is_finished() {
            commands.entity(entity).remove::<BackoutScale>();
        }
    }
}
