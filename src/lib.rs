//! **Bevy XPBD Interp** is a simple library for interpolation of [bevy_xpbd](https://github.com/Jondolf/bevy_xpbd/) rigidbodies.
//! It operates by interpolating between the position/rotation of the current and previous physics update based on how much time has accumulated since the last physics update.
//! The interpolated value is then stored in the `Transform` of some separate entity that may hold meshes/cameras etc.

use bevy::prelude::*;
#[cfg(feature = "2d")]
use bevy_xpbd_2d::{math::PI, prelude::*};
#[cfg(feature = "3d")]
use bevy_xpbd_3d::prelude::*;

pub mod plugin;
pub mod prelude;

/// System set running in `PhysicsUpdate` before `PhysicsStepSet::BroadPhase`
#[derive(SystemSet, Debug, PartialEq, Eq, Clone, Hash)]
pub struct InterpolationCopySet;

/// System set running in `PostUpdate` between `PhysicsSet::Sync` and `TransformSystem::TransformPropagate`
#[derive(SystemSet, Debug, PartialEq, Eq, Clone, Hash)]
pub enum InterpolationSet {
    /// Where the interpolation takes place.
    Interpolation,
    /// `bevy_xpbd_interp` schedules nothing here, but it can be used to safely schedule systems after interpolation but before transforms are propagated by bevy.
    /// One use case would be scheduling updating the camera position to follow some interpolated position here.
    PostInterpolation,
}

/// Does not store the actual interpolated position value, but instead the cached position from the previous physics update and the entity holding the `Position` affected by a `Rigidbody`.
/// The interpolated position value is automatically given to the `Transform` of any entity with a `InterpolatedPosition`.
#[cfg(feature = "3d")]
#[derive(Component)]
pub struct InterpolatedPosition {
    pub source: Entity,
    // The position of the source entity the previous physics update.
    pub previous_position: Option<Vec3>,
    // If enabled the source position will be passed through directly without interpolation.
    pub pass_raw: bool,
}

/// Does not store the actual interpolated position value, but instead the cached position from the previous physics update and the entity holding the `Position` affected by a `Rigidbody`.
/// The interpolated position value is automatically given to the `Transform` of any entity with a `InterpolatedPosition`.
#[cfg(feature = "2d")]
#[derive(Component)]
pub struct InterpolatedPosition {
    pub source: Entity,
    // The position of the source entity the previous physics update.
    pub previous_position: Option<Vec2>,
    // If enabled the source position will be passed through directly without interpolation.
    pub pass_raw: bool,
}

impl InterpolatedPosition {
    pub fn from_source(source: Entity) -> Self {
        Self {
            source,
            previous_position: None,
            pass_raw: false,
        }
    }
}

/// Does not store the actual interpolated rotation value, but instead the cached rotation from the previous physics update and the entity holding the ´Rotation´ affected by a `Rigidbody`.
/// The interpolated rotation value is automatically given to the `Transform` of any entity with a `InterpolatedRotation`.
#[cfg(feature = "3d")]
#[derive(Component)]
pub struct InterpolatedRotation {
    pub source: Entity,
    // The rotation of the source entity the previous physics update.
    pub previous_rotation: Option<Quat>,
    // If enabled the source rotation will be passed through directly without interpolation.
    pub pass_raw: bool,
}

/// Does not store the actual interpolated rotation value, but instead the cached rotation from the previous physics update and the entity holding the ´Rotation´ affected by a `Rigidbody`.
/// The interpolated rotation value is automatically given to the `Transform` of any entity with a `InterpolatedRotation`.
#[cfg(feature = "2d")]
#[derive(Component)]
pub struct InterpolatedRotation {
    pub source: Entity,
    // Angle of the rotation of the source entity the previous physics update.
    pub previous_rotation: Option<f32>,
    // If enabled the source rotation will be passed through directly without interpolation.
    pub pass_raw: bool,
}

impl InterpolatedRotation {
    pub fn from_source(source: Entity) -> Self {
        Self {
            source,
            previous_rotation: None,
            pass_raw: false,
        }
    }
}

/// Caches the `Position` value of the source entity for every `InterpolatedPosition`.
/// Runs in `InterpolationCopySet`.
fn copy_position(
    mut interp_position_q: Query<&mut InterpolatedPosition>,
    source_position_q: Query<&Position>,
) {
    for mut interp in interp_position_q.iter_mut() {
        let position = source_position_q.get(interp.source).unwrap();
        interp.previous_position = Some(position.0);
    }
}

/// Caches the `Rotation` value of the source entity for every `InterpolatedRotation`.
/// Runs in `InterpolationCopySet`.
fn copy_rotation(
    mut interp_rotation_q: Query<&mut InterpolatedRotation>,
    source_rotation_q: Query<&Rotation>,
) {
    #[cfg(feature = "2d")]
    for mut interp in interp_rotation_q.iter_mut() {
        let rotation = source_rotation_q.get(interp.source).unwrap();
        interp.previous_rotation = Some(rotation.as_radians());
    }

    #[cfg(feature = "3d")]
    for mut interp in interp_rotation_q.iter_mut() {
        let rotation = source_rotation_q.get(interp.source).unwrap();
        interp.previous_rotation = Some(rotation.0);
    }
}

/// Performs position interpolation and stores the result in the `Transform` of the entity with the `InterpolatedPosition`.
/// Runs in `InterpolationSet::Interpolation`.
fn interpolate_position(
    mut interp_q: Query<(&mut Transform, &InterpolatedPosition)>,
    source_q: Query<&Position>,
    phys_time: Res<Time<Physics>>,
) {
    // Get the physics time-step
    let (delta, overstep) = match phys_time.timestep_mode() {
        TimestepMode::Fixed {
            delta, overstep, ..
        } => (delta.as_secs_f32(), overstep.as_secs_f32()),
        _ => panic!(
            "The 'PhysicsTimestep' resource does not hold the 'Fixed' variant. Cannot interpolate."
        ),
    };

    for (mut transform, interp_position) in interp_q.iter_mut() {
        let current_position = match source_q.get(interp_position.source) {
            Ok(val) => val,
            Err(_) => {
                warn!("Invalid source entity for InterpolatedPosition. The source entity must have a position component.");
                continue;
            }
        };

        #[cfg(feature = "2d")]
        if interp_position.pass_raw || interp_position.previous_position == None {
            // Use the current position of the physics object directly without interpolating.
            transform.translation = Vec3::new(current_position.0.x, current_position.0.y, 0.0);
        } else {
            // Interpolate between the previous and current position of the physics object.
            let lerp_factor = overstep / delta;
            if let Some(previous_position) = interp_position.previous_position {
                let interp = previous_position.lerp(current_position.0, lerp_factor);
                transform.translation = Vec3::new(interp.x, interp.y, 0.0);
            }
        }

        #[cfg(feature = "3d")]
        if interp_position.pass_raw || interp_position.previous_position == None {
            // Use the current position of the physics object directly without interpolating.
            transform.translation = Vec3::new(current_position.0.x, current_position.0.y, 0.0);
        } else {
            // Interpolate between the previous and current position of the physics object.
            let lerp_factor = overstep / delta;
            if let Some(previous_position) = interp_position.previous_position {
                transform.translation = previous_position.lerp(current_position.0, lerp_factor);
            }
        }
    }
}

/// Performs rotation interpolation and stores the result in the `Transform` of the entity with the `InterpolatedRotation`.
/// Runs in `InterpolationSet::Interpolation`.
fn interpolate_rotation(
    mut interp_q: Query<(&mut Transform, &InterpolatedRotation)>,
    source_q: Query<&Rotation>,
    phys_time: Res<Time<Physics>>,
) {
    // Get the physics time-step
    let (delta, overstep) = match phys_time.timestep_mode() {
        TimestepMode::Fixed {
            delta, overstep, ..
        } => (delta.as_secs_f32(), overstep.as_secs_f32()),
        _ => panic!(
            "The 'PhysicsTimestep' resource does not hold the 'Fixed' variant. Cannot interpolate."
        ),
    };

    for (mut transform, interp_rotation) in interp_q.iter_mut() {
        let current_rotation = match source_q.get(interp_rotation.source) {
            Ok(val) => val,
            Err(_) => {
                warn!("Invalid source entity for InterpolatedRotation. The source entity must have a Rotation component.");
                continue;
            }
        };

        #[cfg(feature = "2d")]
        if interp_rotation.pass_raw || interp_rotation.previous_rotation == None {
            // Use the current rotation of the physics object directly without interpolating.
            transform.rotation = Quat::from(*current_rotation);
            // info!("{}", current_rotation.as_radians());
        } else {
            // Interpolate between the previous and current rotation of the physics object.
            let lerp_factor = overstep / delta;
            if let Some(previous_rotation) = interp_rotation.previous_rotation {
                let delta = current_rotation.as_radians() - previous_rotation;
                // Angles are kept between -pi and pi by bevy_xpbd,
                // and this makes sure we are going the correct way when rotating between the second and third quadrant.
                let interpolated_angle = if delta.abs() <= PI {
                    previous_rotation + lerp_factor * delta
                } else {
                    previous_rotation
                        - (delta / delta.abs()) * lerp_factor * (2.0 * PI - delta.abs())
                };

                transform.rotation = Quat::from(Rotation::from_radians(interpolated_angle));
            }
        }

        #[cfg(feature = "3d")]
        if interp_rotation.pass_raw || interp_rotation.previous_rotation == None {
            // Use the current rotation of the physics object directly without interpolating.
            transform.rotation = current_rotation.0;
        } else {
            // Interpolate between the previous and current rotation of the physics object.
            let lerp_factor = overstep / delta;
            if let Some(previous_rotation) = interp_rotation.previous_rotation {
                transform.rotation = previous_rotation.slerp(current_rotation.0, lerp_factor);
            }
        }
    }
}
