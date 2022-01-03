use bevy::prelude::Vec2;

pub struct Movement{
    pub velocity: Vec2,
    pub target: Vec2,
    pub acceleration: Vec2
}

impl Default for Movement {
    fn default() -> Self {
        Self{
            velocity: Vec2::new(0.0, 0.0),
            target: Vec2::new(0.0, 0.0),
            acceleration: Vec2::new(1.0, 1.0)
        }
    }
}
