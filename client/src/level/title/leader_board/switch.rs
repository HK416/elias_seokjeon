// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- CONSTANTS ---
const SCENE_DURATION: f32 = UI_POPUP_DURATION;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::SwitchToLeaderBoard),
            (
                debug_label,
                setup_scene_timer,
                show_leaderboard_entities,
                setup_leaderboard_interfaces,
                setup_loaderboard_contents,
                play_popup_sounds,
            ),
        )
        .add_systems(
            OnExit(LevelStates::SwitchToLeaderBoard),
            cleanup_scene_timer,
        )
        .add_systems(
            Update,
            update_scene_timer.run_if(in_state(LevelStates::SwitchToLeaderBoard)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: SwitchToLeaderBoard");
}

fn setup_scene_timer(mut commands: Commands) {
    commands.insert_resource(SceneTimer::default());
}

fn show_leaderboard_entities(
    mut query: Query<&mut Visibility, (With<LeaderBoardLevelEntity>, With<TitleLevelRoot>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn setup_leaderboard_interfaces(
    mut commands: Commands,
    query: Query<Entity, (With<UiAnimationTarget>, With<OptionLevelEntity>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(UiBackOutScale::new(SCENE_DURATION, Vec2::ZERO, Vec2::ONE));
    }
}

fn setup_loaderboard_contents() {}

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
        next_state.set(LevelStates::LeaderBoard);
    }
}
