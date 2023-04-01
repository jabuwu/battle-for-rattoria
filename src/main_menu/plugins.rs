use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::MainMenuPlugin;

pub struct MainMenuPlugins;

impl PluginGroup for MainMenuPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group.add(MainMenuPlugin);

        group
    }
}
