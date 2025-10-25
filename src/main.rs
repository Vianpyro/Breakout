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
        .run();
}

fn exit_on_esc_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut app_exit_events: MessageWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }
}
