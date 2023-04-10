use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{
    AreaOfEffectTargetingPlugin, BannerPlugin, BattlePlugin, BattleSplashPlugin, BattlefieldPlugin,
    DamagePlugin, FeelerPlugin, GameDirectorPlugin, GamePlugin, HealthPlugin, IntermissionPlugin,
    IntroPlugin, MusicPlugin, OutroPlugin, PlanningPlugin, ProjectilePlugin, RewindPlugin,
    SandboxPlugin, SfxPlugin, SpineAttackPlugin, SpineFxPlugin, StartPlugin, UnitPlugin,
};

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group.add(GamePlugin);
        group = group.add(GameDirectorPlugin);
        group = group.add(MusicPlugin);
        group = group.add(SfxPlugin);

        // intro
        group = group.add(IntroPlugin);

        // start
        group = group.add(StartPlugin);

        // intermission
        group = group.add(IntermissionPlugin);

        // planning
        group = group.add(PlanningPlugin);

        // battle
        group = group.add(BattlePlugin);
        group = group.add(BattlefieldPlugin);
        group = group.add(UnitPlugin);
        group = group.add(DamagePlugin);
        group = group.add(HealthPlugin);
        group = group.add(SpineAttackPlugin);
        group = group.add(SpineFxPlugin);
        group = group.add(ProjectilePlugin);
        group = group.add(AreaOfEffectTargetingPlugin);
        group = group.add(FeelerPlugin);
        group = group.add(BattleSplashPlugin);
        group = group.add(BannerPlugin);

        // rewind
        group = group.add(RewindPlugin);

        // outro
        group = group.add(OutroPlugin);

        // sandbox
        group = group.add(SandboxPlugin);

        group
    }
}
