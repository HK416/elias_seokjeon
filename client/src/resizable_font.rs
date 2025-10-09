// Import necessary Bevy modules.
use bevy::{prelude::*, window::WindowResized};

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (setup_font_size, update_font_size));
    }
}

// --- COMPONENTS ---

#[derive(Component)]
pub enum ResizableFont {
    Vertical { resolution: f32, size: f32 },
}

impl ResizableFont {
    pub const fn vertical(resolution: f32, size: f32) -> Self {
        Self::Vertical { resolution, size }
    }
}

// --- UPDATE SYSTEMS ---

fn setup_font_size(
    windows: Query<&Window>,
    mut query: Query<(&mut TextFont, &ResizableFont), Added<ResizableFont>>,
) {
    let Ok(window) = windows.single() else { return };
    for (mut text_font, resizable) in query.iter_mut() {
        match *resizable {
            ResizableFont::Vertical { resolution, size } => {
                let font_size = window.height() / resolution * size;
                text_font.font_size = font_size;
            }
        }
    }
}

fn update_font_size(
    mut reader: MessageReader<WindowResized>,
    mut query: Query<(&mut TextFont, &ResizableFont)>,
) {
    for event in reader.read() {
        for (mut text_font, resizable) in query.iter_mut() {
            match *resizable {
                ResizableFont::Vertical { resolution, size } => {
                    let font_size = event.height / resolution * size;
                    text_font.font_size = font_size;
                }
            }
        }
    }
}
