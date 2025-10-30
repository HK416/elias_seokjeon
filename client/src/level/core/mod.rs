mod connect;
mod error;
mod setup;

// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(connect::InnerPlugin)
            .add_plugins(error::InnerPlugin)
            .add_plugins(setup::InnerPlugin);
    }
}
