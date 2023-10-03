use bevy::{prelude::*, transform::TransformSystem};
use bevy_xpbd_3d::{prelude::*, PhysicsSchedule, PhysicsStepSet};

use crate::prelude::*;

pub struct XPBDInterpolationPlugin;

impl Plugin for XPBDInterpolationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShouldInterpolate>();

        app.configure_set(
            PhysicsSchedule,
            InterpolationCopySet.before(PhysicsStepSet::BroadPhase),
        )
        .add_systems(
            PhysicsSchedule,
            crate::copy_position.in_set(InterpolationCopySet),
        );

        app.configure_sets(
            PostUpdate,
            (
                InterpolationSet::Interpolation,
                InterpolationSet::PostInterpolation,
            )
                .chain()
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        )
        .add_systems(
            PostUpdate,
            crate::interpolate_position.in_set(InterpolationSet::Interpolation),
        );
    }
}
