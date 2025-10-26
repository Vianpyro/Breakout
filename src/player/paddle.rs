use bevy::prelude::*;

const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec2 = Vec2::new(1280.0 / 2.0, 720.0 / 2.0);
const PADDLE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const PADDLE_SPEED: f32 = 650.0;

#[derive(Component)]
pub struct Paddle;

pub fn create_paddle(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let paddle_mesh = meshes.add(Rectangle::new(PADDLE_SIZE.x, PADDLE_SIZE.y));
    let paddle_material = materials.add(ColorMaterial::from(PADDLE_COLOR));
    commands.spawn((
        Mesh2d(paddle_mesh),
        MeshMaterial2d(paddle_material),
        Transform::from_translation(Vec3::new(0.0, PADDLE_START_Y, 0.0)),
        Paddle,
    ));
}

pub fn paddle_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    viewport: Res<crate::viewport::WindowViewport>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let half_world_width = viewport.half_width;

    for mut transform in query.iter_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += 1.0;
        }
        let delta_x = direction * PADDLE_SPEED * time.delta_secs();
        transform.translation.x += delta_x;

        let half_paddle_width = PADDLE_SIZE.x / 2.0;
        transform.translation.x = transform
            .translation
            .x
            .clamp(-half_world_width + half_paddle_width, half_world_width - half_paddle_width);
    }
}
