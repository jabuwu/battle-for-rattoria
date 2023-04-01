use bevy::prelude::*;
use bevy_spine::{Spine, SpineBundle, SpineReadyEvent, SpineSet};

use crate::{AddFixedEvent, AssetLibrary, EventSet};

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
pub struct UnitSpawnEvent;

fn unit_spawn(
    mut commands: Commands,
    mut spawn_events: EventReader<UnitSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    for _ in spawn_events.iter() {
        commands.spawn((
            SpineBundle {
                skeleton: asset_library.spine_rat.clone(),
                transform: Transform::from_xyz(-180., 0., 0.)
                    .with_scale(Vec2::splat(0.4).extend(1.)),
                ..Default::default()
            },
            Unit,
        ));
        commands.spawn((
            SpineBundle {
                skeleton: asset_library.spine_rat.clone(),
                transform: Transform::from_xyz(180., 0., 0.)
                    .with_scale(Vec2::new(-0.4, 0.4).extend(1.)),
                ..Default::default()
            },
            Unit,
        ));
    }
}

fn unit_spine_ready(
    mut spine_ready_events: EventReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine, With<Unit>>,
) {
    for spine_ready_event in spine_ready_events.iter() {
        if let Ok(mut spine) = spine_query.get_mut(spine_ready_event.entity) {
            let _ = spine
                .animation_state
                .set_animation_by_name(0, "animation", true);
        }
    }
}

fn unit_update(mut unit_query: Query<&mut Transform, With<Unit>>, time: Res<FixedTime>) {
    for mut unit_transform in unit_query.iter_mut() {
        unit_transform.translation.x +=
            time.period.as_secs_f32() * 100. * unit_transform.scale.x.signum();
    }
}
