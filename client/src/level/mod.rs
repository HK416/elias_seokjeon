mod core;
mod in_game;
mod matching;
mod option;
mod title;

mod constants;
mod resource;
mod system;
mod types;
mod utils;

// Import necessary Bevy modules.
use bevy::prelude::*;

use crate::{
    WND_HEIGHT, WND_WIDTH, assets::path::*, collider::*, resizable_font::*, translatable_text::*,
};

#[cfg(target_arch = "wasm32")]
use crate::web::*;

use self::{constants::*, core::ErrorMessage, resource::*, system::*, types::*, utils::*};

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LevelStates>()
            .add_plugins(core::InnerPlugin)
            .add_plugins(in_game::InnerPlugin)
            .add_plugins(matching::InnerPlugin)
            .add_plugins(option::InnerPlugin)
            .add_plugins(title::InnerPlugin);
    }
}

// --- STATES ---

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum LevelStates {
    Error,
    #[default]
    Setup,
    Connect,

    InitOption,
    InOption,

    LoadTitle,
    InitTitle,
    InTitle,

    InitMatchingCancel,
    InMatchingCancel,

    InitMatching,
    InMatching,

    LoadEnterGame,
    InitEnterGame,

    LoadGame,
}
