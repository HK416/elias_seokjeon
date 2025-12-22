mod init;
mod switch;

// Import necessary Bevy modules.
use bevy::prelude::*;

use crate::assets::{locale::Locale, sound::SystemVolume};

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(init::InnerPlugin)
            .add_plugins(switch::InnerPlugin)
            .add_systems(
                OnEnter(LevelStates::InOption),
                (debug_label, init_selected_slider_flag),
            )
            .add_systems(
                OnExit(LevelStates::InOption),
                (
                    hide_option_interfaces,
                    cleanup_resource,
                    #[cfg(target_arch = "wasm32")]
                    save_volume_options,
                    play_popup_close_sounds,
                ),
            )
            .add_systems(
                PreUpdate,
                (
                    handle_keyboard_inputs,
                    handle_pn_button_pressed,
                    handle_locale_button_pressed,
                    handle_volume_button_pressed,
                    handle_volume_button_pressed_for_mobile,
                    handle_volume_button_released,
                    handle_volume_button_released_for_mobile,
                )
                    .run_if(in_state(LevelStates::InOption)),
            )
            .add_systems(
                Update,
                (
                    handle_spine_animation_completed,
                    update_wave_animation,
                    update_volume_text,
                    update_volume_slider,
                    update_volume_slider_for_mobile,
                )
                    .run_if(in_state(LevelStates::InOption)),
            );

        #[cfg(target_arch = "wasm32")]
        app.add_systems(
            Update,
            packet_receive_loop.run_if(in_state(LevelStates::InOption)),
        )
        .add_systems(
            FixedUpdate,
            (
                update_background_volumes,
                update_effect_volumes,
                update_voice_volumes,
            )
                .run_if(in_state(LevelStates::InOption)),
        );
    }
}

// --- SETUP SYSTEMS ---

fn debug_label() {
    info!("Current Level: InOption");
}

fn init_selected_slider_flag(mut commands: Commands) {
    commands.insert_resource(SelectedSliderCursor::default());
}

// --- CLEANUP SYSTEMS ---

fn hide_option_interfaces(
    mut query: Query<&mut Visibility, (With<OptionLevelEntity>, With<TitleLevelRoot>)>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn cleanup_resource(mut commands: Commands) {
    commands.remove_resource::<SelectedSliderCursor>();
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

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn handle_pn_button_pressed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    mut next_state: ResMut<NextState<LevelStates>>,
    children_query: Query<&Children>,
    mut text_color_query: Query<(&mut TextColor, &OriginColor<TextColor>)>,
    mut button_color_query: Query<(&mut BackgroundColor, &OriginColor<BackgroundColor>)>,
    mut interaction_query: Query<
        (Entity, &PNButton, &Interaction),
        (With<OptionLevelEntity>, Changed<Interaction>, With<Button>),
    >,
) {
    for (entity, &pn_button, interaction) in interaction_query.iter_mut() {
        update_button_visual(
            entity,
            interaction,
            &children_query,
            &mut text_color_query,
            &mut button_color_query,
        );

        match (pn_button, interaction) {
            (PNButton::Positive, Interaction::Pressed) => {
                let source = asset_server.load(SFX_PATH_COMMON_BUTTON_DOWN);
                play_effect_sound(&mut commands, &system_volume, source);
                next_state.set(LevelStates::InTitle);
            }
            (PNButton::Positive, Interaction::Hovered) => {
                let source = asset_server.load(SFX_PATH_COMMON_BUTTON_TOUCH);
                play_effect_sound(&mut commands, &system_volume, source);
            }
            _ => { /* empty */ }
        }
    }
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn handle_locale_button_pressed(
    mut commands: Commands,
    mut locale: ResMut<Locale>,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    children_query: Query<&Children>,
    mut text_color_query: Query<(&mut TextColor, &OriginColor<TextColor>)>,
    mut button_color_query: Query<(&mut BackgroundColor, &OriginColor<BackgroundColor>)>,
    mut interaction_query: Query<
        (Entity, &LocaleButton, &Interaction),
        (With<OptionLevelEntity>, Changed<Interaction>, With<Button>),
    >,
) {
    for (entity, &locale_button, interaction) in interaction_query.iter_mut() {
        update_button_visual(
            entity,
            interaction,
            &children_query,
            &mut text_color_query,
            &mut button_color_query,
        );

        match (locale_button, interaction) {
            (LocaleButton::En, Interaction::Pressed) => {
                let source = asset_server.load(SFX_PATH_COMMON_BUTTON_DOWN);
                play_effect_sound(&mut commands, &system_volume, source);
                *locale = Locale::En;
            }
            (LocaleButton::Ja, Interaction::Pressed) => {
                let source = asset_server.load(SFX_PATH_COMMON_BUTTON_DOWN);
                play_effect_sound(&mut commands, &system_volume, source);
                *locale = Locale::Ja;
            }
            (LocaleButton::Ko, Interaction::Pressed) => {
                let source = asset_server.load(SFX_PATH_COMMON_BUTTON_DOWN);
                play_effect_sound(&mut commands, &system_volume, source);
                *locale = Locale::Ko;
            }
            (LocaleButton::En, Interaction::Hovered)
            | (LocaleButton::Ja, Interaction::Hovered)
            | (LocaleButton::Ko, Interaction::Hovered) => {
                let source = asset_server.load(SFX_PATH_COMMON_BUTTON_TOUCH);
                play_effect_sound(&mut commands, &system_volume, source);
            }
            _ => { /* empty */ }
        }
    }
}

#[allow(clippy::type_complexity)]
fn handle_volume_button_pressed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut selected: ResMut<SelectedSliderCursor>,
    interaction_query: Query<
        (Entity, &VolumeSlider, &Interaction),
        (With<OptionLevelEntity>, Changed<Interaction>),
    >,
) {
    for (entity, &volume_slider, &interaction) in interaction_query.iter() {
        match (volume_slider, interaction) {
            (VolumeSlider::Background, Interaction::Pressed)
            | (VolumeSlider::Effect, Interaction::Pressed)
            | (VolumeSlider::Voice, Interaction::Pressed) => {
                if mouse_buttons.just_pressed(MouseButton::Left) {
                    let source = asset_server.load(SFX_PATH_COMMON_BUTTON_DOWN);
                    play_effect_sound(&mut commands, &system_volume, source);
                    selected.set(volume_slider, entity, 0);
                    break;
                }
            }
            (VolumeSlider::Background, Interaction::Hovered)
            | (VolumeSlider::Effect, Interaction::Hovered)
            | (VolumeSlider::Voice, Interaction::Hovered) => {
                let source = asset_server.load(SFX_PATH_COMMON_POPUP_BUTTON_TOUCH);
                play_effect_sound(&mut commands, &system_volume, source);
            }
            _ => { /* empty */ }
        }
    }
}

#[allow(clippy::type_complexity)]
fn handle_volume_button_pressed_for_mobile(
    mut commands: Commands,
    touches: Res<Touches>,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    mut selected: ResMut<SelectedSliderCursor>,
    interaction_query: Query<
        (Entity, &VolumeSlider, &Interaction),
        (With<OptionLevelEntity>, Changed<Interaction>),
    >,
) {
    if let Some(touch) = touches.iter_just_pressed().last() {
        for (entity, &volume_slider, &interaction) in interaction_query.iter() {
            match (volume_slider, interaction) {
                (VolumeSlider::Background, Interaction::Pressed)
                | (VolumeSlider::Effect, Interaction::Pressed)
                | (VolumeSlider::Voice, Interaction::Pressed) => {
                    let source = asset_server.load(SFX_PATH_COMMON_BUTTON_DOWN);
                    play_effect_sound(&mut commands, &system_volume, source);
                    selected.set(volume_slider, entity, touch.id());
                    break;
                }
                _ => { /* empty */ }
            }
        }
    }
}

fn handle_volume_button_released(
    mut commands: Commands,
    player_info: Res<PlayerInfo>,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut selected: ResMut<SelectedSliderCursor>,
) {
    if mouse_buttons.just_released(MouseButton::Left)
        && let Some((slider, _, _)) = selected.take()
    {
        match slider {
            VolumeSlider::Effect => {
                let source = asset_server.load(SFX_PATH_COMMON_BUTTON_UP);
                play_effect_sound(&mut commands, &system_volume, source);
            }
            VolumeSlider::Voice => {
                let index = player_info.hero as usize;
                if let Some(set) = HERO_VOICE_SETS.get(index)
                    && let Some(path) = set.call_player().choose(&mut rand::rng()).copied()
                {
                    let source = asset_server.load(path);
                    play_voice_sound(
                        &mut commands,
                        &system_volume,
                        source,
                        VoiceChannel::default(),
                    );
                }
            }
            _ => { /* empty */ }
        }
    }
}

fn handle_volume_button_released_for_mobile(
    mut commands: Commands,
    touches: Res<Touches>,
    player_info: Res<PlayerInfo>,
    asset_server: Res<AssetServer>,
    system_volume: Res<SystemVolume>,
    mut selected: ResMut<SelectedSliderCursor>,
) {
    if let Some((_, _, id)) = selected.get()
        && touches.just_released(id)
        && let Some((slider, _, _)) = selected.take()
    {
        match slider {
            VolumeSlider::Effect => {
                let source = asset_server.load(SFX_PATH_COMMON_BUTTON_UP);
                play_effect_sound(&mut commands, &system_volume, source);
            }
            VolumeSlider::Voice => {
                let index = player_info.hero as usize;
                if let Some(set) = HERO_VOICE_SETS.get(index)
                    && let Some(path) = set.call_player().choose(&mut rand::rng()).copied()
                {
                    let source = asset_server.load(path);
                    play_voice_sound(
                        &mut commands,
                        &system_volume,
                        source,
                        VoiceChannel::default(),
                    );
                }
            }
            _ => { /* empty */ }
        }
    }
}

// --- UPDATE SYSTEMS ---

fn update_volume_text(
    system_volume: Res<SystemVolume>,
    mut query: Query<(&VolumeLevelTextId, &mut Text), With<OptionLevelEntity>>,
) {
    for (volume, mut text) in query.iter_mut() {
        match volume {
            VolumeLevelTextId::Background => {
                let percentage = system_volume.get_background().to_linear() * 100.0;
                *text = Text::new(format!("{}", percentage as u8));
            }
            VolumeLevelTextId::Effect => {
                let percentage = system_volume.get_effect().to_linear() * 100.0;
                *text = Text::new(format!("{}", percentage as u8));
            }
            VolumeLevelTextId::Voice => {
                let percentage = system_volume.get_voice().to_linear() * 100.0;
                *text = Text::new(format!("{}", percentage as u8));
            }
        }
    }
}

fn update_volume_slider(
    windows: Query<&Window>,
    slider_query: Query<(&ComputedNode, &UiGlobalTransform), With<OptionLevelEntity>>,
    mut handler_query: Query<(&mut Node, &ChildOf), With<OptionLevelEntity>>,
    parent_query: Query<&ChildOf, With<OptionLevelEntity>>,
    mut system_volume: ResMut<SystemVolume>,
    selected: Res<SelectedSliderCursor>,
) {
    let Ok(window) = windows.single() else { return };

    if let Some((slider, entity, _)) = selected.get()
        && let Some(point) = window.physical_cursor_position()
        && let Ok(child_of) = parent_query.get(entity)
        && let Ok((mut node, child_of)) = handler_query.get_mut(child_of.parent())
        && let Ok((computed_node, &ui_transform)) = slider_query.get(child_of.parent())
        && let Some(norm) = computed_node.normalize_point(ui_transform, point)
    {
        let p = (norm.x + 0.5).clamp(0.0, 1.0);
        node.left = Val::Percent(p * 100.0);
        match slider {
            VolumeSlider::Background => system_volume.background = (p * 255.0) as u8,
            VolumeSlider::Effect => system_volume.effect = (p * 255.0) as u8,
            VolumeSlider::Voice => system_volume.voice = (p * 255.0) as u8,
        }
    }
}

fn update_volume_slider_for_mobile(
    windows: Query<&Window>,
    slider_query: Query<(&ComputedNode, &UiGlobalTransform), With<OptionLevelEntity>>,
    mut handler_query: Query<(&mut Node, &ChildOf), With<OptionLevelEntity>>,
    parent_query: Query<&ChildOf, With<OptionLevelEntity>>,
    mut system_volume: ResMut<SystemVolume>,
    touches: Res<Touches>,
    selected: Res<SelectedSliderCursor>,
) {
    let Ok(window) = windows.single() else { return };

    if let Some((slider, entity, touch_id)) = selected.get()
        && let Some(touch) = touches.get_pressed(touch_id)
        && let Ok(child_of) = parent_query.get(entity)
        && let Ok((mut node, child_of)) = handler_query.get_mut(child_of.parent())
        && let Ok((computed_node, &ui_transform)) = slider_query.get(child_of.parent())
        && let Some(norm) =
            computed_node.normalize_point(ui_transform, touch.position() * window.scale_factor())
    {
        let p = (norm.x + 0.5).clamp(0.0, 1.0);
        node.left = Val::Percent(p * 100.0);
        match slider {
            VolumeSlider::Background => system_volume.background = (p * 255.0) as u8,
            VolumeSlider::Effect => system_volume.effect = (p * 255.0) as u8,
            VolumeSlider::Voice => system_volume.voice = (p * 255.0) as u8,
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn update_background_volumes(
    system_volume: Res<SystemVolume>,
    mut query: Query<&mut WebPlaybackSettings, With<BackgroundSound>>,
) {
    for mut settings in query.iter_mut() {
        settings.volume = system_volume.get_background();
    }
}

#[cfg(target_arch = "wasm32")]
fn update_effect_volumes(
    system_volume: Res<SystemVolume>,
    mut query: Query<&mut WebPlaybackSettings, With<EffectSound>>,
) {
    for mut settings in query.iter_mut() {
        settings.volume = system_volume.get_effect();
    }
}

#[cfg(target_arch = "wasm32")]
fn update_voice_volumes(
    system_volume: Res<SystemVolume>,
    mut query: Query<&mut WebPlaybackSettings, With<VoiceSound>>,
) {
    for mut settings in query.iter_mut() {
        settings.volume = system_volume.get_voice();
    }
}
