#![allow(unused)]

mod constants;
mod resources;
mod components;
mod systems;

use bevy::DefaultPlugins;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::WindowMode;
use constants::*;
use resources::*;
use crate::components::*;
use crate::systems::control_system::control_system;
use crate::systems::input_system::input_system;
use crate::systems::movement_system::movement_system;
use crate::systems::turning_system::turning_system;
use crate::systems::weapon_system::weapon_system;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Space Invaders!".to_string(),
            width: 640.0,
            height: 480.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
//        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup_actors", SystemStage::single(spawn_player.system()))
        .add_system(input_system.system())
        .add_system(control_system.system())
        .add_system(movement_system.system())
        .add_system(weapon_system.system())
        .add_system(turning_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>) {
    // get window handler
    let mut window = windows.get_primary_mut().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // load resources
    commands.insert_resource(Materials {
        player_materials: asset_server.load(PLAYER_SPRITE).into(),
        projectile_materials: asset_server.load(PROJECTILE_SPRITE).into(),
        player_turn_left_frames: load_animations(PLAYER_TURN_LEFT, 9, &asset_server),
        player_turn_right_frames: load_animations(PLAYER_TURN_RIGHT, 9, &asset_server),
    });

    // store window size
    commands.insert_resource(WinSize {
        width: window.width(),
        height: window.height(),
    });

    // position window
    let mut window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(945, 0));
}

fn load_animations(base_path: &str, num_frames: usize, asset_server: &Res<AssetServer>) -> Vec<Handle<Image>> {
    let mut frames: Vec<Handle<Image>> = Vec::new();
    for index in 0..num_frames {
        let frame_path = format!("{}_{:02}.png", base_path, index);
        frames.push(asset_server.load(frame_path.as_str()).into());
    }
    frames
}

fn spawn_player(
    mut commands: Commands,
    materials: Res<Materials>,
    win_size: Res<WinSize>,
) {
    commands.spawn_bundle(SpriteBundle {
        texture: materials.player_materials.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, -win_size.height * 0.5 + SPRITE_SIZE * SCALE, 10.0),
            scale: Vec3::new(SCALE, SCALE, SCALE),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Player)
        .insert(Movement {
            max_velocity: Vec2::new(100.0, 100.0),
            max_acceleration: Vec2::new(100.0, 100.0),
            ..Default::default()
        })
        .insert(Control::default())
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Weapon {
                    muzzle_point: Vec2::new(0.0, SPRITE_SIZE * SCALE * 0.75),
                    rate_of_fire: 0.5,
                    ..Default::default()
                });
        });
}
