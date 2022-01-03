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
use crate::components::movement::Movement;
use crate::components::player::Player;
use crate::systems::input_system::input_system;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Space Invaders!".to_string(),
            width: 640.0,
            height: 480.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup_actors", SystemStage::single(spawn_player.system()))
        .add_system(input_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>) {
    // get window handler
    let mut window = windows.get_primary_mut().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // load resources
    commands.insert_resource(Materials {
        player_materials: materials.add(asset_server.load(PLAYER_SPRITE).into())
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

fn spawn_player(
    mut commands: Commands,
    materials: Res<Materials>,
    win_size: Res<WinSize>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.player_materials.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, -win_size.height * 0.5 + SPRITE_SIZE * SCALE, 10.0),
            scale: Vec3::new(SCALE, SCALE, SCALE),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Player)
        .insert(Movement::default());
}
