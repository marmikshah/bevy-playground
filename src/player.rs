use crate::entities::Velocity;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, change_direction));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub velocity: Velocity,
    pub player: Player,
}

impl PlayerBundle {
    pub fn new() -> Self {
        Self {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., -220., 0.),
                scale: Vec3::new(20., 20., 0.),
                ..Default::default()
            },
            velocity: Velocity(Vec2::new(200., 0.)),
            player: Player,
        }
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::new());
}

pub fn move_player(
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
    window: Query<&Window>,
    time: Res<Time>,
) {
    if let Ok((mut velocity, mut transform)) = query.get_single_mut() {
        transform.translation.x += velocity.0.x * time.delta_secs();

        if transform.translation.x < -(window.single().width() / 2.0) {
            velocity.0.x = f32::abs(velocity.0.x);
        } else if transform.translation.x > window.single().width() / 2.0 {
            velocity.0.x = -f32::abs(velocity.0.x);
        }
    }
}

pub fn change_direction(
    mut query: Query<(&mut Velocity, &mut Transform, &Player)>,
    keys: Res<ButtonInput<KeyCode>>,
    window: Query<&Window>,
) {
    if let Ok((mut velocity, transform, player)) = query.get_single_mut() {
        if keys.pressed(KeyCode::KeyA) && transform.translation.x > -(window.single().width() / 2.)
        {
            velocity.0.x = f32::abs(velocity.0.x) * -1.;
        }

        if keys.pressed(KeyCode::KeyD) && transform.translation.x < window.single().width() / 2. {
            velocity.0.x = f32::abs(velocity.0.x);
        }
    }
}
