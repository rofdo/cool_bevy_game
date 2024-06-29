use bevy::prelude::*;
use std::f32::consts::PI;
mod menu;

#[derive(Debug, Default, States, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Menu,
    Playing,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_state::<GameState>(GameState::Menu)
        .add_systems(Startup, setup_camera)
        .add_plugins(menu::menu_plugin)
        .run();
}

fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(10.0, 10.0, 10.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..Default::default()
    });
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/maps/map0.glb#Scene0"),
        ..default()
    });
}

fn despawn_entities_with_component<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
