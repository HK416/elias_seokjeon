// Import necessary Bevy modules.
use bevy::prelude::*;
use bevy_spine::Spine;

use super::*;

// --- CONSTANTS ---
const SCENE_DURATION: f32 = 0.7;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::SwitchToLoadGame),
            (
                debug_label,
                show_interface,
                setup_scene_timer,
                setup_background_popup,
                setup_entity_fade_out,
                setup_ui_animation,
            ),
        )
        .add_systems(OnExit(LevelStates::SwitchToLoadGame), cleanup_scene_timer)
        .add_systems(
            Update,
            (update_scene_timer, update_popup, update_fade_out)
                .run_if(in_state(LevelStates::SwitchToLoadGame)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: SwitchToLoadGame");
}

fn show_interface(mut query: Query<&mut Visibility, (With<EnterGameLevelEntity>, With<UI>)>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn setup_scene_timer(mut commands: Commands) {
    commands.insert_resource(SceneTimer::default());
}

fn setup_background_popup(
    mut commands: Commands,
    query: Query<(Entity, &BackgroundPattern), With<EnterGameLevelEntity>>,
) {
    for (entity, pattern) in query.iter() {
        let delay = pattern.0 as f32 * 0.05;
        commands
            .entity(entity)
            .insert(BackoutScale::new(0.1, Vec3::ZERO, Vec3::ONE).with_delay(delay));
    }
}

fn setup_entity_fade_out(
    mut commands: Commands,
    query: Query<Entity, (With<Spine>, With<TitleLevelEntity>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(FadeEffect::new(SCENE_DURATION));
    }
}

fn setup_ui_animation(
    mut commands: Commands,
    query: Query<(Entity, &UI), With<EnterGameLevelEntity>>,
) {
    for (entity, &ui) in query.iter() {
        match ui {
            UI::EnterGameLoadingBar => {
                commands.entity(entity).insert(UiBackOutScale::new(
                    SCENE_DURATION,
                    Vec2::ZERO,
                    Vec2::ONE,
                ));
            }
            _ => { /* empty */ }
        }
    }
}

// --- CLEANUP SYSTEMS ---

fn cleanup_scene_timer(mut commands: Commands) {
    commands.remove_resource::<SceneTimer>();
}

// --- UPDATE SYSTEMS ---

fn update_scene_timer(
    mut next_state: ResMut<NextState<LevelStates>>,
    mut scene_timer: ResMut<SceneTimer>,
    time: Res<Time>,
) {
    scene_timer.tick(time.delta_secs());
    if scene_timer.elapsed_sec() >= SCENE_DURATION {
        next_state.set(LevelStates::LoadGame);
    }
}

fn update_popup(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BackoutScale, &mut Transform), With<EnterGameLevelEntity>>,
    time: Res<Time>,
) {
    for (entity, mut popup, mut transform) in query.iter_mut() {
        popup.tick(time.delta_secs());
        *transform = transform.with_scale(popup.scale());
        if popup.is_finished() {
            commands.entity(entity).remove::<BackoutScale>();
        }
    }
}

fn update_fade_out(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Visibility, &mut FadeEffect, &mut Spine)>,
    time: Res<Time>,
) {
    for (entity, mut visibility, mut fade_out, mut spine) in query.iter_mut() {
        fade_out.tick(time.delta_secs());
        spine
            .0
            .skeleton
            .color_mut()
            .set_a(1.0 - fade_out.progress());
        if fade_out.is_finished() {
            *visibility = Visibility::Hidden;
            spine.0.skeleton.color_mut().set_a(1.0);
            commands.entity(entity).remove::<FadeEffect>();
        }
    }
}
