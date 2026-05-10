use avian2d::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Ground: static rigid body with a rectangle collider
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(10.0, 0.1),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(10.0, 0.1)),
            ..default()
        },
        Transform::from_xyz(0.0, -0.05, 0.0),
    ));

    // Bouncing ball: dynamic rigid body with a circle collider
    commands.spawn((
        RigidBody::Dynamic,
        Collider::circle(0.5),
        Sprite {
            color: Color::srgb_u8(255, 0, 0),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
    ));

    // Camera
    commands.spawn(Camera2d::default());
}
