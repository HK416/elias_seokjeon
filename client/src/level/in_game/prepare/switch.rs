// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- CONSTANTS ---
const SCENE_DURATION: f32 = 0.7;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::SwitchToInPrepare),
            (
                debug_label,
                setup_scene_timer,
                setup_loading_ui,
                setup_prepare_ui,
                setup_prepare_entities,
                setup_pvp_vs_fire_effect,
            ),
        )
        .add_systems(OnExit(LevelStates::SwitchToInPrepare), cleanup_scene_timer)
        .add_systems(
            Update,
            (
                update_scene_timer,
                update_prepare_entity,
                update_prepare_interface,
            )
                .run_if(in_state(LevelStates::SwitchToInPrepare)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: SwitchToInPrepare");
}

fn setup_scene_timer(mut commands: Commands) {
    commands.insert_resource(SceneTimer::default());
}

fn setup_loading_ui(
    mut commands: Commands,
    query: Query<(Entity, &UI), With<EnterGameLevelEntity>>,
) {
    for (entity, &ui) in query.iter() {
        match ui {
            UI::EnterGameLoadingBar => {
                commands.entity(entity).insert(UiSmoothScale::new(
                    SCENE_DURATION,
                    Vec2::ONE,
                    Vec2::ZERO,
                ));
            }
            _ => { /* empty */ }
        }
    }
}

fn setup_prepare_ui(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Visibility, &UI), With<InPrepareLevelEntity>>,
) {
    for (entity, mut visibility, &ui) in query.iter_mut() {
        match ui {
            UI::Root => {
                *visibility = Visibility::Visible;
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

#[allow(clippy::type_complexity)]
fn setup_prepare_entities(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Visibility, &mut Spine),
        (With<InGameLevelRoot>, With<InPrepareLevelEntity>),
    >,
) {
    for (entity, mut visibility, mut spine) in query.iter_mut() {
        commands
            .entity(entity)
            .insert(FadeEffect::new(SCENE_DURATION));
        *visibility = Visibility::Visible;
        spine.0.skeleton.color_mut().set_a(0.0);
    }
}

fn setup_pvp_vs_fire_effect(
    mut query: Query<(&mut ImageNode, &mut AnimationTimer), With<InPrepareLevelEntity>>,
) {
    for (mut image_node, mut timer) in query.iter_mut() {
        timer.reset();
        if let Some(atlas) = image_node.texture_atlas.as_mut() {
            atlas.index = timer.frame_index();
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
        next_state.set(LevelStates::InPrepareGame);
    }
}

fn update_prepare_entity(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadeEffect, &mut Spine)>,
    time: Res<Time>,
) {
    for (entity, mut fade_in, mut spine) in query.iter_mut() {
        fade_in.tick(time.delta_secs());
        spine.0.skeleton.color_mut().set_a(fade_in.progress());
        if fade_in.is_finished() {
            commands.entity(entity).remove::<FadeEffect>();
        }
    }
}

fn update_prepare_interface(
    mut query: Query<(&mut ImageNode, &mut AnimationTimer), With<InPrepareLevelEntity>>,
    time: Res<Time>,
) {
    for (mut image_node, mut timer) in query.iter_mut() {
        timer.tick(time.delta_secs());
        if let Some(atlas) = image_node.texture_atlas.as_mut() {
            atlas.index = timer.frame_index();
        }
    }
}
