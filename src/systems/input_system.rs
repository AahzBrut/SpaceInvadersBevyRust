use bevy::prelude::{Input, KeyCode, Query, Res, Transform, Vec2, With};
use crate::{Movement, Player};

pub fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Movement, &mut Transform), With<Player>>
){
    for (mut movement, mut transform) in query.iter_mut(){
        movement.target = Vec2::new(0.0, 1.0);
        let direction = if keyboard_input.pressed(KeyCode::A) { -1.0 } else if keyboard_input.pressed(KeyCode::D) { 1.0 } else { 0.0 };
        transform.translation.x += direction;
    }
}