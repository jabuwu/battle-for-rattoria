use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    AddFixedEvent, AssetLibrary, Depth, EventSet, SpawnSet, UpdateSet, DEPTH_BATTLE_SPLASH,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum BattleSplashSystem {
    Spawn,
    OnReady,
    Play,
    Events,
}

pub struct BattleSplashPlugin;

impl Plugin for BattleSplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_event::<BattleSplashSpawnEvent>()
            .add_fixed_event::<BattleSplashPlayEvent>()
            .add_fixed_event::<BattleSplashEndedEvent>()
            .add_system(
                battle_splash_spawn
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(BattleSplashSystem::Spawn)
                    .in_set(SpawnSet)
                    .after(EventSet::<BattleSplashSpawnEvent>::Sender),
            )
            .add_system(
                battle_splash_on_ready
                    .in_set(BattleSplashSystem::OnReady)
                    .in_set(SpineSet::OnReady),
            )
            .add_system(
                battle_splash_play
                    .in_set(BattleSplashSystem::OnReady)
                    .after(EventSet::<BattleSplashPlayEvent>::Sender)
                    .in_set(UpdateSet),
            )
            .add_system(
                battle_splash_events
                    .in_set(BattleSplashSystem::Events)
                    .in_set(EventSet::<BattleSplashEndedEvent>::Sender)
                    .in_set(UpdateSet),
            );
    }
}

#[derive(Component)]
pub struct BattleSplash {
    play_battle_start: bool,
}

#[derive(Default)]
pub struct BattleSplashSpawnEvent {
    pub play_battle_start: bool,
}

pub struct BattleSplashPlayEvent {
    pub kind: BattleSplashKind,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum BattleSplashKind {
    Start,
    Victory,
    Defeat,
}

#[derive(Default)]
pub struct BattleSplashEndedEvent;

fn battle_splash_spawn(
    mut commands: Commands,
    mut spawn_events: EventReader<BattleSplashSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    for spawn_event in spawn_events.iter() {
        commands.spawn((
            SpineBundle {
                skeleton: asset_library.spine_battle_splash.clone(),
                ..Default::default()
            },
            BattleSplash {
                play_battle_start: spawn_event.play_battle_start,
            },
            Depth::from(DEPTH_BATTLE_SPLASH),
        ));
    }
}

fn battle_splash_on_ready(
    mut spine_ready_events: EventReader<SpineReadyEvent>,
    mut spine_query: Query<(&mut Spine, &BattleSplash)>,
) {
    for spine_ready_event in spine_ready_events.iter() {
        if let Ok((mut spine, battle_splash)) = spine_query.get_mut(spine_ready_event.entity) {
            if battle_splash.play_battle_start {
                let _ = spine
                    .animation_state
                    .set_animation_by_name(0, "battle_start", false);
            } else {
                let _ = spine
                    .animation_state
                    .set_animation_by_name(0, "init", false);
            }
        }
    }
}

fn battle_splash_play(
    mut play_events: EventReader<BattleSplashPlayEvent>,
    mut spine_query: Query<&mut Spine, With<BattleSplash>>,
) {
    for play_event in play_events.iter() {
        for mut spine in spine_query.iter_mut() {
            let animation = match play_event.kind {
                BattleSplashKind::Start => "battle_start",
                BattleSplashKind::Victory => "victory",
                BattleSplashKind::Defeat => "defeat",
            };
            let _ = spine
                .animation_state
                .set_animation_by_name(0, animation, false);
        }
    }
}

fn battle_splash_events(
    mut spine_events: EventReader<SpineEvent>,
    mut battle_splash_ended_events: EventWriter<BattleSplashEndedEvent>,
    battle_splash_query: Query<&BattleSplash>,
) {
    for spine_event in spine_events.iter() {
        match spine_event {
            SpineEvent::Complete { entity, .. } => {
                if battle_splash_query.contains(*entity) {
                    battle_splash_ended_events.send_default();
                }
            }
            _ => {}
        }
    }
}
