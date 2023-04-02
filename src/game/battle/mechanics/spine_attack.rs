use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{DamageSystem, HurtBox, UpdateSet};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum SpineAttackSystem {
    Update,
}

pub struct SpineAttackPlugin;

impl Plugin for SpineAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spine_attack_update
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(SpineAttackSystem::Update)
                .in_set(UpdateSet)
                .before(DamageSystem::Update),
        );
    }
}

#[derive(Default, Component)]
pub struct SpineAttack {
    pub hurt_box: HurtBox,
}

pub fn spine_attack_update(
    mut spine_events: EventReader<SpineEvent>,
    mut commands: Commands,
    spine_attack_query: Query<&SpineAttack>,
) {
    for spine_event in spine_events.iter() {
        match spine_event {
            SpineEvent::Event { entity, name, .. } => {
                if let Ok(spine_attack) = spine_attack_query.get(*entity) {
                    match name.as_str() {
                        "damage_start" => {
                            if let Some(mut entity_commands) = commands.get_entity(*entity) {
                                entity_commands.insert(spine_attack.hurt_box);
                            }
                        }
                        "damage_end" => {
                            if let Some(mut entity_commands) = commands.get_entity(*entity) {
                                entity_commands.remove::<HurtBox>();
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
