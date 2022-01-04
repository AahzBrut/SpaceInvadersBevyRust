use bevy::prelude::{Query, Res, Time, Transform, With};

use crate::{Control, Movement};

pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Movement), With<Movement>>
){
    for (mut transform, movement) in query.iter_mut(){
        transform.translation.x += movement.velocity.x * time.delta_seconds();
    }
}
