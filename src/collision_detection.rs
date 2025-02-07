use crate::asteroids::Asteroid;
use crate::spaceship::Spaceship;
use bevy::{
    prelude::*,
    transform,
    utils::{define_label, hashbrown::HashMap},
};

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

#[derive(Component)]
pub struct DespawnMarker;

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (entity_a, transform_a, collider_b) in query.iter() {
        for (entity_b, transform_b, collider_a) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a
                    .translation()
                    .distance(transform_b.translation());
                if distance <= collider_a.radius + collider_b.radius {
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}

fn handle_asteroid_collisions(
    mut commands: Commands,
    asteroid_query: Query<(Entity, &Collider), With<Asteroid>>,
    spaceship_query: Query<Entity, With<Spaceship>>,
) {
    for (entity, collider) in asteroid_query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            if asteroid_query.get(collided_entity).is_ok() {
                continue;
            }
            if let Some(mut entity_cmd) = commands.get_entity(entity) {
                entity_cmd.insert(DespawnMarker);
            }
            if spaceship_query.get(collided_entity).is_ok() {
                continue;
            }
            if let Some(mut entity_cmd) = commands.get_entity(collided_entity) {
                entity_cmd.insert(DespawnMarker);
            }
        }
    }
}
