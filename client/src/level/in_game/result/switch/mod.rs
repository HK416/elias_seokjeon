mod defeat;
mod draw;
mod victory;

// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(defeat::InnerPlugin)
            .add_plugins(draw::InnerPlugin)
            .add_plugins(victory::InnerPlugin);
    }
}
