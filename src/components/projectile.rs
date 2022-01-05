#[derive(Debug)]
pub struct Projectile{
    pub damage: f32,
}

impl Default for Projectile {
    fn default() -> Self {
        Self{
            damage: 10.0,
        }
    }
}
