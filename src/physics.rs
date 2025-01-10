/*
 * This module contains all the systems that are related to
 * movement, collisions, etc that can either be chained
 * together or called at FixedUpdate to improve speed.
 *
 */
use bevy::prelude::*;

use crate::sprites;

pub struct PhysicsManager;

impl Plugin for PhysicsManager {
    fn build(&self, app: &mut App) {
        app.add_event::<collision::CollisionEvent>().add_systems(
            FixedUpdate,
            (
                sprites::player::move_player,
                sprites::bullet::move_bullet,
                sprites::brick::move_brick,
                collision::check_collisions,
            )
                .chain(),
        );
    }
}

mod collision {
    use bevy::{
        math::bounding::{Aabb2d, IntersectsVolume},
        prelude::*,
    };

    use crate::{
        entities::{HealthPoints, Power},
        sprites::brick::Brick,
        sprites::bullet::Bullet,
    };

    #[derive(Event, Default)]
    pub struct CollisionEvent;

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
                let did_collide = bullet_collision(
                    translation_to_xyxy(bullet_transform),
                    translation_to_xyxy(collider_transform),
                );

                if did_collide {
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

    pub fn bullet_collision(bullet: Aabb2d, bbox: Aabb2d) -> bool {
        if !bullet.intersects(&bbox) {
            return false;
        }

        true
    }
}
