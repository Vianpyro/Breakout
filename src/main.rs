use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::WindowMode;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                ..default()
            }),
            ..default()
        }))
        .add_systems(PreUpdate, exit_on_esc_system)
        .add_systems(Startup, (spawn_camera, spawn_map))
        .run();
}

fn exit_on_esc_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut app_exit_events: MessageWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn(DirectionalLight::default());
}

fn spawn_map(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let count = 16;
    let ball_mesh = meshes.add(Circle::new(20.0));
    for i in 0..count {
        let color = Color::hsl((i as f32 / count as f32) * 360.0, 1.0, 0.5);
        let ball_material = materials.add(ColorMaterial::from(color));
        commands.spawn((
            Mesh2d(ball_mesh.clone()),
            MeshMaterial2d(ball_material),
            Transform::from_translation(Vec3::new((-8.0 + i as f32) * 40.0, 0.0, 0.0)),
        ));
    }
}
