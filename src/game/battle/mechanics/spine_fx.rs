use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{DamageSystem, UpdateSet};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum SpineFxSystem {
    Ready,
    Update,
}

pub struct SpineFxPlugin;

impl Plugin for SpineFxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spine_fx_ready
                .in_set(SpineFxSystem::Ready)
                .in_set(SpineSet::OnReady),
        )
        .add_system(
            spine_fx_update
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(SpineFxSystem::Update)
                .in_set(UpdateSet)
                .before(DamageSystem::Update),
        );
    }
}

#[derive(Default, Component)]
pub struct SpineFx;

pub fn spine_fx_ready(
    mut spine_ready_events: EventReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine, With<SpineFx>>,
) {
    for spine_ready_event in spine_ready_events.iter() {
        if let Ok(mut spine) = spine_query.get_mut(spine_ready_event.entity) {
            let _ = spine
                .animation_state
                .set_animation_by_name(0, "animation", false);
        }
    }
}

pub fn spine_fx_update(
    mut spine_events: EventReader<SpineEvent>,
    mut commands: Commands,
    spine_fx_query: Query<&SpineFx>,
) {
    for spine_event in spine_events.iter() {
        match spine_event {
            SpineEvent::Complete { entity, .. } => {
                if spine_fx_query.contains(*entity) {
                    if let Some(entity_commands) = commands.get_entity(*entity) {
                        entity_commands.despawn_recursive();
                    }
                }
            }
            _ => {}
        }
    }
}
