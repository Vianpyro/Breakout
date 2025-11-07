use crate::game::paddle::{PADDLE_SIZE, PADDLE_SPEED};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const BALL_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
pub const BALL_SPEED: f32 = PADDLE_SPEED * 0.75;
pub const BALL_RADIUS: f32 = PADDLE_SIZE.y / 2.0;

#[derive(Component)]
pub struct Ball;

pub fn create_ball(commands: &mut Commands, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>) {
    let ball_mesh = meshes.add(Circle::new(BALL_RADIUS));
    let ball_material = materials.add(ColorMaterial::from(BALL_COLOR));

    let mut initial_velocity = Vec2::ZERO;
    randomize_velocity(&mut initial_velocity, BALL_SPEED, BALL_SPEED);

    commands.spawn((
        Ball,
        Mesh2d(ball_mesh),
        MeshMaterial2d(ball_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        RigidBody::Dynamic,
        Collider::ball(BALL_RADIUS),
        Velocity {
            linvel: initial_velocity,
            angvel: 0.0,
        },
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        GravityScale(0.0),
        ActiveEvents::COLLISION_EVENTS,
    ));
}

pub fn ball_physics_system(
    viewport: Res<crate::viewport::WindowViewport>,
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    _time: Res<Time>,
) {
    let half_world_width = viewport.half_width;
    let half_world_height = viewport.half_height;

    for (mut transform, mut velocity) in ball_query.iter_mut() {
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
            transform.translation = Vec3::new(0.0, 0.0, 0.0);
            randomize_velocity(&mut velocity.linvel, BALL_SPEED, BALL_SPEED);
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
