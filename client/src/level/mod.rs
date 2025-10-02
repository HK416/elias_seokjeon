mod error;
mod setup;

mod constants;
mod resource;
mod types;

// Import necessary Bevy modules.
use bevy::prelude::*;

use crate::{assets::path::*, resizable_font::*};

#[cfg(target_arch = "wasm32")]
use crate::web::*;

use self::{constants::*, resource::*, types::*};

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LevelStates>()
            .add_plugins(setup::InnerPlugin)
            .add_plugins(error::InnerPlugin);
    }
}

// --- STATES ---

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum LevelStates {
    #[default]
    Setup,
    Error,
}
