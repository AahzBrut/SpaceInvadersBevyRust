use bevy::prelude::Component;

#[derive(Debug, Component)]
pub struct Control {
    pub turn_left: bool,
    pub turn_right: bool,
    pub accelerate: bool,
    pub decelerate: bool,
    pub fire: bool,
}

impl Default for Control{
    fn default() -> Self {
        Self{
            turn_left: false,
            turn_right: false,
            accelerate: false,
            decelerate: false,
            fire: false
        }
    }
}