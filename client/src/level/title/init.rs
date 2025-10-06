// Import necessary Bevy modules.
use bevy::prelude::*;

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelStates::InitTitle),
            (debug_label, setup_title_screen),
        )
        .add_systems(
            OnExit(LevelStates::InitTitle),
            (remove_resource, cleanup_loading_screen),
        )
        .add_systems(
            Update,
            (
                observe_entity_creation,
                update_entity_spawn_progress,
                check_loading_progress,
            )
                .run_if(in_state(LevelStates::InitTitle)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InitTitle");
}

fn setup_title_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut loading_entities = LoadingEntities::default();
    let entity = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            Visibility::Hidden,
            SpawnRequest,
        ))
        .with_children(|parent| {
            let texture = asset_server.load(IMG_PATH_BACKGROUND);
            let entity = parent
                .spawn((
                    ImageNode::new(texture),
                    Node {
                        width: Val::Percent(100.0),
                        aspect_ratio: Some(1593.0 / 1019.0),
                        ..Default::default()
                    },
                    ZIndex(5),
                    Visibility::Inherited,
                    SpawnRequest,
                ))
                .id();
            loading_entities.insert(entity);
        })
        .id();
    loading_entities.insert(entity);

    // --- Resource Insersion ---
    commands.insert_resource(loading_entities);
}

// --- CLEANUP SYSTEMS ---

fn remove_resource(mut commands: Commands) {
    commands.remove_resource::<LoadingEntities>();
}

// --- UPDATE SYSTEMS ---

fn observe_entity_creation(
    mut commands: Commands,
    mut loading_entities: ResMut<LoadingEntities>,
    query: Query<(Entity, Option<&ChildOf>), Added<SpawnRequest>>,
) {
    for (entity, child_of) in query.iter() {
        loading_entities.remove(entity);

        let mut commands = commands.entity(entity);
        commands.remove::<SpawnRequest>();

        if child_of.is_some() {
            commands.insert(TitleLevelSub);
        } else {
            commands.insert(TitleLevelRoot);
        }
    }
}

fn check_loading_progress(
    loading_entities: Res<LoadingEntities>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    if loading_entities.is_empty() {
        next_state.set(LevelStates::InTitle);
    }
}
