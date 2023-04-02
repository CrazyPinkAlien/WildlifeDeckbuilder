// Defines the smooth move component used to move something to a target location

use bevy::prelude::{Query, Resource, Component, Vec3, Res, Commands, Entity};
use bevy::transform::components::Transform;

#[derive(Resource)]
pub struct SmoothMoveSpeed(f32);

#[derive(Component)]
pub struct SmoothMove {
    target_location: Vec3,
}

pub fn smooth_move(mut query: Query<(Entity, &SmoothMove, &mut Transform)>, speed: Res<SmoothMoveSpeed>, mut commands: Commands) {
    for (entity, smooth_move, mut transform) in query.iter_mut() {
        // If the entity is at the target location, remove the SmoothMove component
        if transform.translation == smooth_move.target_location {
            commands.entity(entity).remove::<SmoothMove>();
        } else {
            // Otherwise, move the entity towards the target location
            let movement = (smooth_move.target_location - transform.translation) * speed.0;
            transform.translation += movement;
        }
    }
}