// Defines the smooth move component used to move something to a target location

use bevy::prelude::{Query, Component, Vec3, Commands, Entity};
use bevy::transform::components::Transform;

static SMOOTH_MOVE_SPEED: f32 = 0.5;

#[derive(Component)]
pub struct SmoothMove {
    target_location: Vec3,
}

pub fn smooth_move(mut query: Query<(Entity, &SmoothMove, &mut Transform)>, mut commands: Commands) {
    for (entity, smooth_move, mut transform) in query.iter_mut() {
        // If the entity is at the target location, remove the SmoothMove component
        if transform.translation == smooth_move.target_location {
            commands.entity(entity).remove::<SmoothMove>();
        } else {
            // Otherwise, move the entity towards the target location
            let movement = (smooth_move.target_location - transform.translation) * SMOOTH_MOVE_SPEED;
            transform.translation += movement;
        }
    }
}