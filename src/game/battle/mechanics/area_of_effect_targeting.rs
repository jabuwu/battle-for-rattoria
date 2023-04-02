use std::collections::HashMap;

use bevy::prelude::*;

use crate::{DamageReceiveEvent, EventSet, Team, UpdateSet};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum AreaOfEffectTargetingSystem {
    Update,
}

pub struct AreaOfEffectTargetingPlugin;

impl Plugin for AreaOfEffectTargetingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AreaOfEffectTargeting>().add_system(
            area_of_effect_targeting_update
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(AreaOfEffectTargetingSystem::Update)
                .in_set(UpdateSet)
                .after(EventSet::<DamageReceiveEvent>::Sender),
        );
    }
}

#[derive(Resource, Default)]
pub struct AreaOfEffectTargeting {
    targets: HashMap<Team, Vec2>,
}

#[derive(Component)]
pub struct Target {
    pub team: Team,
}

impl AreaOfEffectTargeting {
    pub fn get_target(&self, target_team: Team) -> Option<Vec2> {
        if let Some(target) = self.targets.get(&target_team) {
            Some(*target)
        } else {
            None
        }
    }
}

pub fn area_of_effect_targeting_update(
    mut area_of_effect_targeting: ResMut<AreaOfEffectTargeting>,
    target_query: Query<(&Target, &GlobalTransform)>,
) {
    let mut positions_map: HashMap<Team, Vec<Vec2>> = HashMap::new();
    for (target, target_transform) in target_query.iter() {
        let position = target_transform.translation().truncate();
        if position.x * -target.team.move_direction() < 500. {
            if let Some(positions) = positions_map.get_mut(&target.team) {
                positions.push(position);
            } else {
                positions_map.insert(target.team, vec![position]);
            }
        }
    }
    area_of_effect_targeting.targets = HashMap::new();
    for (team, positions) in positions_map.iter() {
        let mut target_position = Vec2::ZERO;
        for position in positions.iter() {
            target_position += *position;
        }
        target_position /= positions.len() as f32;
        area_of_effect_targeting
            .targets
            .insert(*team, target_position);
    }
}
