use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const WALL_THICKNESS: f32 = 10.0;

#[derive(Component)]
pub struct Wall;

pub fn create_walls(commands: &mut Commands, viewport: &Res<crate::viewport::WindowViewport>) {
    let half_width = viewport.half_width;
    let half_height = viewport.half_height;

    // Helper closure to spawn a wall with common components
    let mut spawn_wall = |translation: Vec3, collider: Collider| {
        commands.spawn((
            Wall,
            Transform::from_translation(translation),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
            RigidBody::Fixed,
            collider,
            Restitution::coefficient(1.0),
            Friction::coefficient(0.0),
        ));
    };

    // Left wall
    spawn_wall(
        Vec3::new(-half_width - WALL_THICKNESS / 2.0, 0.0, 0.0),
        Collider::cuboid(WALL_THICKNESS / 2.0, half_height),
    );

    // Right wall
    spawn_wall(
        Vec3::new(half_width + WALL_THICKNESS / 2.0, 0.0, 0.0),
        Collider::cuboid(WALL_THICKNESS / 2.0, half_height),
    );

    // Top wall
    spawn_wall(
        Vec3::new(0.0, half_height + WALL_THICKNESS / 2.0, 0.0),
        Collider::cuboid(half_width, WALL_THICKNESS / 2.0),
    );
}
