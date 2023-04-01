use bevy::prelude::*;
use bevy_spine::{Spine, SpineBundle, SpineReadyEvent, SpineSet};
use rand::prelude::*;

use crate::{AddFixedEvent, AssetLibrary, Depth, DepthLayer, EventSet, Transform2, YOrder};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum UnitSystem {
    Spawn,
    Update,
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_event::<UnitSpawnEvent>()
            .add_system(
                unit_spawn
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(UnitSystem::Spawn)
                    .after(EventSet::<UnitSpawnEvent>::Sender),
            )
            .add_system(unit_spine_ready.in_set(SpineSet::OnReady))
            .add_system(
                unit_update
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(UnitSystem::Update),
            );
    }
}

#[derive(Component)]
pub struct Unit;

#[derive(Default)]
pub struct UnitSpawnEvent {
    pub position: Vec2,
    pub moving_right: bool,
}

fn unit_spawn(
    mut commands: Commands,
    mut spawn_events: EventReader<UnitSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    for spawn_event in spawn_events.iter() {
        commands.spawn((
            SpineBundle {
                skeleton: asset_library.spine_rat.clone(),
                ..Default::default()
            },
            Transform2::from_translation(spawn_event.position).with_scale(Vec2::new(
                if spawn_event.moving_right { 0.4 } else { -0.4 },
                0.4,
            )),
            Depth::from(DepthLayer::YOrder(0.)),
            YOrder,
            Unit,
        ));
    }
}

fn unit_spine_ready(
    mut spine_ready_events: EventReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine, With<Unit>>,
) {
    let mut rng = thread_rng();
    for spine_ready_event in spine_ready_events.iter() {
        if let Ok(mut spine) = spine_query.get_mut(spine_ready_event.entity) {
            if let Ok(mut track) = spine
                .animation_state
                .set_animation_by_name(0, "animation", true)
            {
                track.set_track_time(rng.gen_range(0.0..1.0));
            }
        }
    }
}

fn unit_update(mut unit_query: Query<&mut Transform2, With<Unit>>, time: Res<FixedTime>) {
    for mut unit_transform in unit_query.iter_mut() {
        unit_transform.translation.x +=
            time.period.as_secs_f32() * 200. * unit_transform.scale.x.signum();
    }
}
