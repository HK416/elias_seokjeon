mod core;
mod in_game;

mod constants;
mod resource;
mod system;
mod types;

// Import necessary Bevy modules.
use bevy::prelude::*;

use crate::{assets::path::*, resizable_font::*};

#[cfg(target_arch = "wasm32")]
use crate::web::*;

use self::{constants::*, core::ErrorMessage, resource::*, system::*, types::*};

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LevelStates>()
            .add_plugins(core::InnerPlugin)
            .add_plugins(in_game::InnerPlugin);
    }
}

// --- STATES ---

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum LevelStates {
    Error,
    #[default]
    Setup,
    Connect,

    LoadGame,
}
