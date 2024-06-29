use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Debug, Resource)]
struct Settings {
    amount: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Settings { amount: 100 })
        .add_systems(Startup, (setup_camera, setup_map))
        .run();
}

fn setup_camera(mut commands: Commands, settings: Res<Settings>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(10.0, 10.0, 10.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..Default::default()
    });
}

fn setup_map(mut commands: Commands, settings: Res<Settings>, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/maps/map0.glb#Scene0"),
        ..default()
    });
}
