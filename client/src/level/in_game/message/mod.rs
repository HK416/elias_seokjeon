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
            .add_systems(OnEnter(LevelStates::InTitleMessage), debug_label)
            .add_systems(OnExit(LevelStates::InTitleMessage), hide_interface)
            .add_systems(
                PreUpdate,
                (handle_keyboard_input, handle_button_interaction)
                    .run_if(in_state(LevelStates::InTitleMessage)),
            )
            .add_systems(
                Update,
                (handle_spine_animation_completed, update_wave_animation)
                    .run_if(in_state(LevelStates::InTitleMessage)),
            );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            Update,
            packet_receive_loop.run_if(in_state(LevelStates::InTitleMessage)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InTitleMessage");
}

// --- CLEANUP SYSTEMS ---

fn hide_interface(mut query: Query<&mut Visibility, (With<UI>, With<TitleMessageLevelEntity>)>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

// --- PREUPDATE SYSTEMS ---

fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        next_state.set(LevelStates::InTitle);
    }
}

#[allow(clippy::type_complexity)]
fn handle_button_interaction(
    mut next_state: ResMut<NextState<LevelStates>>,
    children_query: Query<&Children>,
    mut text_color_query: Query<(&mut TextColor, &OriginColor<TextColor>)>,
    mut button_color_query: Query<(&mut BackgroundColor, &OriginColor<BackgroundColor>)>,
    mut interaction_query: Query<
        (Entity, &UI, &Interaction),
        (With<TitleMessageLevelEntity>, Changed<Interaction>),
    >,
) {
    for (entity, &ui, interaction) in interaction_query.iter_mut() {
        update_button_visual(
            entity,
            interaction,
            &children_query,
            &mut text_color_query,
            &mut button_color_query,
        );

        match (ui, interaction) {
            (UI::PositiveButton, Interaction::Pressed) => {
                next_state.set(LevelStates::InTitle);
            }
            _ => { /* empty */ }
        }
    }
}
