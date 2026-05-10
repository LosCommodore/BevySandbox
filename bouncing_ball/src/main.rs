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
    // Ground: 600 pixels wide, 20 pixels high
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(600.0, 20.0), // Match collider to sprite
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(600.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -100.0, 0.0),
    ));

    // Bouncing ball: 40 pixels diameter
    commands.spawn((
        RigidBody::Dynamic,
        Collider::circle(20.0), // Radius 20 = Diameter 40
        Sprite {
            color: Color::srgb_u8(255, 0, 0),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 200.0, 0.0),
    ));

    // Camera
    commands.spawn(Camera2d::default());
}
