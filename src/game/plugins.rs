use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{BattlePlugin, GameDirectorPlugin, GamePlugin, PlanningPlugin, UnitPlugin};

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group.add(GamePlugin);
        group = group.add(GameDirectorPlugin);

        // planning
        group = group.add(PlanningPlugin);

        // battle
        group = group.add(BattlePlugin);
        group = group.add(UnitPlugin);

        group
    }
}
