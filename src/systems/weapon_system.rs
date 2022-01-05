use bevy::prelude::*;

use crate::{Control, Weapon};

pub fn weapon_system(
    time: Res<Time>,
    query: Query<(&Transform, &Children, &Control)>,
    mut q_children: Query<&mut Weapon>,
) {
    for (transform, children, control) in query.iter() {
        if control.fire {
            for &child in children.iter() {
                if let Ok(mut weapon) = q_children.get_mut(child) {
                    weapon.time_since_last_fire += time.delta_seconds();
                    if weapon.time_since_last_fire >= weapon.rate_of_fire {
                        println!("Projectile fired!");
                        weapon.time_since_last_fire = 0.0;
                    }
                }
            }
        }
    }
}
