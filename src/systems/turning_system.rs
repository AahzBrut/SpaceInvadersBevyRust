use bevy::prelude::*;
use crate::{Materials, Movement, Player};

pub fn turning_system(
    materials: Res<Materials>,
    mut query: Query<(&mut Handle<ColorMaterial>, &Movement), With<Player>>,
) {
    for (mut sprite, movement) in query.iter_mut() {
        animate_turn(&mut sprite, movement, &materials);
    }
}

fn animate_turn(sprite: &mut Handle<ColorMaterial>, movement: &Movement, materials: &Res<Materials>) {
    let num_stages = materials.player_turn_left_frames.len() as f32 - 1.0;
    let stage = if movement.acceleration.x == 0.0 {0} else {(num_stages * (movement.acceleration.x.abs() / movement.max_acceleration.x)) as usize};
    if movement.acceleration.x > 0.0 {
        *sprite = materials.player_turn_right_frames.get(stage).unwrap().clone();
    } else {
        *sprite = materials.player_turn_left_frames.get(stage).unwrap().clone();
    }
}
