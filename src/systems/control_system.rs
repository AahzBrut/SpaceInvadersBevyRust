use bevy::prelude::{Mut, Query, Res, Time, With};
use crate::components::*;

pub fn control_system(
    time: Res<Time>,
    mut query: Query<(&mut Movement, &Control), With<Control>>,
) {
    for (mut movement, control) in query.iter_mut() {
        horizontal_movement(time.delta_seconds(), movement, control)
    }
}

fn horizontal_movement(delta_time: f32, mut movement: Mut<Movement>, control: &Control) {
    if control.turn_left {
        movement.acceleration.x = (movement.acceleration.x - movement.max_acceleration.x * delta_time).clamp(-movement.max_acceleration.x, movement.max_acceleration.x);
        movement.velocity.x = (movement.velocity.x + movement.acceleration.x * delta_time).clamp(-movement.max_velocity.x, movement.max_velocity.x);
    } else if control.turn_right {
        movement.acceleration.x = (movement.acceleration.x + movement.max_acceleration.x * delta_time).clamp(-movement.max_acceleration.x, movement.max_acceleration.x);
        movement.velocity.x = (movement.velocity.x + movement.acceleration.x * delta_time).clamp(-movement.max_velocity.x, movement.max_velocity.x);
    } else {
        if movement.velocity.x.abs() > 0.00001 {
            if movement.velocity.x > 0.0 {
                movement.acceleration.x = if movement.acceleration.x > 0.0 { 0.0 } else { movement.acceleration.x };
                movement.acceleration.x = (movement.acceleration.x - movement.max_acceleration.x * delta_time).clamp(-movement.max_acceleration.x, movement.max_acceleration.x);
                movement.velocity.x = (movement.velocity.x + movement.acceleration.x * delta_time).clamp(-movement.max_velocity.x, movement.max_velocity.x);
            } else {
                movement.acceleration.x = if movement.acceleration.x < 0.0 { 0.0 } else { movement.acceleration.x };
                movement.acceleration.x = (movement.acceleration.x + movement.max_acceleration.x * delta_time).clamp(-movement.max_acceleration.x, movement.max_acceleration.x);
                movement.velocity.x = (movement.velocity.x + movement.acceleration.x * delta_time).clamp(-movement.max_velocity.x, movement.max_velocity.x);
            }
        } else {
            movement.velocity.x = 0.0;
        }
    }
}
