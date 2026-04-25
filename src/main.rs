mod gui;
use bevy::{
    color::palettes::tailwind::SLATE_50,
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    sprite_render::Wireframe2dPlugin,
};
use gui::EventHistory;
use std::time::Duration;

use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use crate::gui::gui_system;

#[derive(Component, Default)]
#[require(Transform)]
struct RectShape {
    width: f32,
    height: f32,
}

#[derive(Component)]
struct SmallBlock;

#[derive(Component)]
struct BigBlock;

#[derive(Component)]
struct Wall;

#[derive(Component, Default)]
struct Impulse {
    mass: f32,
    velocity: f32,
}

#[derive(Resource, Default)]
struct CollisionCounter(usize);

#[derive(Component)]
struct Collidable();

#[derive(Component)]
struct CollisionTextMarker();

#[derive(Event)]
struct CollionsHappended;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut event_history: ResMut<EventHistory>,
) {
    commands.spawn(Camera2d);

    let v_big = -1.;
    let x_big = 300.;
    let width_big: f32 = 200.;
    let height_big: f32 = 200.;

    // --------- BIG BLOCK ---------

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(width_big, height_big))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0., 255., 0.))),
        Transform::from_xyz(x_big, height_big / 2., 0.0),
        RectShape {
            width: width_big,
            height: height_big,
        },
        Impulse {
            velocity: v_big,
            mass: 100.,
        },
        BigBlock,
        Collidable(),
    ));

    // --------- SMALL BLOCK ---------

    let width_small = 50.;
    let height_small = 50.;
    let v_small = 0.;
    let x_small = 0.;
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(width_small, height_small))),
        MeshMaterial2d(materials.add(Color::linear_rgb(255., 0., 0.))),
        Transform::from_xyz(x_small, height_small / 2.0, 0.0),
        RectShape {
            width: width_small,
            height: height_small,
        },
        Impulse {
            velocity: v_small,
            mass: 1.,
        },
        SmallBlock,
        Collidable(),
    ));

    // --------- WAlL ---------

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
        Transform::from_xyz(-300.0, 0.0, 1.0),
        Wall,
        Collidable(),
    ));

    // -- Event History

    event_history.x_big.push(x_big);
    event_history.v_big.push(v_big);
    event_history.x_small.push(x_small);
    event_history.v_small.push(v_small);

    // --------- COLLISION COUNTER ---------

    commands.spawn((
        Node {
            width: percent(100.),
            margin: px(20.).top(),
            ..default()
        },
        Text::new("0"),
        TextLayout::new_with_justify(Justify::Center),
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(SLATE_50.into()),
        CollisionTextMarker(),
    ));
}

fn fixed_update(mut query: Query<(&mut Transform, &mut Impulse)>) {
    for (mut transform, impulse) in &mut query {
        transform.translation.x += impulse.velocity;
    }
}

fn check_collisions(
    mut commands: Commands,
    mut small_block: Single<(&Transform, &RectShape, &mut Impulse), With<SmallBlock>>,
    mut big_block: Single<
        (&Transform, &RectShape, &mut Impulse),
        (With<BigBlock>, Without<Wall>, Without<SmallBlock>),
    >,
    wall: Single<(&Transform, &RectShape), (With<Wall>, Without<SmallBlock>, Without<BigBlock>)>,
    mut event_history: ResMut<EventHistory>,
    mut counter: ResMut<CollisionCounter>,
) {
    fn bounding(t: &Transform, shape: &RectShape) -> Aabb2d {
        let min = Vec2::new(
            t.translation.x - shape.width / 2.,
            t.translation.y - shape.height / 2.,
        );
        let max = Vec2::new(
            t.translation.x + shape.width / 2.,
            t.translation.y + shape.height / 2.,
        );
        Aabb2d { min, max }
    }

    let bb_wall = bounding(&wall.0, &wall.1);
    let bb_small = bounding(&small_block.0, &small_block.1);
    let bb_big = bounding(&big_block.0, &big_block.1);

    if bb_wall.intersects(&bb_small) {
        small_block.2.velocity *= -1.;
    } else if bb_small.intersects(&bb_big) {
        let impulse_small = &mut small_block.2;
        let impulse_big = &mut big_block.2;

        let m1 = impulse_small.mass;
        let m2 = impulse_big.mass;
        let v1 = impulse_small.velocity;
        let v2 = impulse_big.velocity;

        let new_v1 = (m1 * v1 + m2 * (2. * v2 - v1)) / (m1 + m2);
        let new_v2 = (m2 * v2 + m1 * (2. * v1 - v2)) / (m1 + m2);
        impulse_small.velocity = new_v1;
        impulse_big.velocity = new_v2;
    } else {
        return;
    }

    counter.0 += 1;
    commands.trigger(CollionsHappended);

    event_history.x_small.push(small_block.0.translation.x);
    event_history.v_small.push(small_block.2.velocity);
    event_history.x_big.push(big_block.0.translation.x);
    event_history.v_big.push(big_block.2.velocity);
}

fn collision_text_update(
    mut query: Query<&mut Text, With<CollisionTextMarker>>,
    counter: Res<CollisionCounter>,
) {
    for mut span in &mut query {
        span.0 = counter.0.to_string();
    }
}

fn main() {
    App::new()
        .init_resource::<CollisionCounter>()
        .init_resource::<EventHistory>()
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f64(
            1.0 / 100.0,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(Wireframe2dPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (fixed_update, check_collisions).chain())
        .add_systems(EguiPrimaryContextPass, gui_system)
        .add_systems(
            Update,
            collision_text_update.run_if(resource_changed::<CollisionCounter>),
        )
        .run();
}
