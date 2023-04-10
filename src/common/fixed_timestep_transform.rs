use std::hash::Hash;

use bevy::{
    prelude::*,
    transform::systems::{propagate_transforms, sync_simple_transforms},
};
use fixed_timestep::FixedSet;

use crate::transform2_propagate;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum FixedTransformSystem {
    Transform2Propagate,
    TransformPropagate,
}

pub struct FixedTimestepTransformPlugin;

impl Plugin for FixedTimestepTransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            transform2_propagate
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(FixedTransformSystem::Transform2Propagate)
                .in_base_set(FixedSet::PostUpdate)
                .before(FixedTransformSystem::TransformPropagate),
            sync_simple_transforms
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(FixedTransformSystem::TransformPropagate)
                .in_base_set(FixedSet::PostUpdate),
            propagate_transforms
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(FixedTransformSystem::TransformPropagate)
                .in_base_set(FixedSet::PostUpdate),
        ));
    }
}
