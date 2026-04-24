use std::time::Duration;

//use anyhow::Result;
use bevy::{
    color::palettes::tailwind::SLATE_50,
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    sprite_render::Wireframe2dPlugin,
};
use bevy_egui::{EguiContexts, egui};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use egui_plot::{Line, Plot, PlotPoints};

#[derive(Component, Default)]
#[require(Transform)]
struct RectShape {
    width: f32,
    height: f32,
}

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

fn plot_system(mut contexts: EguiContexts) -> Result<()> {
    egui::Window::new("Plot").show(contexts.ctx_mut()?, |ui| {
        let sin: PlotPoints = (0..1000)
            .map(|i| {
                let x = i as f64 * 0.01;
                [x, x.sin()]
            })
            .collect();

        let line = Line::new("my_line", sin);

        Plot::new("plot")
            .view_aspect(2.0)
            .show(ui, |plot_ui| plot_ui.line(line));
    });
    Ok(())
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

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(width, height))),
        MeshMaterial2d(materials.add(Color::linear_rgb(255., 0., 0.))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RectShape { width, height },
        Impulse {
            velocity: 0.,
            mass: 1.,
        },
        Collidable(),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(width, height))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0., 255., 0.))),
        Transform::from_xyz(300.0, 0.0, 0.0),
        RectShape { width, height },
        Impulse {
            velocity: -1.,
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
        Transform::from_xyz(-300.0, 0.0, 1.0),
        Collidable(),
    ));

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
    mut query: Query<(&Transform, &RectShape, Option<&mut Impulse>), With<Collidable>>,
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

    let mut iter = query.iter_combinations_mut();
    while let Some([(transform, shape, impulse), (transform2, shape2, impulse2)]) =
        iter.fetch_next()
    {
        let one = bounding(transform, &shape);
        let two = bounding(transform2, &shape2);
        if !one.intersects(&two) {
            continue;
        }
        counter.0 += 1;
        commands.trigger(CollionsHappended);

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

            (Some(mut impulse), None) => {
                impulse.velocity *= -1.;
            }
            (None, Some(mut impulse)) => {
                impulse.velocity *= -1.;
            }

            _ => (),
        }
    }
}

fn ui_example_system(mut contexts: EguiContexts) -> Result<()> {
    egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
        ui.label("world");
    });
    Ok(())
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
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f64(
            1.0 / 100.0,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(Wireframe2dPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (fixed_update, check_collisions).chain())
        .add_systems(EguiPrimaryContextPass, ui_example_system)
        .add_systems(
            Update,
            collision_text_update.run_if(resource_changed::<CollisionCounter>),
        )
        .run();
}
