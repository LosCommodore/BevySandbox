use bevy::{prelude::*, sprite_render::Wireframe2dPlugin};

#[derive(Component)]
struct MyBoulder();

#[derive(Component, Default)]
struct Boulder {
    velocity: f32,
}

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
        MyBoulder(),
        Boulder { velocity: 10. },
    ));
}

fn fixed_update(mut e: Single<(&mut Transform, &mut Boulder), With<MyBoulder>>) {
    e.0.translation.x += e.1.velocity;
    if e.0.translation.x < -100. {
        e.1.velocity *= -1.;
        e.0.translation.x = -99.;
    }
    if e.0.translation.x > 200. {
        e.0.translation.x = 200.;
        e.1.velocity *= -1.;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Wireframe2dPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, fixed_update)
        .run();
}
