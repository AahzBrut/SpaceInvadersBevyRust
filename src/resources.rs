use bevy::prelude::*;

// game resources
pub struct Materials {
    pub player_materials: Handle<ColorMaterial>,
    pub projectile_materials: Handle<ColorMaterial>,
    pub player_turn_left_frames: Vec<Handle<ColorMaterial>>,
    pub player_turn_right_frames: Vec<Handle<ColorMaterial>>,
}

// window size
pub struct WinSize {
    pub width: f32,
    pub height: f32,
}
