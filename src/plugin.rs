use bevy::{prelude::*, transform::TransformSystem};
use bevy_xpbd_3d::{prelude::*, PhysicsSchedule, PhysicsStepSet};

use crate::prelude::*;

pub struct XPBDInterpolationPlugin;

impl Plugin for XPBDInterpolationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShouldInterpolateXPBD(true));

        #[cfg(feature = "enable_toggle_with_space")]
        app.add_systems(Update, toggle_should_interp);

        app.configure_set(
            PhysicsSchedule,
            InterpolationCopySet.before(PhysicsStepSet::BroadPhase),
        )
        .add_systems(
            PhysicsSchedule,
            (crate::copy_position, crate::copy_rotation)
                .chain()
                .in_set(InterpolationCopySet),
        );

        app.configure_sets(
            PostUpdate,
            (
                InterpolationSet::Interpolate,
                InterpolationSet::PostInterpolation,
            )
                .chain()
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        )
        .add_systems(
            PostUpdate,
            (crate::interpolate_position, crate::interpolate_rotation)
                .chain()
                .in_set(InterpolationSet::Interpolate),
        );

        // app.add_systems(
        //     PostUpdate,
        //     crate::update_interpolation.in_set(InterpolationSet),
        // )
        // .add_systems(
        //     PhysicsSchedule,
        //     crate::copy_old_transforms.in_set(InterpolationCopySet),
        // );
    }
}
