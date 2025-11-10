use crate::game::paddle::{PADDLE_SIZE, PADDLE_SPEED};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const BALL_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
pub const BALL_SPEED: f32 = PADDLE_SPEED * 0.75;
pub const BALL_RADIUS: f32 = PADDLE_SIZE.y / 2.0;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct BallHeld;

pub fn create_ball(commands: &mut Commands, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) {
    let ball_mesh = meshes.add(Circle::new(BALL_RADIUS));
    let ball_material = materials.add(ColorMaterial::from(BALL_COLOR));

    // While the ball is held, make it kinematic so we manually position it.
    commands
        .spawn((
            // Rendering
            Mesh2d(ball_mesh),
            MeshMaterial2d(ball_material),
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .insert((
            // Game logic
            Ball, BallHeld,
        ))
        .insert((
            // Physics
            RigidBody::KinematicPositionBased,
            Collider::ball(BALL_RADIUS),
            Velocity::default(),
            Restitution::coefficient(1.0),
            Friction::coefficient(0.0),
            GravityScale(0.0),
            ActiveEvents::COLLISION_EVENTS,
        ));
}

pub fn ball_physics_system(
    viewport: Res<crate::viewport::WindowViewport>,
    mut commands: Commands,
    mut ball_query: Query<(Entity, &mut Transform, &mut Velocity), With<Ball>>,
    _time: Res<Time>,
) {
    let half_world_width = viewport.half_width;
    let half_world_height = viewport.half_height;

    for (entity, mut transform, mut velocity) in ball_query.iter_mut() {
        // Maintain constant speed
        let current_speed = velocity.linvel.length();
        if current_speed != 0.0 {
            velocity.linvel = velocity.linvel.normalize() * BALL_SPEED;
        }

        // Bounce off walls (left and right)
        if transform.translation.x - BALL_RADIUS <= -half_world_width && velocity.linvel.x < 0.0 {
            velocity.linvel.x = -velocity.linvel.x;
            transform.translation.x = -half_world_width + BALL_RADIUS;
        }
        if transform.translation.x + BALL_RADIUS >= half_world_width && velocity.linvel.x > 0.0 {
            velocity.linvel.x = -velocity.linvel.x;
            transform.translation.x = half_world_width - BALL_RADIUS;
        }

        // Bounce off top wall
        if transform.translation.y + BALL_RADIUS >= half_world_height && velocity.linvel.y > 0.0 {
            velocity.linvel.y = -velocity.linvel.y;
            transform.translation.y = half_world_height - BALL_RADIUS;
        }

        // Reset ball position if it goes below the paddle (game over condition)
        if transform.translation.y - BALL_RADIUS <= -half_world_height {
            velocity.linvel = Vec2::ZERO;
            velocity.angvel = 0.0;
            commands.entity(entity).insert(RigidBody::KinematicPositionBased).insert(BallHeld);
        }
    }
}

pub fn ball_paddle_collision_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut ball_query: Query<&mut Velocity, With<Ball>>,
    paddle_query: Query<(&Transform, &crate::game::paddle::Paddle), (With<crate::game::paddle::Paddle>, Without<Ball>)>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(h1, h2, _) = collision_event {
            // Check if collision is between ball and paddle
            let (ball_entity, paddle_entity) = if ball_query.get(*h1).is_ok() && paddle_query.get(*h2).is_ok() {
                (*h1, *h2)
            } else if ball_query.get(*h2).is_ok() && paddle_query.get(*h1).is_ok() {
                (*h2, *h1)
            } else {
                continue;
            };

            if let (Ok(mut ball_velocity), Ok((_paddle_transform, paddle))) = (ball_query.get_mut(ball_entity), paddle_query.get(paddle_entity)) {
                // Modify ball velocity based on paddle movement
                let paddle_velocity_influence = paddle.velocity * 0.3; // 30% of paddle velocity
                ball_velocity.linvel.x += paddle_velocity_influence;

                // Ensure ball bounces upward
                if ball_velocity.linvel.y < 0.0 {
                    ball_velocity.linvel.y = -ball_velocity.linvel.y;
                }

                // Normalize to maintain speed
                ball_velocity.linvel = ball_velocity.linvel.normalize() * BALL_SPEED;
            }
        }
    }
}

// While the ball has the `BallHeld` marker, keep it positioned just above the paddle.
pub fn stick_ball_system(
    paddle_query: Query<&Transform, (With<crate::game::paddle::Paddle>, Without<Ball>)>,
    mut ball_query: Query<&mut Transform, (With<Ball>, With<BallHeld>)>,
) {
    let paddle_tf = match paddle_query.single() {
        Ok(t) => t,
        Err(_) => return,
    };

    // Position the ball centered on the paddle and just above it.
    let paddle_top = paddle_tf.translation.y + (crate::game::paddle::PADDLE_SIZE.y / 2.0);
    for mut ball_tf in ball_query.iter_mut() {
        ball_tf.translation.x = paddle_tf.translation.x;
        ball_tf.translation.y = paddle_top + BALL_RADIUS + 1.0;
    }
}

pub fn launch_ball_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    paddle_query: Query<&crate::game::paddle::Paddle>,
    mut held_balls: Query<(Entity, &mut Velocity), (With<Ball>, With<BallHeld>)>,
) {
    if !keyboard_input.just_pressed(KeyCode::ArrowUp) {
        return;
    }

    let paddle_vel = paddle_query.single().map(|p| p.velocity).unwrap_or(0.0);

    for (entity, mut velocity) in held_balls.iter_mut() {
        // Switch to dynamic so physics takes over
        commands.entity(entity).remove::<BallHeld>().insert(RigidBody::Dynamic);

        // Give initial velocity: primarily upwards, with a little horizontal influence
        let init_x = paddle_vel * 0.3;
        velocity.linvel = Vec2::new(init_x, BALL_SPEED);
        velocity.angvel = 0.0;
    }
}
