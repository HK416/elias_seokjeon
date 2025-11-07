mod init;

// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(init::InnerPlugin)
            .add_systems(OnEnter(LevelStates::InPrepareGame), debug_label);
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InitGame");
}
