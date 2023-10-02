use bevy::prelude::*;
use bevy_xpbd_3d::{prelude::*, PhysicsSchedule, PhysicsStepSet};

use bevy_xpbd_interp::prelude::*;

const PHYSICS_TIMESTEP: f32 = 1. / 10.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            XPBDInterpolationPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            PhysicsSchedule,
            update_box.before(PhysicsStepSet::BroadPhase),
        )
        .insert_resource(PhysicsTimestep::Fixed(PHYSICS_TIMESTEP))
        .run();
}

#[derive(Component)]
pub struct BoxLabel;

fn setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    // Rendered object
    let rendered_box = commands
        .spawn(PbrBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Box::new(1., 1., 1.))),
            material: material_assets.add(StandardMaterial {
                base_color: Color::rgb(0.4, 0.8, 0.6),
                ..default()
            }),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
        .id();

    // Physical object
    commands.spawn((
        RigidBody::Kinematic,
        Position::default(),
        Rotation::default(),
        InterpolatedPosition::new(rendered_box),
        InterpolatedRotation::new(rendered_box),
        BoxLabel,
    ));

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 8.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn update_box(
    mut box_q: Query<(&mut AngularVelocity, &mut LinearVelocity), With<BoxLabel>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let speed = 3.;
    for (mut angular_velocity, mut linear_velocity) in box_q.iter_mut() {
        let mut velocity = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            velocity.y += speed;
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            velocity.x -= speed;
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            velocity.y -= speed;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            velocity.x += speed;
        }

        linear_velocity.0 = velocity;

        angular_velocity.0 = Vec3::from_array([1., 1.5, 2.]) * 0.4;
    }
}
