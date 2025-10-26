#![windows_subsystem = "windows"]

mod player;
mod viewport;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::WindowMode;
use viewport::{VirtualResolution, WindowViewport};

fn main() {
    App::new()
        .insert_resource(VirtualResolution { width: 1280.0, height: 720.0 })
        .insert_resource(viewport::ScalingStrategy::AutoMin)
        .insert_resource(WindowViewport::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (
                spawn_camera,
                spawn_map,
                viewport::set_initial_window_viewport,
                viewport::update_camera_on_resize,
            ),
        )
        .add_systems(PreUpdate, exit_on_esc_system)
        .add_systems(Update, (viewport::maybe_update_window_viewport, viewport::update_camera_on_resize))
        .add_systems(Update, player::paddle::paddle_movement_system)
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

fn spawn_map(mut _commands: Commands, mut _meshes: ResMut<Assets<Mesh>>, mut _materials: ResMut<Assets<ColorMaterial>>) {
    player::paddle::create_paddle(_commands, _meshes, _materials);
}
