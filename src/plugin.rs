//! Sets up the XPBDInterplationPlugin.

use bevy::{prelude::*, transform::TransformSystem};
#[cfg(feature = "2d")]
use bevy_xpbd_2d::{prelude::*, PhysicsSchedule, PhysicsStepSet};
#[cfg(feature = "3d")]
use bevy_xpbd_3d::{prelude::*, PhysicsSchedule, PhysicsStepSet};

use crate::prelude::*;

pub struct XPBDInterpolationPlugin;

impl Plugin for XPBDInterpolationPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PhysicsSchedule,
            InterpolationCopySet.before(PhysicsStepSet::BroadPhase),
        )
        .add_systems(
            PhysicsSchedule,
            (crate::copy_position, crate::copy_rotation).in_set(InterpolationCopySet),
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
            (crate::interpolate_position, crate::interpolate_rotation)
                .in_set(InterpolationSet::Interpolation),
        );
    }
}
