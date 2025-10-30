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
                show_interface,
                setup_scene_timer,
                setup_ui_animation,
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

fn show_interface(mut query: Query<&mut Visibility, (With<UI>, With<OptionLevelEntity>)>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn setup_scene_timer(mut commands: Commands) {
    commands.insert_resource(SceneTimer::default());
}

fn setup_ui_animation(
    mut commands: Commands,
    query: Query<(Entity, &UI), With<OptionLevelEntity>>,
) {
    for (entity, &ui) in query.iter() {
        match ui {
            UI::Modal => {
                commands.entity(entity).insert(UiBackOutScale::new(
                    SCENE_DURATION,
                    Vec2::ZERO,
                    Vec2::ONE,
                ));
            }
            _ => { /* empty */ }
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
        next_state.set(LevelStates::InOption);
    }
}
