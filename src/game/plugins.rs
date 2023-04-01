use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{BattlePlugin, GamePlugin, UnitPlugin};

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group.add(GamePlugin);

        // battle
        group = group.add(BattlePlugin);
        group = group.add(UnitPlugin);

        group
    }
}
