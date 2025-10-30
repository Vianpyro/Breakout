use crate::viewport::{DEFAULT_VIRTUAL_HEIGHT, DEFAULT_VIRTUAL_WIDTH};
use bevy::prelude::*;

const PADDLE_ACCELERATION: f32 = 2500.0;
const PADDLE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
pub const PADDLE_SIZE: Vec2 = Vec2::new(DEFAULT_VIRTUAL_WIDTH / 10.0, DEFAULT_VIRTUAL_HEIGHT / 40.0);
pub const PADDLE_SPEED: f32 = 750.0;
const PADDLE_START_Y: f32 = -DEFAULT_VIRTUAL_HEIGHT / 2.0 + PADDLE_SIZE.y * 2.0;

#[derive(Component)]
pub struct Paddle {
    pub velocity: f32,
}

pub fn create_paddle(commands: &mut Commands, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) {
    let paddle_mesh = meshes.add(Rectangle::new(PADDLE_SIZE.x, PADDLE_SIZE.y));
    let paddle_material = materials.add(ColorMaterial::from(PADDLE_COLOR));
    commands.spawn((
        Mesh2d(paddle_mesh),
        MeshMaterial2d(paddle_material),
        Transform::from_translation(Vec3::new(0.0, PADDLE_START_Y, 0.0)),
        Paddle { velocity: 0.0 },
    ));
}

pub fn paddle_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    viewport: Res<crate::viewport::WindowViewport>,
    mut query: Query<(&mut Transform, &mut Paddle), With<Paddle>>,
) {
    let half_world_width = viewport.half_width;
    let delta_time = time.delta_secs();

    for (mut transform, mut paddle) in query.iter_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += 1.0;
        }

        // Apply acceleration when input is held
        if direction != 0.0 {
            paddle.velocity += direction * PADDLE_ACCELERATION * delta_time;
            paddle.velocity = paddle.velocity.clamp(-PADDLE_SPEED, PADDLE_SPEED);
        } else {
            // Decelerate towards 0 when no input (simple friction)
            let deceleration = PADDLE_ACCELERATION * delta_time;
            if paddle.velocity.abs() <= deceleration {
                paddle.velocity = 0.0;
            } else {
                paddle.velocity -= paddle.velocity.signum() * deceleration;
            }
        }

        // Move by current velocity
        transform.translation.x += paddle.velocity * delta_time;

        // Clamp to world bounds
        let half_paddle_width = PADDLE_SIZE.x / 2.0;
        transform.translation.x = transform
            .translation
            .x
            .clamp(-half_world_width + half_paddle_width, half_world_width - half_paddle_width);
    }
}
