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
            OnEnter(LevelStates::SwitchToGameDefeat),
            (
                debug_label,
                setup_scene_timer,
                show_game_result_entities,
                show_game_result_interfaces,
                setup_game_result_spines,
                setup_game_result_patterns,
                setup_game_result_interfaces,
                setup_in_game_sprite,
                setup_in_game_spines,
                setup_in_game_interfaces,
                play_popup_bobble_sounds,
            ),
        )
        .add_systems(
            OnExit(LevelStates::SwitchToGameDefeat),
            (
                cleanup_scene_timer,
                cleanup_in_game_entities,
                play_in_game_defeat_sound,
            ),
        )
        .add_systems(
            Update,
            (
                update_scene_timer,
                update_game_result_spines,
                update_game_result_patterns,
                update_in_game_spines,
                update_in_game_sprites,
            )
                .run_if(in_state(LevelStates::SwitchToGameDefeat)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: SwitchToGameDefeat");
}

fn setup_scene_timer(mut commands: Commands) {
    commands.insert_resource(SceneTimer::default());
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn show_game_result_entities(
    mut query: Query<
        &mut Visibility,
        (
            Without<GameResultVictory>,
            Without<GameResultDefeat>,
            Without<GameResultDraw>,
            With<InGameResultLevelEntity>,
            With<InGameLevelRoot>,
        ),
    >,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

#[allow(clippy::type_complexity)]
fn show_game_result_interfaces(
    mut query: Query<
        &mut Visibility,
        (
            With<GameResultDefeat>,
            With<InGameResultLevelEntity>,
            With<InGameLevelRoot>,
        ),
    >,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

#[allow(clippy::type_complexity)]
fn setup_game_result_spines(
    mut commands: Commands,
    mut spines: Query<
        (Entity, &mut Spine, &Character, &mut CharacterAnimState),
        With<InGameResultLevelEntity>,
    >,
) {
    for (entity, mut spine, character, mut anim_state) in spines.iter_mut() {
        commands
            .entity(entity)
            .insert(FadeEffect::new(SCENE_DURATION));

        *anim_state = CharacterAnimState::Sad;
        play_character_animation(&mut spine, *character, *anim_state);
        spine.skeleton.color_mut().set_a(0.0);
    }
}

#[allow(clippy::type_complexity)]
fn setup_game_result_patterns(
    mut commands: Commands,
    query: Query<(Entity, &BackgroundPattern), (With<Sprite>, With<InGameResultLevelEntity>)>,
) {
    for (entity, pattern) in query.iter() {
        let delay = pattern.0 as f32 * 0.05;
        commands
            .entity(entity)
            .insert(BackoutScale::new(0.1, Vec3::ZERO, Vec3::ONE).with_delay(delay));
    }
}

#[allow(clippy::type_complexity)]
fn setup_game_result_interfaces(
    mut commands: Commands,
    query: Query<Entity, (With<UiAnimationTarget>, With<InGameResultLevelEntity>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(UiBackOutScale::new(SCENE_DURATION, Vec2::ZERO, Vec2::ONE));
    }
}

#[allow(clippy::type_complexity)]
fn setup_in_game_sprite(
    mut commands: Commands,
    sprites: Query<
        Entity,
        (
            Without<BackgroundPattern>,
            With<Sprite>,
            With<InGameLevelEntity>,
        ),
    >,
) {
    for entity in sprites.iter() {
        commands
            .entity(entity)
            .insert(FadeEffect::new(SCENE_DURATION));
    }
}

fn setup_in_game_spines(
    mut commands: Commands,
    spines: Query<Entity, (With<Spine>, With<InGameLevelEntity>)>,
) {
    for entity in spines.iter() {
        commands
            .entity(entity)
            .insert(FadeEffect::new(SCENE_DURATION));
    }
}

fn setup_in_game_interfaces(
    mut commands: Commands,
    query: Query<Entity, (With<UiAnimationTarget>, With<InGameLevelEntity>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(UiSmoothScale::new(SCENE_DURATION, Vec2::ONE, Vec2::ZERO));
    }
}

// --- CLEANUP SYSTEMS ---

fn cleanup_scene_timer(mut commands: Commands) {
    commands.remove_resource::<SceneTimer>();
}

fn cleanup_in_game_entities(
    mut commands: Commands,
    query: Query<Entity, (With<InGameLevelEntity>, With<InGameLevelRoot>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// --- UPDATE SYSTEMS ---

fn update_scene_timer(
    mut next_state: ResMut<NextState<LevelStates>>,
    mut scene_timer: ResMut<SceneTimer>,
    time: Res<Time>,
) {
    scene_timer.tick(time.delta_secs());
    if scene_timer.elapsed_sec() >= SCENE_DURATION {
        next_state.set(LevelStates::InGameResult);
    }
}

fn update_game_result_spines(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadeEffect, &mut Spine), With<InGameResultLevelEntity>>,
    time: Res<Time>,
) {
    for (entity, mut fade_in, mut spine) in query.iter_mut() {
        fade_in.tick(time.delta_secs());
        spine.skeleton.color_mut().set_a(fade_in.progress());
        if fade_in.is_finished() {
            commands.entity(entity).remove::<FadeEffect>();
        }
    }
}

fn update_game_result_patterns(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BackoutScale, &mut Transform), With<InGameResultLevelEntity>>,
    time: Res<Time>,
) {
    for (entity, mut backout, mut transform) in query.iter_mut() {
        backout.tick(time.delta_secs());
        transform.scale = backout.scale();
        if backout.is_finished() {
            commands.entity(entity).remove::<BackoutScale>();
        }
    }
}

fn update_in_game_spines(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadeEffect, &mut Spine), With<InGameLevelEntity>>,
    time: Res<Time>,
) {
    for (entity, mut fade_out, mut spine) in query.iter_mut() {
        fade_out.tick(time.delta_secs());
        spine.skeleton.color_mut().set_a(1.0 - fade_out.progress());
        if fade_out.is_finished() {
            commands.entity(entity).remove::<FadeEffect>();
        }
    }
}

fn update_in_game_sprites(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadeEffect, &mut Sprite), With<InGameLevelEntity>>,
    time: Res<Time>,
) {
    for (entity, mut fade_out, mut sprite) in query.iter_mut() {
        fade_out.tick(time.delta_secs());
        sprite.color = sprite.color.with_alpha(1.0 - fade_out.progress());
        if fade_out.is_finished() {
            commands.entity(entity).remove::<FadeEffect>();
        }
    }
}
