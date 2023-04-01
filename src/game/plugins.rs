use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::GamePlugin;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group.add(GamePlugin);

        group
    }
}
