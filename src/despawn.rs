use crate::asteroids::DespawnMarker;
use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {}
}

fn despawn_marked_entities(mut commands: Commands, query: Query<Entity, With<DespawnMarker>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
