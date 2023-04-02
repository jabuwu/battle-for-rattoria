use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{
    BattlePlugin, DamagePlugin, GameDirectorPlugin, GamePlugin, PlanningPlugin, StartPlugin,
    UnitPlugin,
};

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group.add(GamePlugin);
        group = group.add(GameDirectorPlugin);

        // start
        group = group.add(StartPlugin);

        // planning
        group = group.add(PlanningPlugin);

        // battle
        group = group.add(BattlePlugin);
        group = group.add(UnitPlugin);
        group = group.add(DamagePlugin);

        group
    }
}
