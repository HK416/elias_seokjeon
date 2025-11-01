use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelStates::InitGame), (debug_label, setup_in_game))
            .add_systems(OnExit(LevelStates::InitGame), cleanup_loading_resource)
            .add_systems(
                Update,
                (
                    update_spawn_progress,
                    observe_entiey_creation,
                    check_loading_progress,
                )
                    .run_if(in_state(LevelStates::InitGame)),
            );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            PreUpdate,
            handle_received_packets.run_if(in_state(LevelStates::InitGame)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InitGame");
}

fn setup_in_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut loading_entities = LoadingEntities::default();
    setup_in_game_entities(&mut commands, &asset_server, &mut loading_entities);
    setup_in_game_interface(&mut commands, &asset_server, &mut loading_entities);

    // --- Resource Insertion ---
    commands.insert_resource(loading_entities);
}

fn setup_in_game_entities(
    commands: &mut Commands,
    asset_server: &AssetServer,
    loading_entities: &mut LoadingEntities,
) {
    // TODO
}

fn setup_in_game_interface(
    commands: &mut Commands,
    asset_server: &AssetServer,
    loading_entities: &mut LoadingEntities,
) {
    // TODO
}

// --- PREUPDATE SYSTEMS ---

#[cfg(target_arch = "wasm32")]
fn handle_received_packets(
    mut commands: Commands,
    mut next_state: ResMut<NextState<LevelStates>>,
    network: Res<Network>,
) {
    for result in network.try_iter() {
        match result {
            Ok(packet) => match packet {
                Packet::GameLoadTimeout => {
                    commands.insert_resource(ErrorMessage::new(
                        "game_load_timeout",
                        "Failed to enter the game due to a connection timeout.",
                    ));
                    next_state.set(LevelStates::SwitchToTitleMessage);
                }
                _ => { /* empty */ }
            },
            Err(e) => {
                commands.insert_resource(ErrorMessage::from(e));
                next_state.set(LevelStates::Error);
            }
        }
    }
}

// --- UPDATE SYSTEMS ---

#[allow(clippy::type_complexity)]
fn update_spawn_progress(
    loading_assets: Res<LoadingEntities>,
    mut set: ParamSet<(
        Query<&mut Node, With<EnterGameLoadingBar>>,
        Query<&mut Node, With<EnterGameLoadingCursor>>,
    )>,
) {
    let progress = loading_assets.percent();

    if let Ok(mut node) = set.p0().single_mut() {
        node.width = Val::Percent(progress * 100.0);
    }

    if let Ok(mut node) = set.p1().single_mut() {
        node.left = Val::Percent(progress * 100.0);
    }
}

fn observe_entiey_creation(
    mut commands: Commands,
    mut loading_entities: ResMut<LoadingEntities>,
    query: Query<(Entity, Option<&ChildOf>), Added<SpawnRequest>>,
) {
    for (entity, child_of) in query.iter() {
        loading_entities.remove(entity);

        let mut commands = commands.entity(entity);
        commands.remove::<SpawnRequest>();

        commands.insert(InGameLevelEntity);
        if child_of.is_none() {
            commands.insert(InGameLevelRoot);
        }
    }
}

fn check_loading_progress(
    mut commands: Commands,
    loading_entities: Res<LoadingEntities>,
    mut next_state: ResMut<NextState<LevelStates>>,
) {
    if loading_entities.is_empty() {
        // TODO
    }
}
