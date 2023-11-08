// This example sets up a rigidbody that is rotated and moved by bevy_xpbd, and a separate entity with a rendered box and InterpolatedPosition/Rotation components.

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use bevy_xpbd_2d_interp::prelude::*;

const PHYSICS_UPDATE_FREQ: f64 = 10.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            XPBDInterpolationPlugin,
        ))
        .insert_resource(Time::new_with(Physics::fixed_hz(1.0 / PHYSICS_UPDATE_FREQ)))
        .insert_resource(IsInterpolating(true)) // Has no effect on actual interpolation, is just for ui
        .add_systems(Startup, setup)
        .add_systems(Update, (update_box, toggle_interpolation, update_ui))
        .run();
}

fn setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    // The entity being transformed by bevy_xpbd
    let physics_entity = commands
        .spawn((
            RigidBody::Kinematic,
            Position::default(),
            Rotation::default(),
        ))
        .id();

    // Rendered box that holds the interpolated position/rotation
    // (Probably shouldn't be a 3d box since this is the box_2d example)
    commands.spawn((
        PbrBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Box::new(1., 1., 1.))),
            material: material_assets.add(StandardMaterial {
                base_color: Color::rgb(0.4, 0.8, 0.6),
                ..default()
            }),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        InterpolatedPosition::from_source(physics_entity),
        InterpolatedRotation::from_source(physics_entity),
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

    // UI
    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 18.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn update_box(
    mut box_q: Query<(&mut AngularVelocity, &mut LinearVelocity)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let speed = 4.;
    for (mut angular_velocity, mut linear_velocity) in box_q.iter_mut() {
        let mut velocity = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            velocity.y += speed;
        }
        if keyboard_input.pressed(KeyCode::A) {
            velocity.x -= speed;
        }
        if keyboard_input.pressed(KeyCode::S) {
            velocity.y -= speed;
        }
        if keyboard_input.pressed(KeyCode::D) {
            velocity.x += speed;
        }

        linear_velocity.0 = velocity;
        angular_velocity.0 = 0.8;
    }
}

// IsInterpolating has no effect on actual interpolation and is just for ui.
// Toggle interpolation with InterpolatedPosition::pass_raw and InterpolatedRotation::pass_raw instead.
#[derive(Resource)]
struct IsInterpolating(bool);

fn toggle_interpolation(
    mut interp_q: Query<(&mut InterpolatedPosition, &mut InterpolatedRotation)>,
    mut is_interpolating: ResMut<IsInterpolating>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let (mut pos, mut rot) = interp_q.single_mut();
        pos.pass_raw = !pos.pass_raw;
        rot.pass_raw = !rot.pass_raw;

        is_interpolating.0 = !pos.pass_raw;
    }
}

fn update_ui(mut text: Query<&mut Text>, is_interpolating: Res<IsInterpolating>) {
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    text.clear();

    text.push_str("Move box with <WASD>");
    text.push_str("\nToggle interpolation with <Space>");

    if is_interpolating.0 {
        text.push_str("\n\nInterpolation: on");
    } else {
        text.push_str("\n\nInterpolation: off");
    }

    text.push_str(&format!(
        "\nPhysics update frequency: {}hz (See PHYSICS_UPDATE_FREQ const in 'examples/box.rs')",
        PHYSICS_UPDATE_FREQ
    ));
}
