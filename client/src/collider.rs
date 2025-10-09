// Import necessary Bevy modules.
use bevy::prelude::*;
use serde::Deserialize;

// --- PLUGIN ---

pub struct InnerPlugin;

impl Plugin for InnerPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        #[cfg(not(feature = "no-debuging-gizmo"))]
        app.add_systems(
            PostUpdate,
            (handle_collider_gizmo, draw_collider_gizmo),
        );
    }
}

// --- COMPONENTS ---

#[derive(Debug, Clone, Copy, Component, Deserialize)]
pub enum Collider2d {
    Box { offset: Vec2, size: Vec2 },
    Circle { offset: Vec2, radius: f32 },
}

impl Collider2d {
    pub fn contains(this: (&Self, &GlobalTransform), point: Vec2) -> bool {
        match this.0 {
            Collider2d::Box { offset, size } => {
                let pos = this.1.translation().xy();
                let min = pos + offset - size * 0.5;
                let max = pos + offset + size * 0.5;

                point.x >= min.x && point.x <= max.x && point.y >= min.y && point.y <= max.y
            }
            Collider2d::Circle { offset, radius } => {
                let center = this.1.translation().xy() + offset;
                let distance_squared = (center - point).length_squared();
                distance_squared <= radius * radius
            }
        }
    }

    pub fn intersects(this: (&Self, &GlobalTransform), other: (&Self, &GlobalTransform)) -> bool {
        match (this.0, other.0) {
            (
                Collider2d::Box {
                    offset: a_offset,
                    size: a_size,
                },
                Collider2d::Box {
                    offset: b_offset,
                    size: b_size,
                },
            ) => {
                let a_pos = this.1.translation().xy();
                let a_min = a_pos + *a_offset - *a_size * 0.5;
                let a_max = a_pos + *a_offset + *a_size * 0.5;
                let b_pos = other.1.translation().xy();
                let b_min = b_pos + *b_offset - *b_size * 0.5;
                let b_max = b_pos + *b_offset + *b_size * 0.5;

                a_max.x >= b_min.x && a_min.x <= b_max.x && a_max.y >= b_min.y && a_min.y <= b_max.y
            }
            (
                Collider2d::Box {
                    offset: a_offset,
                    size: a_size,
                },
                Collider2d::Circle {
                    offset: b_offset,
                    radius: b_radius,
                },
            ) => {
                let a_pos = this.1.translation().xy();
                let a_min = a_pos + *a_offset - *a_size * 0.5;
                let a_max = a_pos + *a_offset + *a_size * 0.5;
                let b_center = other.1.translation().xy() + *b_offset;
                let closest_point = b_center.clamp(a_min, a_max);

                let distance_squared = (closest_point - b_center).length_squared();
                distance_squared <= b_radius * b_radius
            }
            (Collider2d::Circle { .. }, Collider2d::Box { .. }) => {
                Collider2d::intersects(other, this)
            }
            (
                Collider2d::Circle {
                    offset: a_offset,
                    radius: a_radius,
                },
                Collider2d::Circle {
                    offset: b_offset,
                    radius: b_radius,
                },
            ) => {
                let a_center = this.1.translation().xy() + *a_offset;
                let b_center = other.1.translation().xy() + *b_offset;

                let distance_squared = (a_center - b_center).length_squared();
                let radius_sum = a_radius + b_radius;

                distance_squared <= radius_sum * radius_sum
            }
        }
    }
}

// --- POSTUPDATE SYSTEMS ---

#[cfg(not(feature = "no-debuging-gizmo"))]
fn handle_collider_gizmo(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    if keyboard_input.just_pressed(KeyCode::F4) {
        for (_, config, _) in config_store.iter_mut() {
            config.enabled ^= true;
        }
    }
}

#[cfg(not(feature = "no-debuging-gizmo"))]
fn draw_collider_gizmo(mut gizmos: Gizmos, query: Query<(&Collider2d, &GlobalTransform)>) {
    const GIZMO_COLOR: Color = Color::srgb(1.0, 1.0, 0.0);
    for (collider, transform) in query.iter() {
        match collider {
            Collider2d::Box { offset, size } => {
                let center = transform.translation().xy() + *offset;
                gizmos.rect_2d(center, *size, GIZMO_COLOR);
            }
            Collider2d::Circle { offset, radius } => {
                let center = transform.translation().xy() + *offset;
                gizmos.circle_2d(center, *radius, GIZMO_COLOR);
            }
        }
    }
}
