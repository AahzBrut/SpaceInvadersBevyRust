use bevy::prelude::*;

use crate::{Materials, SCALE};
use crate::components::*;

pub fn weapon_system(
    mut commands: Commands,
    materials: Res<Materials>,
    time: Res<Time>,
    query: Query<(&Transform, &Children, &Control)>,
    mut q_children: Query<&mut Weapon>,
) {
    for (transform, children, control) in query.iter() {
        for &child in children.iter() {
            if let Ok(mut weapon) = q_children.get_mut(child) {
                weapon.time_since_last_fire += time.delta_seconds();
                if control.fire && weapon.time_since_last_fire >= weapon.rate_of_fire {
                    fire_projectile(&mut commands, &materials, transform, weapon);
                }
            }
        }
    }
}

fn fire_projectile(commands: &mut Commands, materials: &Res<Materials>, transform: &Transform, mut weapon: Mut<Weapon>) {
    println!("Projectile fired!");
    weapon.time_since_last_fire = 0.0;
    let pos_x = transform.translation.x + weapon.muzzle_point.x;
    let pos_y = transform.translation.y + weapon.muzzle_point.y;
    commands
        .spawn_bundle(
            SpriteBundle {
                texture: materials.projectile_materials.clone(),
                transform: Transform {
                    translation: Vec3::new(pos_x, pos_y, 1.0),
                    scale: Vec3::new(SCALE, SCALE, SCALE),
                    ..Default::default()
                },
                ..Default::default()
            }
        )
        .insert(Projectile { ..Default::default() })
        .insert(Movement {
            velocity: Vec2::new(0.0, weapon.projectile_velocity),
            ..Default::default()
        });
}
