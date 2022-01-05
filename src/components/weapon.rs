use bevy::prelude::Vec2;

#[derive(Debug)]
pub enum ProjectileType{
    LASER
}

#[derive(Debug)]
pub struct Weapon {
    pub muzzle_point: Vec2,
    pub rate_of_fire: f32,
    pub projectile_velocity: f32,
    pub time_since_last_fire: f32,
    pub projectile_type: ProjectileType,
}

impl Default for Weapon {
    fn default() -> Self {
        Self{
            muzzle_point: Vec2::new(0.0, 0.0),
            rate_of_fire: 0.0,
            projectile_velocity: 100.0,
            time_since_last_fire: f32::MAX,
            projectile_type: ProjectileType::LASER,
        }
    }
}
