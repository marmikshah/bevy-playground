use std::{num, time::Duration};

use bevy::{prelude::*, scene::ron::de, time::common_conditions::on_timer};

use crate::{components::Velocity, constants::WINDOW_HEIGHT};

#[derive(Resource, Default, DerefMut, Deref)]
pub struct TotalBricks(pub i32);

pub struct BrickPlugin;
impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TotalBricks>()
            .add_systems(
                Update,
                spawn_brick.run_if(on_timer(Duration::from_secs_f32(2.0))),
            )
            .add_systems(Update, move_brick);
    }
}

#[derive(Component)]
pub struct HealthPoints(i32);

#[derive(Component)]
pub struct Brick;

#[derive(Bundle)]
pub struct BrickBundle {
    sprite: Sprite,
    transform: Transform,
    velocity: Velocity,
    hp: HealthPoints,
    brick: Brick,
}

impl BrickBundle {
    pub fn new(x: f32, y: f32, hp: HealthPoints, velocity: Velocity) -> Self {
        Self {
            sprite: Sprite::from_color(Color::srgb(0., 0.25, 0.), Vec2::ONE),
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                scale: Vec3::new(10., 10., 0.0),
                ..Default::default()
            },
            velocity: velocity,
            hp: hp,
            brick: Brick,
        }
    }
}

fn spawn_brick(mut commands: Commands, mut num_bricks: ResMut<TotalBricks>) {
    // println!("Current Bricks: {}", num_bricks.0);
    if num_bricks.0 < 3 {
        let velocity = Velocity(Vec2::new(0., -500.));
        let hp: HealthPoints = HealthPoints(100);
        let brick = BrickBundle::new(0., WINDOW_HEIGHT, hp, velocity);
        println!("Spawning Brick!");
        commands.spawn(brick);
        num_bricks.0 += 1;
    }
}

fn move_brick(mut query: Query<(&mut Transform, &mut Velocity), With<Brick>>, time: Res<Time>) {
    let gravity: f32 = -9.8 * 2.0;
    let damping: f32 = 0.97;
    let bottom_limit: f32 = -(WINDOW_HEIGHT / 2.0);
    let top_limit: f32 = WINDOW_HEIGHT / 2.0;

    for (mut transform, mut velocity) in query.iter_mut() {
        velocity.0.y += gravity * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();

        if transform.translation.y <= bottom_limit {
            transform.translation.y = bottom_limit;
            velocity.0.y = -velocity.0.y * damping;

            if velocity.0.y.abs() < 0.1 {
                velocity.0.y = 0.0;
            }
        }

        if transform.translation.y >= top_limit {
            transform.translation.y = top_limit;
            velocity.0.y = 0.0;
        }
    }
}
