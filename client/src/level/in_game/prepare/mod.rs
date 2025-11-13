mod init;
mod switch;

// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(init::InnerPlugin)
            .add_plugins(switch::InnerPlugin)
            .add_systems(
                OnEnter(LevelStates::InPrepareGame),
                (debug_label, hide_loading_interfaces),
            )
            .add_systems(
                Update,
                update_pvp_vs_fire_effect.run_if(in_state(LevelStates::InPrepareGame)),
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

// --- UPDATE SYSTEMS ---

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
