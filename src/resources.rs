use bevy::prelude::*;

// game resources
pub struct Materials {
    pub player_materials: Handle<Image>,
    pub projectile_materials: Handle<Image>,
    pub player_turn_left_frames: Vec<Handle<Image>>,
    pub player_turn_right_frames: Vec<Handle<Image>>,
}

// window size
pub struct WinSize {
    pub width: f32,
    pub height: f32,
}
