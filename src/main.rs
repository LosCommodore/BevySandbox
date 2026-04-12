use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    sprite_render::Wireframe2dPlugin,
};

#[derive(Component, Default)]
#[require(Transform, Impulse)]
struct RectShape {
    width: f32,
    height: f32,
}

#[derive(Component, Default)]
struct Impulse {
    mass: f32,
    velocity: f32,
}

#[derive(Component)]
struct Collidable();

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
        RectShape { width, height },
        Impulse {
            velocity: 10.,
            mass: 1.,
        },
        Collidable(),
    ));

    commands.spawn((
        Mesh2d(my_shape),
        MeshMaterial2d(materials.add(Color::linear_rgb(0., 255., 0.))),
        Transform::from_xyz(100.0, 0.0, 0.0),
        RectShape { width, height },
        Impulse {
            velocity: 10.,
            mass: 1.,
        },
        Collidable(),
    ));

    let wall_width = 100.;
    let wall_height = 400.;

    commands.spawn((
        RectShape {
            width: wall_width,
            height: wall_height,
        },
        Sprite {
            custom_size: Some(Vec2::new(wall_width, wall_height)),
            image: asset_server.load("bricksx64.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0,
            },
            ..default()
        },
        Transform::from_xyz(-200.0, 0.0, 1.0),
        Collidable(),
    ));
}

fn fixed_update(mut query: Query<(&mut Transform, &mut Impulse)>) {
    for (mut transform, mut impulse) in &mut query {
        transform.translation.x += impulse.velocity;
        if transform.translation.x < -100. {
            impulse.velocity *= -1.;
            transform.translation.x = -99.;
        }
        if transform.translation.x > 200. {
            transform.translation.x = 200.;
            impulse.velocity *= -1.;
        }
    }
}

fn check_collisions(
    mut query: Query<(&Transform, &RectShape, Option<&mut Impulse>), With<Collidable>>,
) {
    fn bounding(t: &Transform, boulder: &RectShape) -> Aabb2d {
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
    while let Some(
        [
            (transform, shape, mut impulse),
            (transform2, shape2, mut impulse2),
        ],
    ) = iter.fetch_next()
    {
        let one = bounding(transform, &shape);
        let two = bounding(transform2, &shape2);
        if !one.intersects(&two) {
            continue;
        }
        match (impulse, impulse2) {
            (Some(mut impulse), Some(mut impulse2)) => {
                let m1 = impulse.mass;
                let m2 = impulse2.mass;
                let v1 = impulse.velocity;
                let v2 = impulse2.velocity;

                let new_v1 = (m1 * v1 + m2 * (2. * v2 - v1)) / (m1 + m2);
                let new_v2 = (m2 * v2 + m1 * (2. * v1 - v2)) / (m1 + m2);
                impulse.velocity = new_v1;
                impulse2.velocity = new_v2;
            }
            (Some(mut impulse), None) | (None, Some(mut impulse)) => {
                impulse.velocity *= -1.;
            }
            _ => (),
        }
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
