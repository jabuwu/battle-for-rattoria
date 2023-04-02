use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{DamageSystem, HurtBox, UpdateSet};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum SpineAttackSystem {
    Ready,
    Update,
}

pub struct SpineAttackPlugin;

impl Plugin for SpineAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spine_attack_ready
                .in_set(SpineAttackSystem::Ready)
                .in_set(SpineSet::OnReady),
        )
        .add_system(
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

pub fn spine_attack_ready(
    mut spine_ready_events: EventReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine, With<SpineAttack>>,
) {
    for spine_ready_event in spine_ready_events.iter() {
        if let Ok(mut spine) = spine_query.get_mut(spine_ready_event.entity) {
            let _ = spine
                .animation_state
                .set_animation_by_name(0, "animation", false);
        }
    }
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
            SpineEvent::Complete { entity, .. } => {
                if spine_attack_query.contains(*entity) {
                    if let Some(entity_commands) = commands.get_entity(*entity) {
                        entity_commands.despawn_recursive();
                    }
                }
            }
            _ => {}
        }
    }
}
