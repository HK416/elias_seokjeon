mod init;
mod load;

// Import necessary Bevy modules.
use bevy::prelude::*;
use bevy_spine::Spine;

use super::*;

// --- CONSTANTS ---
const FADE_IN_OUT_DURATION: f32 = 1.0;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(init::InnerPlugin)
            .add_plugins(load::InnerPlugin)
            .add_systems(
                OnEnter(LevelStates::LoadGame),
                (
                    debug_label,
                    show_interface,
                    setup_background_entities,
                    setup_spine_entities,
                ),
            )
            .add_systems(
                Update,
                (update_fade_in, update_fade_out).run_if(in_state(LevelStates::LoadGame)),
            );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: LoadGame");
}

fn show_interface(mut query: Query<&mut Visibility, (With<EnterGameLevelEntity>, With<UI>)>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn setup_background_entities(mut commands: Commands, query: Query<Entity, With<BluredBackground>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(FadeIn::new(FADE_IN_OUT_DURATION));
    }
}

fn setup_spine_entities(mut commands: Commands, query: Query<Entity, With<Spine>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(FadeOut::new(FADE_IN_OUT_DURATION));
    }
}

// --- UPDATE SYSTEMS ---

fn update_fade_in(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadeIn, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut fade_in, mut sprite) in query.iter_mut() {
        fade_in.tick(time.delta_secs());
        sprite.color = sprite.color.with_alpha(fade_in.progress());
        if fade_in.is_finished() {
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}

fn update_fade_out(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadeOut, &mut Spine)>,
    time: Res<Time>,
) {
    for (entity, mut fade_out, mut spine) in query.iter_mut() {
        fade_out.tick(time.delta_secs());
        spine
            .0
            .skeleton
            .color_mut()
            .set_a(1.0 - fade_out.progress());
        if fade_out.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
