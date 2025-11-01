use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const WALL_THICKNESS: f32 = 10.0;

#[derive(Component)]
pub struct Wall;

pub fn create_walls(commands: &mut Commands, viewport: Res<crate::viewport::WindowViewport>) {
    let half_width = viewport.half_width;
    let half_height = viewport.half_height;

    // Left wall
    commands.spawn((
        Wall,
        Transform::from_translation(Vec3::new(-half_width - WALL_THICKNESS / 2.0, 0.0, 0.0)),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        RigidBody::Fixed,
        Collider::cuboid(WALL_THICKNESS / 2.0, half_height),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
    ));

    // Right wall
    commands.spawn((
        Wall,
        Transform::from_translation(Vec3::new(half_width + WALL_THICKNESS / 2.0, 0.0, 0.0)),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        RigidBody::Fixed,
        Collider::cuboid(WALL_THICKNESS / 2.0, half_height),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
    ));

    // Top wall
    commands.spawn((
        Wall,
        Transform::from_translation(Vec3::new(0.0, half_height + WALL_THICKNESS / 2.0, 0.0)),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        RigidBody::Fixed,
        Collider::cuboid(half_width, WALL_THICKNESS / 2.0),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
    ));
}
