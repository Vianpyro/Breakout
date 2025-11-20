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
        .add_systems(PreUpdate, (exit_on_esc_system, game::ball::stick_ball_system))
        .add_systems(
            Update,
            (
                game::paddle::paddle_movement_system,
                game::ball::launch_ball_system,
                game::ball::ball_physics_system,
                game::ball::ball_paddle_collision_system,
                game::ball::ball_brick_collision_system,
                viewport::maybe_update_window_viewport,
                viewport::update_camera_on_resize,
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
    game::walls::create_walls(&mut commands, &viewport);

    for i in 0..(crate::game::brick::DEFAULT_BRICK_SIZE.y as i32 + 5) {
        let height = viewport.half_height - (i as f32 * (crate::game::brick::DEFAULT_BRICK_SIZE.y + 5.0));
        spawn_brick_row(&mut commands, &mut *meshes, &mut *materials, &viewport, height);
    }
}

fn spawn_brick_row(commands: &mut Commands, meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>, viewport: &Res<WindowViewport>, height: f32) {
    let brick_size = game::brick::DEFAULT_BRICK_SIZE;
    let hit_points = 3;
    let spacing = 5.0;

    let half_brick = brick_size.x / 2.0;
    let left_limit = -viewport.half_width + half_brick;
    let right_limit = viewport.half_width - half_brick;

    // If the brick is wider than the available space, place a single center brick.
    if left_limit > right_limit {
        let position = Vec2::new(0.0, 0.0);
        game::brick::create_brick(commands, meshes, materials, position, brick_size, hit_points);
        return;
    }

    let mut x_positions: Vec<f32> = Vec::new();
    x_positions.push(0.0);

    let mut k = 1;
    loop {
        let offset = k as f32 * (brick_size.x + spacing);
        let right_x = offset;
        let left_x = -offset;

        let mut added = false;

        if right_x <= right_limit {
            x_positions.push(right_x);
            added = true;
        }

        if left_x >= left_limit {
            x_positions.push(left_x);
            added = true;
        }

        if !added {
            break;
        }

        k += 1;
    }

    for x in x_positions.iter() {
        let position = Vec2::new(*x, height);
        game::brick::create_brick(commands, meshes, materials, position, brick_size, hit_points);
    }
}
