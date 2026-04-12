use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    sprite_render::Wireframe2dPlugin,
};

#[derive(Component, Default)]
#[require(Transform)]
struct Boulder {
    velocity: f32,
    mass: f32,
    width: f32,
    height: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    let width = 50.;
    let height = 100.;

    let my_shape = meshes.add(Rectangle::new(width, height));

    commands.spawn((
        Mesh2d(my_shape.clone()),
        MeshMaterial2d(materials.add(Color::linear_rgb(255., 0., 0.))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Boulder {
            velocity: 10.,
            mass: 1.,
            width,
            height,
        },
    ));

    commands.spawn((
        Mesh2d(my_shape),
        MeshMaterial2d(materials.add(Color::linear_rgb(0., 255., 0.))),
        Transform::from_xyz(100.0, 0.0, 0.0),
        Boulder {
            velocity: 10.,
            mass: 1.,
            width,
            height,
        },
    ));

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(100., 400.)),
            image: asset_server.load("bricksx64.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0,
            },
            ..default()
        },
        Transform::from_xyz(-200.0, 0.0, 1.0),
    ));
}

fn fixed_update(mut query: Query<(&mut Transform, &mut Boulder)>) {
    for (mut transform, mut boulder) in &mut query {
        transform.translation.x += boulder.velocity;
        if transform.translation.x < -100. {
            boulder.velocity *= -1.;
            transform.translation.x = -99.;
        }
        if transform.translation.x > 200. {
            transform.translation.x = 200.;
            boulder.velocity *= -1.;
        }
    }
}

fn check_collisions(mut query: Query<(&Transform, &mut Boulder)>) {
    fn bounding(t: &Transform, boulder: &Boulder) -> Aabb2d {
        let top_left = Vec2::new(
            t.translation.x - boulder.width / 2.,
            t.translation.y + boulder.height / 2.,
        );
        let bottom_right = Vec2::new(
            t.translation.x + boulder.width / 2.,
            t.translation.y + boulder.height / 2.,
        );
        Aabb2d {
            min: top_left,
            max: bottom_right,
        }
    }
    let mut iter = query.iter_combinations_mut();
    while let Some([(transform, mut boulder), (transform2, mut boulder2)]) = iter.fetch_next() {
        let one = bounding(transform, &boulder);
        let two = bounding(transform2, &boulder2);
        if !one.intersects(&two) {
            continue;
        }
        let m1 = boulder.mass;
        let m2 = boulder2.mass;
        let v1 = boulder.velocity;
        let v2 = boulder2.velocity;

        let new_v1 = (m1 * v1 + m2 * (2. * v2 - v1)) / (m1 + m2);
        let new_v2 = (m2 * v2 + m1 * (2. * v1 - v2)) / (m1 + m2);
        boulder.velocity = new_v1;
        boulder2.velocity = new_v2;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Wireframe2dPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (fixed_update, check_collisions).chain())
        .run();
}
