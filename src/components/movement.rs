use bevy::prelude::{Component, Vec2, Vec3};

#[derive(Debug, Component)]
pub struct Movement{
    pub max_velocity: Vec2,
    pub max_acceleration: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2
}

impl Default for Movement {
    fn default() -> Self {
        Self{
            max_velocity: Vec2::new(0.0, 0.0),
            max_acceleration: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            acceleration: Vec2::new(0.0, 0.0)
        }
    }
}
