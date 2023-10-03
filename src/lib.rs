use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub mod plugin;
pub mod prelude;

/// Runs in PhysicsUpdate before PhysicsStepSet::BroadPhase
#[derive(SystemSet, Debug, PartialEq, Eq, Clone, Hash)]
pub struct InterpolationCopySet;

/// Runs in PostUpdate between PhysicsSet::Sync and TransformSystem::TransformPropagate
#[derive(SystemSet, Debug, PartialEq, Eq, Clone, Hash)]
pub enum InterpolationSet {
    Interpolation,
    /// Can be used to safely schedule systems after interpolation but before transforms are propagated by bevy.
    PostInterpolation,
}

#[derive(Component)]
pub struct InterpolatedPosition {
    pub source: Entity,
    // The position of the source entity the previous physics update.
    pub previous_position: Option<Vec3>,
    // If enabled the source position will be passed through directly without interpolation.
    pub pass_through: bool,
}

impl InterpolatedPosition {
    pub fn new(source: Entity) -> Self {
        Self {
            source,
            previous_position: None,
            pass_through: false,
        }
    }
}

#[derive(Component)]
pub struct InterpolatedRotation {
    pub source: Entity,
    // The rotation of the source entity the previous physics update.
    pub previous_rotation: Option<Quat>,
    // If enabled the source rotation will be passed through directly without interpolation.
    pub pass_through: bool,
}

impl InterpolatedRotation {
    pub fn new(source: Entity) -> Self {
        Self {
            source,
            previous_rotation: None,
            pass_through: false,
        }
    }
}

fn copy_position(
    mut interp_position_q: Query<&mut InterpolatedPosition>,
    source_position_q: Query<&Position>,
) {
    for mut interp in interp_position_q.iter_mut() {
        let position = source_position_q.get(interp.source).unwrap();
        interp.previous_position = Some(position.0);
    }
}

fn copy_rotation(
    mut interp_rotation_q: Query<&mut InterpolatedRotation>,
    source_rotation_q: Query<&Rotation>,
) {
    for mut interp in interp_rotation_q.iter_mut() {
        let rotation = source_rotation_q.get(interp.source).unwrap();
        interp.previous_rotation = Some(rotation.0);
    }
}

fn interpolate_position(
    mut interp_q: Query<(&mut Transform, &InterpolatedPosition)>,
    source_q: Query<&Position>,
    phys_loop: Res<PhysicsLoop>,
    phys_timestep: Res<PhysicsTimestep>,
) {
    // Get the physics time-step
    let time_step = match *phys_timestep {
        PhysicsTimestep::Fixed(value) => value,
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

        if interp_position.pass_through || interp_position.previous_position == None {
            // Use the current position of the physics object directly without interpolating.
            transform.translation = current_position.0;
        } else {
            // Interpolate between the previous and current position of the physics object.
            // This means that the interpolated position will not be completely up-to-date.
            let lerp_factor = phys_loop.accumulator / time_step;
            if let Some(previous_position) = interp_position.previous_position {
                transform.translation = previous_position.lerp(current_position.0, lerp_factor);
            }
        }
    }
}

fn interpolate_rotation(
    mut interp_q: Query<(&mut Transform, &InterpolatedRotation)>,
    source_q: Query<&Rotation>,
    phys_loop: Res<PhysicsLoop>,
    phys_timestep: Res<PhysicsTimestep>,
) {
    // Get the physics time-step
    let time_step = match *phys_timestep {
        PhysicsTimestep::Fixed(value) => value,
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

        if interp_rotation.pass_through || interp_rotation.previous_rotation == None {
            // Use the current rotation of the physics object directly without interpolating.
            transform.rotation = current_rotation.0;
        } else {
            // Interpolate between the previous and current rotation of the physics object.
            // This means that the interpolated rotation will not be completely up-to-date.
            let lerp_factor = phys_loop.accumulator / time_step;
            if let Some(previous_rotation) = interp_rotation.previous_rotation {
                transform.rotation = previous_rotation.slerp(current_rotation.0, lerp_factor);
            }
        }
    }
}
