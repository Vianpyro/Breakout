#![windows_subsystem = "windows"]

mod game;
mod viewport;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_rapier2d::prelude::*;
use viewport::{VirtualResolution, WindowViewport};

fn main() {
    App::new()
        .insert_resource(VirtualResolution::default())
        .insert_resource(viewport::ScalingStrategy::AutoMin)
        .insert_resource(WindowViewport::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
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
        .add_systems(
            Update,
            (
                game::paddle::paddle_movement_system,
                game::ball::ball_physics_system,
                game::ball::ball_paddle_collision_system,
            ),
        )
        .run();
}

fn exit_on_esc_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn(DirectionalLight::default());
}

fn spawn_map(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, viewport: Res<WindowViewport>) {
    game::paddle::create_paddle(&mut commands, &mut meshes, &mut materials);
    game::ball::create_ball(&mut commands, &mut meshes, &mut materials);
    game::walls::create_walls(&mut commands, viewport);
}
