#![allow(unused)]

use bevy::DefaultPlugins;
use bevy::prelude::*;

const PLAYER_SPRITE: &str = "./player/player.png";
const SPRITE_SIZE: f32 = 256.0;
const SCALE: f32 = 0.125;

pub struct Materials {
    player_materials: Handle<ColorMaterial>
}

struct WinSize{
    width: f32,
    height: f32
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Space Invaders!".to_string(),
            width: 598.0,
            height: 376.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup_actors", SystemStage::single(spawn_player.system()))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    // get window handler
    let mut window = windows.get_primary_mut().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // load resources
    commands.insert_resource(Materials{
        player_materials: materials.add(asset_server.load(PLAYER_SPRITE).into())
    });

    // store window size
    commands.insert_resource(WinSize{
        width: window.width(),
        height: window.height()
    });

    // position window
    let mut window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(1000, 0));
}

fn spawn_player(
    mut commands: Commands,
    materials: Res<Materials>,
    win_size: Res<WinSize>
) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.player_materials.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, -win_size.height * 0.5 + SPRITE_SIZE * SCALE, 10.0),
            scale: Vec3::new(SCALE, SCALE, SCALE),
            ..Default::default()
        },
        ..Default::default()
    });
}
