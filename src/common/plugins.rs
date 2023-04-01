use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_audio_plus::AudioPlusPlugin;
use bevy_spine::SpinePlugin;

use crate::{FixedTimestepPlugin, ForceRatioPlugin, Transform2Plugin, YOrderPlugin};

pub struct CommonPlugins;

impl PluginGroup for CommonPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group.add(AudioPlusPlugin);
        group = group.add(SpinePlugin);

        group = group.add(FixedTimestepPlugin);
        group = group.add(ForceRatioPlugin);
        group = group.add(Transform2Plugin);
        group = group.add(YOrderPlugin);

        group
    }
}
