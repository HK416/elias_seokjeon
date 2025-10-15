mod init;

// Import necessary Bevy modules.
use bevy::prelude::*;

use crate::assets::sound::SystemVolume;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(init::InnerPlugin)
            .add_systems(
                OnEnter(LevelStates::InOption),
                (debug_label, show_interface),
            )
            .add_systems(
                OnExit(LevelStates::InOption),
                (
                    hide_interface,
                    #[cfg(target_arch = "wasm32")]
                    save_volume_options,
                ),
            )
            .add_systems(
                PreUpdate,
                (handle_keyboard_inputs,).run_if(in_state(LevelStates::InOption)),
            )
            .add_systems(
                Update,
                (handle_spine_animation_completed, update_wave_animation)
                    .run_if(in_state(LevelStates::InOption)),
            );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InOption");
}

fn show_interface(mut query: Query<&mut Visibility, (With<UI>, With<OptionLevelEntity>)>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

// --- CLEANUP SYSTEMS ---

fn hide_interface(mut query: Query<&mut Visibility, (With<UI>, With<OptionLevelEntity>)>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

#[cfg(target_arch = "wasm32")]
fn save_volume_options(system_volume: Res<SystemVolume>) {
    if let Some(storage) = get_local_storage()
        && let Ok(value) = serde_json::to_string(&*system_volume)
    {
        info!("Store system volume: {}", &value);
        let _ = storage.set_item(SYSTEM_VOLUME_KEY, &value);
    }
}

// --- PREUPDATE SYSTEMS ---

fn handle_keyboard_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(LevelStates::InTitle);
    }
}
