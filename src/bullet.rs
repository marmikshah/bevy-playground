use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{collision::Collider, components::Velocity, constants::WINDOW_HEIGHT};

const BULLET_SHAPE: bevy::prelude::Vec2 = Vec2::new(2.0, 10.0);
const BULLET_COLOR: bevy::prelude::Color = Color::srgb(1.0, 0.0, 0.0);

#[derive(Component)]
pub struct Power(pub i32);

pub struct BulletPlugin;

#[derive(Bundle)]
pub struct BulletBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub bullet: Bullet,
    pub power: Power,
    pub velocity: Velocity,
    pub collider: Collider,
}

impl BulletBundle {
    pub fn new(x: f32, y: f32, speed: f32) -> Self {
        Self {
            sprite: Sprite::from_color(BULLET_COLOR, Vec2::ONE),
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                scale: Vec3::new(BULLET_SHAPE.x, BULLET_SHAPE.y, 0.),
                ..Default::default()
            },
            bullet: Bullet,
            power: Power(1),
            velocity: Velocity(Vec2::new(0., speed)),
            collider: Collider,
        }
    }
}

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_bullet.run_if(on_timer(Duration::from_secs_f32(0.1))),
        )
        .add_systems(Update, move_bullet);
    }
}

#[derive(Component)]
pub struct Bullet;

pub fn spawn_bullet(
    mut commands: Commands,
    mut query: Query<&Transform, With<crate::player::Player>>,
) {
    if let Ok(transform) = query.get_single() {
        let x = transform.translation.x;
        let y = transform.translation.y + transform.scale.y + crate::constants::BULLET_OFFSET;

        let bundle = BulletBundle::new(x, y, 250.);
        commands.spawn(bundle);
    }
}

pub fn move_bullet(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Velocity), With<Bullet>>,
    time: Res<Time>,
) {
    for (entity, mut transform, velocity) in query.iter_mut() {
        transform.translation.y += velocity.0.y * time.delta_secs();

        if transform.translation.y > WINDOW_HEIGHT * 2. {
            commands.entity(entity).despawn();
        }
    }
}
