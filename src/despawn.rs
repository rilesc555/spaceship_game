use crate::collision_detection::DespawnMarker;
use crate::schedule::InGameSet;
use bevy::prelude::*;

pub struct DespawnPlugin;

const DESPAWN_DISTANCE: f32 = 100.0;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (mark_faraway_entities, despawn_marked_entities)
                .chain()
                .in_set(InGameSet::DespawnEntities),
        );
    }
}

fn despawn_marked_entities(mut commands: Commands, query: Query<Entity, With<DespawnMarker>>) {
    //for entity in query.iter() {
    //    commands.entity(entity).despawn_recursive();
    query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    })
}

fn mark_faraway_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);
        if distance >= DESPAWN_DISTANCE {
            commands.entity(entity).try_insert(DespawnMarker);
        }
    }
}
