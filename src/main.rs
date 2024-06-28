use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Debug, Resource)]
struct Settings {
    amount: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Settings { amount: 100 })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, settings: Res<Settings>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    for i in 0..settings.amount {
        // spawn white triangle
        let color = Color::WHITE;
        let shape = Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::new(10.0, 10.0),
            Vec2::new(-10.0, 10.0),
            Vec2::new(0.0, -10.0),
        )));
        commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(i as f32, 0f32, 0f32),
            ..Default::default()
        });
    }
}
