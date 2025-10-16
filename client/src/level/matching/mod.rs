mod cancel;

mod init;

// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(cancel::InnerPlugin)
            .add_plugins(init::InnerPlugin)
            .add_systems(
                OnEnter(LevelStates::InMatching),
                (debug_label, show_interface),
            )
            .add_systems(OnExit(LevelStates::InMatching), hide_interface)
            .add_systems(
                PreUpdate,
                (handle_button_interaction,).run_if(in_state(LevelStates::InMatching)),
            )
            .add_systems(
                Update,
                (handle_spine_animation_completed, update_wave_animation)
                    .run_if(in_state(LevelStates::InMatching)),
            );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InMatching");
}

fn show_interface(mut query: Query<&mut Visibility, (With<UI>, With<MatchingLevelEntity>)>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

// --- CLEANUP SYSTEMS ---

fn hide_interface(mut query: Query<&mut Visibility, (With<UI>, With<MatchingLevelEntity>)>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

// --- PREUPDATE SYSTEMS ---

fn handle_button_interaction(
    mut next_state: ResMut<NextState<LevelStates>>,
    children_query: Query<&Children>,
    mut text_color_query: Query<(&mut TextColor, &OriginColor)>,
    mut button_color_query: Query<(&mut BackgroundColor, &OriginColor)>,
    mut interaction_query: Query<
        (Entity, &UI, &Interaction),
        (With<MatchingLevelEntity>, Changed<Interaction>),
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
            (UI::InMatchingCancelButton, Interaction::Pressed) => {
                next_state.set(LevelStates::InMatchingCancel);
            }
            _ => { /* empty */ }
        }
    }
}
