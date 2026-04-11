use bevy::{prelude::*, sprite_render::Wireframe2dPlugin};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let my_shape = meshes.add(Rectangle::new(50.0, 100.0));
    let color = Color::linear_rgb(255., 0., 0.);

    commands.spawn((
        Mesh2d(my_shape),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Wireframe2dPlugin::default())
        .add_systems(Startup, setup)
        .run();
}
