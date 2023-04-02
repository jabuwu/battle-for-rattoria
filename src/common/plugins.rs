use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_audio_plus::AudioPlusPlugin;
use bevy_egui::EguiPlugin;
use bevy_spine::SpinePlugin;

use crate::{
    DebugDrawPlugin, DialoguePlugin, FixedTimestepPlugin, ForceRatioPlugin, FramesToLivePlugin,
    SetsPlugin, Transform2Plugin, YOrderPlugin,
};

pub struct CommonPlugins;

impl PluginGroup for CommonPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group.add(AudioPlusPlugin);
        group = group.add(SpinePlugin);
        group = group.add(EguiPlugin);

        group = group.add(SetsPlugin);

        group = group.add(FixedTimestepPlugin);
        group = group.add(ForceRatioPlugin);
        group = group.add(Transform2Plugin);
        group = group.add(YOrderPlugin);
        group = group.add(FramesToLivePlugin);
        group = group.add(DebugDrawPlugin);
        group = group.add(DialoguePlugin);

        group
    }
}
