mod error;
mod setup;

// Import necessary Bevy modules.
use bevy::prelude::*;

pub use self::error::ErrorMessage;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(error::InnerPlugin)
            .add_plugins(setup::InnerPlugin);
    }
}
