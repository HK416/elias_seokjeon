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
            OnEnter(LevelStates::SwitchToInOption),
            (
                debug_label,
                show_option_entities,
                setup_scene_timer,
                setup_option_interfaces,
            ),
        )
        .add_systems(OnExit(LevelStates::SwitchToInOption), cleanup_scene_timer)
        .add_systems(
            Update,
            update_scene_timer.run_if(in_state(LevelStates::SwitchToInOption)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: SwitchToInOption");
}

fn show_option_entities(
    mut query: Query<&mut Visibility, (With<OptionLevelEntity>, With<TitleLevelRoot>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn setup_scene_timer(mut commands: Commands) {
    commands.insert_resource(SceneTimer::default());
}

fn setup_option_interfaces(
    mut commands: Commands,
    query: Query<Entity, (With<UiAnimationTarget>, With<OptionLevelEntity>)>,
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
        next_state.set(LevelStates::InOption);
    }
}
