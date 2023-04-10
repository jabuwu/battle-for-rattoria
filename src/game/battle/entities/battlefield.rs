use bevy::prelude::*;

use crate::{AddFixedEvent, AssetLibrary, Depth, EventSet, SpawnSet, Transform2};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum BattlefieldSystem {
    Spawn,
}

pub struct BattlefieldPlugin;

impl Plugin for BattlefieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_event::<BattlefieldSpawnEvent>().add_system(
            battlefield_spawn
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(BattlefieldSystem::Spawn)
                .in_set(SpawnSet)
                .after(EventSet::<BattlefieldSpawnEvent>::Sender),
        );
    }
}

#[derive(Component)]
pub struct Battlefield;

#[derive(Default)]
pub struct BattlefieldSpawnEvent;

fn battlefield_spawn(
    mut commands: Commands,
    mut spawn_events: EventReader<BattlefieldSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    for _ in spawn_events.iter() {
        commands.spawn((
            SpriteBundle {
                texture: asset_library.image_background_bg.clone(),
                ..Default::default()
            },
            Transform2::default(),
            Depth::Exact(0.),
            Battlefield,
        ));
    }
}
