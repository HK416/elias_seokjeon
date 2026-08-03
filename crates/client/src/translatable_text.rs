// Import necessary Bevy modules.
use bevy::prelude::*;

use crate::assets::locale::{Locale, LocalizationAssets, LocalizationData};

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_translatable_text_changed.run_if(resource_changed::<Locale>),
                handle_translatable_text_added,
            ),
        );
    }
}

// --- COMPONENTS ---

#[derive(Component)]
pub struct TranslatableText(pub String);

// --- UPDATE SYSTEMS ---

fn handle_translatable_text_changed(
    locale: Res<Locale>,
    locale_assets: Res<LocalizationAssets>,
    locale_data: Res<Assets<LocalizationData>>,
    mut query: Query<(&mut Text, &TranslatableText)>,
) {
    if let Some(handle) = locale_assets.locale.get(&*locale)
        && let Some(data) = locale_data.get(handle.id())
    {
        for (mut text, translatable_text) in query.iter_mut() {
            if let Some(new_text) = data.0.get(&translatable_text.0) {
                *text = Text::new(new_text);
            } else {
                error!("Locale text not found: {}", translatable_text.0);
            }
        }
    }
}

fn handle_translatable_text_added(
    locale: Res<Locale>,
    locale_assets: Res<LocalizationAssets>,
    locale_data: Res<Assets<LocalizationData>>,
    mut query: Query<(&mut Text, &TranslatableText), Added<TranslatableText>>,
) {
    if let Some(handle) = locale_assets.locale.get(&*locale)
        && let Some(data) = locale_data.get(handle.id())
    {
        for (mut text, translatable_text) in query.iter_mut() {
            if let Some(new_text) = data.0.get(&translatable_text.0) {
                *text = Text::new(new_text);
            } else {
                error!("Locale text not found: {}", translatable_text.0);
            }
        }
    }
}
