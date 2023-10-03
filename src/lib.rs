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

fn copy_position(
    mut interp_position_q: Query<&mut InterpolatedPosition>,
    source_position_q: Query<&Position>,
) {
    for mut interp in interp_position_q.iter_mut() {
        let position = source_position_q.get(interp.source).unwrap();
        interp.previous_position = Some(position.0);
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
        let current_position = source_q.get(interp_position.source).unwrap();

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
