mod init;

// Import necessary Bevy modules.
use bevy::prelude::*;

use crate::assets::{locale::Locale, sound::SystemVolume};

use super::*;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(init::InnerPlugin)
            .add_systems(
                OnEnter(LevelStates::InOption),
                (debug_label, show_interface, init_selected_slider_flag),
            )
            .add_systems(
                OnExit(LevelStates::InOption),
                (
                    hide_interface,
                    cleanup_resource,
                    #[cfg(target_arch = "wasm32")]
                    save_volume_options,
                ),
            )
            .add_systems(
                PreUpdate,
                (handle_keyboard_inputs, handle_button_interaction)
                    .run_if(in_state(LevelStates::InOption)),
            )
            .add_systems(
                Update,
                (
                    handle_slider_pressed,
                    handle_slider_pressed_for_moblie,
                    handle_slider_release,
                    handle_slider_release_for_moblie,
                    handle_spine_animation_completed,
                    update_wave_animation,
                    update_volume_value,
                    update_slider_cursor,
                    update_slider_cursor_for_moblie,
                )
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

fn init_selected_slider_flag(mut commands: Commands) {
    commands.insert_resource(SelectedSliderCursor::default());
}

// --- CLEANUP SYSTEMS ---

fn hide_interface(mut query: Query<&mut Visibility, (With<UI>, With<OptionLevelEntity>)>) {
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

fn handle_button_interaction(
    mut locale: ResMut<Locale>,
    mut next_state: ResMut<NextState<LevelStates>>,
    children_query: Query<&Children>,
    mut text_color_query: Query<(&mut TextColor, &OriginColor)>,
    mut button_color_query: Query<(&mut BackgroundColor, &OriginColor)>,
    mut interaction_query: Query<
        (Entity, &UI, &Interaction),
        (With<OptionLevelEntity>, With<Button>),
    >,
) {
    for (entity, &ui, interaction) in interaction_query.iter_mut() {
        update_button_visual(
            entity,
            match (ui, *locale) {
                (UI::LocaleButtonEn, Locale::En)
                | (UI::LocaleButtonJa, Locale::Ja)
                | (UI::LocaleButtonKo, Locale::Ko) => &Interaction::Pressed,
                _ => interaction,
            },
            &children_query,
            &mut text_color_query,
            &mut button_color_query,
        );

        match (ui, interaction) {
            (UI::InOptionExitButton, Interaction::Pressed) => {
                next_state.set(LevelStates::InTitle);
            }
            (UI::LocaleButtonEn, Interaction::Pressed) => {
                *locale = Locale::En;
            }
            (UI::LocaleButtonJa, Interaction::Pressed) => {
                *locale = Locale::Ja;
            }
            (UI::LocaleButtonKo, Interaction::Pressed) => {
                *locale = Locale::Ko;
            }
            _ => { /* empty */ }
        }
    }
}

fn handle_slider_pressed(
    mut selected: ResMut<SelectedSliderCursor>,
    interaction_query: Query<
        (Entity, &UI, &Interaction),
        (With<OptionLevelEntity>, Changed<Interaction>),
    >,
) {
    for (entity, &ui, &interaction) in interaction_query.iter() {
        match (ui, interaction) {
            (UI::BackgroundVolumeSlider, Interaction::Pressed)
            | (UI::EffectVolumeSlider, Interaction::Pressed)
            | (UI::VoiceVolumeSlider, Interaction::Pressed) => {
                selected.set(ui, entity, 0);
                break;
            }
            _ => { /* empty */ }
        }
    }
}

fn handle_slider_pressed_for_moblie(
    touches: Res<Touches>,
    mut selected: ResMut<SelectedSliderCursor>,
    interaction_query: Query<
        (Entity, &UI, &Interaction),
        (With<OptionLevelEntity>, Changed<Interaction>),
    >,
) {
    for touch in touches.iter_just_pressed() {
        for (entity, &ui, &interaction) in interaction_query.iter() {
            match (ui, interaction) {
                (UI::BackgroundVolumeSlider, Interaction::Pressed)
                | (UI::EffectVolumeSlider, Interaction::Pressed)
                | (UI::VoiceVolumeSlider, Interaction::Pressed) => {
                    selected.set(ui, entity, touch.id());
                    return;
                }
                _ => { /* empty */ }
            }
        }
    }
}

fn handle_slider_release(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut selected: ResMut<SelectedSliderCursor>,
) {
    if mouse_buttons.just_released(MouseButton::Left) {
        let _ = selected.take();
    }
}

fn handle_slider_release_for_moblie(
    touches: Res<Touches>,
    mut selected: ResMut<SelectedSliderCursor>,
) {
    if let Some((_, _, id)) = selected.get()
        && touches.just_released(id)
    {
        let _ = selected.take();
    }
}

// --- UPDATE SYSTEMS ---

fn update_volume_value(
    system_volume: Res<SystemVolume>,
    mut query: Query<(&UI, &mut Text), With<OptionLevelEntity>>,
) {
    for (ui, mut text) in query.iter_mut() {
        match ui {
            UI::BackgroundVolume => {
                let percentage = system_volume.get_background().to_linear() * 100.0;
                *text = Text::new(format!("{}", percentage as u8));
            }
            UI::EffectVolume => {
                let percentage = system_volume.get_effect().to_linear() * 100.0;
                *text = Text::new(format!("{}", percentage as u8));
            }
            UI::VoiceVolume => {
                let percentage = system_volume.get_voice().to_linear() * 100.0;
                *text = Text::new(format!("{}", percentage as u8));
            }
            _ => { /* empty */ }
        }
    }
}

fn update_slider_cursor(
    windows: Query<&Window>,
    slider_query: Query<(&ComputedNode, &UiGlobalTransform), With<OptionLevelEntity>>,
    mut handler_query: Query<(&mut Node, &ChildOf), With<OptionLevelEntity>>,
    parent_query: Query<&ChildOf, With<OptionLevelEntity>>,
    mut system_volume: ResMut<SystemVolume>,
    selected: Res<SelectedSliderCursor>,
) {
    let Ok(window) = windows.single() else { return };

    if let Some((ui, entity, _)) = selected.get()
        && let Some(point) = window.physical_cursor_position()
        && let Ok(child_of) = parent_query.get(entity)
        && let Ok((mut node, child_of)) = handler_query.get_mut(child_of.parent())
        && let Ok((computed_node, &ui_transform)) = slider_query.get(child_of.parent())
        && let Some(norm) = computed_node.normalize_point(ui_transform, point)
    {
        let p = (norm.x + 0.5).clamp(0.0, 1.0);
        node.left = Val::Percent(p * 100.0);
        match ui {
            UI::BackgroundVolumeSlider => system_volume.background = (p * 255.0) as u8,
            UI::EffectVolumeSlider => system_volume.effect = (p * 255.0) as u8,
            UI::VoiceVolumeSlider => system_volume.voice = (p * 255.0) as u8,
            _ => { /* empty */ }
        }
    }
}

fn update_slider_cursor_for_moblie(
    windows: Query<&Window>,
    slider_query: Query<(&ComputedNode, &UiGlobalTransform), With<OptionLevelEntity>>,
    mut handler_query: Query<(&mut Node, &ChildOf), With<OptionLevelEntity>>,
    parent_query: Query<&ChildOf, With<OptionLevelEntity>>,
    mut system_volume: ResMut<SystemVolume>,
    touches: Res<Touches>,
    selected: Res<SelectedSliderCursor>,
) {
    let Ok(window) = windows.single() else { return };

    if let Some((ui, entity, touch_id)) = selected.get()
        && let Some(touch) = touches.get_pressed(touch_id)
        && let Ok(child_of) = parent_query.get(entity)
        && let Ok((mut node, child_of)) = handler_query.get_mut(child_of.parent())
        && let Ok((computed_node, &ui_transform)) = slider_query.get(child_of.parent())
        && let Some(norm) =
            computed_node.normalize_point(ui_transform, touch.position() * window.scale_factor())
    {
        let p = (norm.x + 0.5).clamp(0.0, 1.0);
        node.left = Val::Percent(p * 100.0);
        match ui {
            UI::BackgroundVolumeSlider => system_volume.background = (p * 255.0) as u8,
            UI::EffectVolumeSlider => system_volume.effect = (p * 255.0) as u8,
            UI::VoiceVolumeSlider => system_volume.voice = (p * 255.0) as u8,
            _ => { /* empty */ }
        }
    }
}
