use bevy::prelude::{Input, KeyCode, Query, Res, Time, Transform, Vec2, With};
use crate::components::*;

pub fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Control), With<Player>>
){
    for (mut control) in query.iter_mut(){
        control.turn_left = keyboard_input.pressed(KeyCode::A);
        control.turn_right = keyboard_input.pressed(KeyCode::D);
        control.accelerate = keyboard_input.pressed(KeyCode::W);
        control.decelerate = keyboard_input.pressed(KeyCode::S);
        control.fire = keyboard_input.pressed(KeyCode::Space);
    }
}