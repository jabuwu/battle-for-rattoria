use bevy::prelude::*;

use crate::{SecondOrder, Transform2};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum TargetTransformSystem {
    Update,
}

pub struct TargetTransformPlugin;

impl Plugin for TargetTransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(target_transform_update.in_set(TargetTransformSystem::Update));
    }
}

#[derive(Component)]
pub struct TargetTransform {
    pub second_order: SecondOrder<Vec2>,
    pub target: Vec2,
}

fn target_transform_update(
    mut target_transform_query: Query<(&mut Transform2, &mut TargetTransform)>,
    time: Res<Time>,
) {
    for (mut transform, mut target_transform) in target_transform_query.iter_mut() {
        let target = target_transform.target;
        transform.translation = target_transform
            .second_order
            .update(target, time.delta_seconds());
    }
}
