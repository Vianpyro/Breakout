use bevy::prelude::*;

const DEFAULT_VIEWPORT_WIDTH: f32 = 1920.0;
const DEFAULT_VIEWPORT_HEIGHT: f32 = 1080.0;
pub const DEFAULT_VIRTUAL_WIDTH: f32 = 1280.0;
pub const DEFAULT_VIRTUAL_HEIGHT: f32 = 720.0;

// Virtual resolution (in world units) that should be visible on screen regardless
// of the physical pixel resolution. For example, a height of 720 means the camera
// will display 720 world units vertically; the projection scale is adjusted so
// that this maps to the current window pixel height.
#[derive(Resource)]
pub struct VirtualResolution {
    pub width: f32,
    pub height: f32,
}

impl Default for VirtualResolution {
    fn default() -> Self {
        Self {
            width: DEFAULT_VIRTUAL_WIDTH,
            height: DEFAULT_VIRTUAL_HEIGHT,
        }
    }
}

// Strategy for how the virtual resolution should be mapped to the window.
#[derive(Resource, Clone, Copy)]
pub enum ScalingStrategy {
    FixedVertical,
    FixedHorizontal,
    AutoMin,
    AutoMax,
}

impl Default for ScalingStrategy {
    fn default() -> Self {
        ScalingStrategy::FixedVertical
    }
}

// Cached window viewport (world-space half extents + optional raw pixel info).
#[derive(Resource)]
pub struct WindowViewport {
    // world-space half extents (matching camera projection)
    pub half_width: f32,
    pub half_height: f32,
    // raw pixel size and projection scale
    pub pixel_width: f32,
    pub pixel_height: f32,
    pub scale: f32, // camera projection scale (world units per pixel)
}

impl Default for WindowViewport {
    fn default() -> Self {
        Self {
            half_width: DEFAULT_VIEWPORT_WIDTH / 2.0,
            half_height: DEFAULT_VIEWPORT_HEIGHT / 2.0,
            pixel_width: DEFAULT_VIEWPORT_WIDTH,
            pixel_height: DEFAULT_VIEWPORT_HEIGHT,
            scale: 1.0,
        }
    }
}

// Initialize the viewport resource from the primary window. The function sets
// pixel dimensions and computes world-space half extents using the current
// viewport.scale value.
pub fn set_initial_window_viewport(mut viewport: ResMut<WindowViewport>, windows: Query<&Window, With<bevy::window::PrimaryWindow>>) {
    if let Some(window) = windows.iter().next() {
        let scale = viewport.scale;
        viewport.pixel_width = window.width();
        viewport.pixel_height = window.height();
        viewport.scale = scale;
        viewport.half_width = window.width() * scale / 2.0;
        viewport.half_height = window.height() * scale / 2.0;
    }
}

// Update cached viewport only when the PrimaryWindow changes (e.g. resized).
pub fn maybe_update_window_viewport(mut viewport: ResMut<WindowViewport>, windows: Query<&Window, (With<bevy::window::PrimaryWindow>, Changed<Window>)>) {
    if let Some(window) = windows.iter().next() {
        let scale = viewport.scale; // keep existing scale
        viewport.pixel_width = window.width();
        viewport.pixel_height = window.height();
        viewport.half_width = window.width() * scale / 2.0;
        viewport.half_height = window.height() * scale / 2.0;
    }
}

// Called on startup and when the primary window is resized. This system updates the
// camera orthographic projection scale so that the virtual resolution maps to the
// current window pixel dimensions (i.e. world_units_per_pixel = virtual_height / window_pixel_height).
pub fn update_camera_on_resize(
    virtual_resolution: Res<VirtualResolution>,
    strat: Res<ScalingStrategy>,
    mut viewport: ResMut<WindowViewport>,
    windows: Query<&Window, (With<bevy::window::PrimaryWindow>, Changed<Window>)>,
    mut cam_transforms: Query<&mut Transform, With<Camera2d>>,
) {
    if let Some(window) = windows.iter().next() {
        // Compute candidate scales for horizontal and vertical mappings.
        let pixel_width = window.width();
        let pixel_height = window.height();
        let scale_width = virtual_resolution.width / pixel_width; // world units per pixel to satisfy width
        let scale_height = virtual_resolution.height / pixel_height; // world units per pixel to satisfy height

        let final_scale = match *strat {
            ScalingStrategy::FixedVertical => scale_height,
            ScalingStrategy::FixedHorizontal => scale_width,
            ScalingStrategy::AutoMin => scale_width.min(scale_height),
            ScalingStrategy::AutoMax => scale_width.max(scale_height),
        };

        // Apply the scale to each 2D camera's transform so the virtual resolution
        // maps to the window size. This adjusts the camera zoom via Transform.scale
        // rather than mutating projection components.
        for mut t in cam_transforms.iter_mut() {
            t.scale = Vec3::splat(final_scale);
        }

        // update cached viewport
        viewport.pixel_width = pixel_width;
        viewport.pixel_height = pixel_height;
        viewport.scale = final_scale;
        viewport.half_width = pixel_width * final_scale / 2.0;
        viewport.half_height = pixel_height * final_scale / 2.0;
    }
}
