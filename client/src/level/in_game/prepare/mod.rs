mod init;
mod switch;

// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- CONSTANTS ---

pub const SCENE_DURATION: f32 = 3.0;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(init::InnerPlugin)
            .add_plugins(switch::InnerPlugin)
            .add_systems(
                OnEnter(LevelStates::InPrepareGame),
                (debug_label, hide_loading_interfaces, setup_scene_timer),
            )
            .add_systems(OnExit(LevelStates::InPrepareGame), cleanup_scene_timer)
            .add_systems(
                Update,
                (update_scene_timer, update_pvp_vs_fire_effect)
                    .run_if(in_state(LevelStates::InPrepareGame)),
            );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            Update,
            packet_receive_loop.run_if(in_state(LevelStates::InPrepareGame)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InPrepareGame");
}

fn hide_loading_interfaces(
    mut query: Query<&mut Visibility, (With<EnterGameLevelEntity>, With<UI>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn setup_scene_timer(mut commands: Commands) {
    commands.insert_resource(SceneTimer::default());
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
        next_state.set(LevelStates::SwitchToInGame);
    }
}

fn update_pvp_vs_fire_effect(
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
