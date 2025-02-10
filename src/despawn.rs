use crate::collision_detection::DespawnMarker;
use crate::schedule::InGameSet;
use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_marked_entities, despawn_faraway_entities)
                .chain()
                .in_set(InGameSet::DespawnEntities),
        );
    }
}

fn despawn_marked_entities(mut commands: Commands, query: Query<Entity, With<DespawnMarker>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_faraway_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);
        if distance >= DESPAWN_DISTANCE {
            commands.entity(entity).insert(DespawnMarker);
        }
    }
}
