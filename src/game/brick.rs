use crate::viewport::{DEFAULT_VIRTUAL_HEIGHT, DEFAULT_VIRTUAL_WIDTH};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const DEFAULT_BRICK_SIZE: Vec2 = Vec2::new(DEFAULT_VIRTUAL_WIDTH / 30.0, DEFAULT_VIRTUAL_HEIGHT / 40.0);

#[derive(Component)]
pub struct Brick {
    pub hits_remaining: u8,
}

pub fn create_brick(commands: &mut Commands, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>, position: Vec2, mut size: Vec2, hits: u8) {
    if size == Vec2::ZERO {
        size = DEFAULT_BRICK_SIZE
    }

    let brick_mesh = meshes.add(Rectangle::new(size.x, size.y));
    let brick_material = materials.add(ColorMaterial::from(Color::srgb(0.8, 0.2, 0.2)));

    commands.spawn((
        Brick { hits_remaining: hits },
        Mesh2d(brick_mesh),
        MeshMaterial2d(brick_material),
        Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        RigidBody::Fixed,
        Collider::cuboid(size.x / 2.0, size.y / 2.0),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
    ));
}

pub fn hit_brick(commands: &mut Commands, bricks: &mut Query<&mut Brick>, entity: Entity) {
    if let Ok(mut brick) = bricks.get_mut(entity) {
        if brick.hits_remaining > 1 {
            brick.hits_remaining -= 1;
        } else {
            commands.entity(entity).despawn();
        }
    }
}
