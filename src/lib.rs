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
    Interpolate,
    /// Can be used to safely schedule systems after interpolation but before transforms are "locked in" by bevy.
    PostInterpolation,
}

#[derive(Resource, Default)]
pub struct ShouldInterpolateXPBD(pub bool);

fn toggle_should_interp(
    mut should_interp: ResMut<ShouldInterpolateXPBD>,
    key_input: Res<Input<KeyCode>>,
) {
    if key_input.just_pressed(KeyCode::Space) {
        should_interp.0 = !should_interp.0;
        println!("Set 'ShouldInterpolateXPBD' to {:?}", should_interp.0);
    }
}

#[derive(Component)]
pub struct InterpolatedPosition {
    pub target_entity: Entity,
    pub previous_position: Vec3,
}

impl InterpolatedPosition {
    pub fn new(target_entity: Entity) -> Self {
        Self {
            target_entity,
            previous_position: Default::default(),
        }
    }
}

#[derive(Component)]
pub struct InterpolatedRotation {
    pub target_entity: Entity,
    pub previous_rotation: Quat,
}

impl InterpolatedRotation {
    pub fn new(target_entity: Entity) -> Self {
        Self {
            target_entity,
            previous_rotation: Default::default(),
        }
    }
}
#[derive(Component)]
pub struct InterpolatedTransform {
    pub interpolated_entity: Entity,
    pub previous_translation: Vec3,
    pub previous_rotation: Quat,
}

impl InterpolatedTransform {
    pub fn new(interpolated_entity: Entity) -> Self {
        Self {
            interpolated_entity,
            previous_translation: Default::default(),
            previous_rotation: Default::default(),
        }
    }
}

fn copy_position(mut query: Query<(&Position, &mut InterpolatedPosition)>) {
    for (pos, mut interp) in query.iter_mut() {
        interp.previous_position = pos.0;
    }
}

fn copy_rotation(mut query: Query<(&Rotation, &mut InterpolatedRotation)>) {
    for (rot, mut interp) in query.iter_mut() {
        interp.previous_rotation = rot.0;
    }
}

// Runs in PhysicsSchedule before the broadface and before any Transforms are updated by physics
fn copy_old_transforms(mut query: Query<(&Position, &Rotation, &mut InterpolatedTransform)>) {
    for (pos, rot, mut interp) in query.iter_mut() {
        interp.previous_translation = pos.0;
        interp.previous_rotation = rot.0;
    }
}

fn interpolate_position(
    physical_entity_q: Query<(&Position, &InterpolatedPosition)>,
    mut target_entity_q: Query<&mut Transform, Without<InterpolatedPosition>>,
    phys_loop: Res<PhysicsLoop>,
    phys_timestep: Res<PhysicsTimestep>,
    should_interp: Res<ShouldInterpolateXPBD>,
) {
    for (pos, interp_pos) in physical_entity_q.iter() {
        // Get the entity that should store the interpolated values
        let mut target_transform = target_entity_q.get_mut(interp_pos.target_entity).unwrap();

        if should_interp.0 {
            // Get the physics time step
            let time_step = match *phys_timestep {
                PhysicsTimestep::Fixed(value) => value,
                _ => panic!("The 'PhysicsTimestep' resource does not hold the 'Fixed' variant. Cannot interpolate."),
            };

            let lerp_factor = phys_loop.accumulator / time_step;

            target_transform.translation = interp_pos.previous_position.lerp(pos.0, lerp_factor);
        } else {
            target_transform.translation = pos.0;
        }
    }
}

fn interpolate_rotation(
    physical_entity_q: Query<(&Rotation, &InterpolatedRotation)>,
    mut target_entity_q: Query<&mut Transform, Without<InterpolatedRotation>>,
    phys_loop: Res<PhysicsLoop>,
    phys_timestep: Res<PhysicsTimestep>,
    should_interp: Res<ShouldInterpolateXPBD>,
) {
    for (rot, interp_rot) in physical_entity_q.iter() {
        // Get the entity that should store the interpolated values
        let mut target_transform = target_entity_q.get_mut(interp_rot.target_entity).unwrap();

        if should_interp.0 {
            // Get the physics time step
            let time_step = match *phys_timestep {
                PhysicsTimestep::Fixed(value) => value,
                _ => panic!("The 'PhysicsTimestep' resource does not hold the 'Fixed' variant. Cannot interpolate."),
            };

            let lerp_factor = phys_loop.accumulator / time_step;

            target_transform.rotation = interp_rot.previous_rotation.slerp(rot.0, lerp_factor);
        } else {
            target_transform.rotation = rot.0;
        }
    }
}

// Runs in PostUpdate
pub fn update_interpolation(
    mut commands: Commands,
    physical_entity_q: Query<(Option<&Position>, Option<&Rotation>, &InterpolatedTransform)>,
    mut interp_entity_q: Query<&mut Transform, Without<InterpolatedTransform>>,
    phys_loop: Res<PhysicsLoop>,
    phys_timestep: Res<PhysicsTimestep>,
    should_interp: Res<ShouldInterpolateXPBD>,
) {
    // Loop through every entity with an InterpTransform
    for (position, rotation, interp) in physical_entity_q.iter() {
        // Get the entity that should store the interpolated values
        let interp_entity = commands.entity(interp.interpolated_entity).id();
        let mut interp_transform = interp_entity_q.get_mut(interp_entity).unwrap();

        if should_interp.0 {
            // Get the physics time step
            let time_step = match *phys_timestep {
                PhysicsTimestep::Fixed(value) => value,
                _ => panic!("The 'PhysicsTimestep' resource does not hold the 'Fixed' variant. Cannot interpolate."),
            };

            let lerp_factor = phys_loop.accumulator / time_step;

            if let Some(pos) = position {
                // Interpolate translation
                interp_transform.translation = interp.previous_translation.lerp(pos.0, lerp_factor);
            };

            if let Some(rot) = rotation {
                // Interpolate rotation
                interp_transform.rotation = interp.previous_rotation.slerp(rot.0, lerp_factor);
            };
        } else {
            if let Some(pos) = position {
                interp_transform.translation = pos.0;
            };

            if let Some(rot) = rotation {
                interp_transform.rotation = rot.0;
            };
        }
    }
}
