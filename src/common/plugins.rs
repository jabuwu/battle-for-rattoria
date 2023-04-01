use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_audio_plus::AudioPlusPlugin;

use crate::ForceRatioPlugin;

pub struct CommonPlugins;

impl PluginGroup for CommonPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group.add(AudioPlusPlugin);

        group = group.add(ForceRatioPlugin);

        group
    }
}
