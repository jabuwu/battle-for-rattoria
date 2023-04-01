use bevy::prelude::*;

use crate::{AddFixedEvent, UnitSpawnEvent};

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_event::<BattleStartEvent>()
            .add_system(battle_start.in_schedule(CoreSchedule::FixedUpdate));
    }
}

#[derive(Default)]
pub struct BattleStartEvent;

fn battle_start(
    mut start_events: EventReader<BattleStartEvent>,
    mut unit_spawn_events: EventWriter<UnitSpawnEvent>,
) {
    for _ in start_events.iter() {
        unit_spawn_events.send_default();
    }
}
