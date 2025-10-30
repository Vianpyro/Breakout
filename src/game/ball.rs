use crate::game::paddle::PADDLE_SPEED;
use bevy::prelude::*;
use rand::Rng;

const BALL_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const BALL_SPEED: f32 = PADDLE_SPEED * 0.75;
const BALL_SIZE: Vec2 = Vec2::new(15.0, 15.0);

#[derive(Component)]
pub struct Ball {
    velocity: Vec2,
}

pub fn create_ball(commands: &mut Commands, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) {
    let ball_mesh = meshes.add(Circle::new(BALL_SIZE.x / 1.9));
    let ball_material = materials.add(ColorMaterial::from(BALL_COLOR));

    let mut initial_velocity = Vec2::ZERO;
    randomize_velocity(&mut initial_velocity, BALL_SPEED, BALL_SPEED);

    commands.spawn((
        Mesh2d(ball_mesh),
        MeshMaterial2d(ball_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Ball { velocity: initial_velocity },
    ));
}

pub fn ball_movement_system(time: Res<Time>, viewport: Res<crate::viewport::WindowViewport>, mut query: Query<(&mut Transform, &mut Ball), With<Ball>>) {
    let half_world_width = viewport.half_width;
    let half_world_height = viewport.half_height;
    let delta_time = time.delta_secs();

    for (mut transform, mut ball) in query.iter_mut() {
        transform.translation.x += ball.velocity.x * delta_time;
        transform.translation.y += ball.velocity.y * delta_time;

        // Bounce off walls
        if transform.translation.x - BALL_SIZE.x / 2.0 <= -half_world_width || transform.translation.x + BALL_SIZE.x / 2.0 >= half_world_width {
            ball.velocity.x = -ball.velocity.x;
        }
        if transform.translation.y - BALL_SIZE.y / 2.0 <= -half_world_height || transform.translation.y + BALL_SIZE.y / 2.0 >= half_world_height {
            ball.velocity.y = -ball.velocity.y;
        }
    }
}

fn randomize_velocity(vector: &mut Vec2, x: f32, y: f32) {
    let mut random_thread = rand::rng();
    vector.x = match random_thread.random_bool(0.5) {
        true => x,
        false => -x,
    };
    vector.y = match random_thread.random_bool(0.5) {
        true => y,
        false => -y,
    };
}
