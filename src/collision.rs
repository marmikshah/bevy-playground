use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::{
    brick::{Brick, HealthPoints},
    bullet::{Bullet, Power},
    components::Velocity,
};

#[derive(Component, Default)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
}

pub struct CollisionEngine;

impl Plugin for CollisionEngine {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(FixedUpdate, check_collisions);
    }
}

pub fn translation_to_xyxy(transform: &Transform) -> Aabb2d {
    let x = transform.translation.x;
    let y = transform.translation.y;
    let width = transform.scale.x;
    let height = transform.scale.y;

    Aabb2d {
        min: Vec2::new(x - width, y - height),
        max: Vec2::new(x + width, y + height),
    }
}

pub fn check_collisions(
    mut commands: Commands,
    bullet: Query<(Entity, &Transform, &Power), With<Bullet>>,
    mut brick: Query<(Entity, &Transform, &mut HealthPoints), With<Brick>>,
    mut events: EventWriter<CollisionEvent>,
) {
    for (bullet_entity, bullet_transform, power) in &bullet {
        for (collider_entity, collider_transform, mut health) in &mut brick {
            let collision = bullet_collision(
                translation_to_xyxy(bullet_transform),
                translation_to_xyxy(collider_transform),
            );

            if let Some(_collision) = collision {
                events.send_default();
                health.0 -= power.0;

                commands.entity(bullet_entity).despawn();

                println!("Collided!, Remaining HP: {}", health.0);

                if health.0 <= 0 {
                    commands.entity(collider_entity).despawn();
                    println!("Despawning");
                }
            }
        }
    }
}

pub fn bullet_collision(bullet: Aabb2d, bbox: Aabb2d) -> Option<Collision> {
    if !bullet.intersects(&bbox) {
        return None;
    }

    Some(Collision::BOTTOM)
}
