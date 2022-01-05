use bevy::prelude::*;

// game resources
pub struct Materials {
    pub player_materials: Handle<ColorMaterial>,
    pub projectile_materials: Handle<ColorMaterial>,
}

// window size
pub struct WinSize{
    pub width: f32,
    pub height: f32,
}
