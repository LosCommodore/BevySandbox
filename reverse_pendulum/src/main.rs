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
    // Increase gravity for faster bouncing (default is about -9.81, tune this value as needed)
    commands.insert_resource(Gravity::default());

    let ball = commands
        .spawn((
            Mass(5.),
            LinearDamping(0.0),  // Stops velocity decay
            AngularDamping(0.0), // Stops rotation decay
            RigidBody::Dynamic,
            LinearVelocity(Vec2::new(150.0, 0.0)), // initial velocity
            Collider::circle(20.0),
            Restitution::new(1.), // <--- Add this (0.0 = no bounce, 1.0 = perfect bounce)
            Sprite {
                color: Color::srgb_u8(255, 0, 0),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 200.0, 0.0),
            Friction::ZERO,
        ))
        .id();

    // Rod
    let rod = commands
        .spawn((
            Mass(5.),
            LinearDamping(0.0),  // Stops velocity decay
            AngularDamping(0.0), // Stops rotation decay
            RigidBody::Dynamic,
            Restitution::new(1.), // <--- Add this (0.0 = no bounce, 1.0 = perfect bounce)
            //Collider::rectangle(10.0, 200.0),
            Sprite {
                color: Color::srgb_u8(255, 0, 0),
                custom_size: Some(Vec2::new(10.0, 200.0)),
                ..default()
            },
            Friction::ZERO,
            Transform::from_xyz(0.0, 100.0, 0.0),
        ))
        .id();

    // Ground
    let ground = commands
        .spawn((
            LinearDamping(0.0),  // Stops velocity decay
            AngularDamping(0.0), // Stops rotation decay
            RigidBody::Static,
            Collider::rectangle(600.0, 20.0),
            Restitution::new(1.), // Optional: adding some to the ground as well
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(600.0, 20.0)),
                ..default()
            },
            Friction::ZERO,
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();

    commands.spawn(RevoluteJoint::new(ground, rod).with_anchor(Vec2::new(0.0, 10.0)));
    commands.spawn(RevoluteJoint::new(rod, ball).with_anchor(Vec2::new(0.0, 200.0)));

    // Camera
    commands.spawn(Camera2d::default());
}
