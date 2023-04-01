use bevy::prelude::*;

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
            SpriteBundle {
                texture: asset_library.image_rat.clone(),
                transform: Transform::from_xyz(-300., 0., 0.)
                    .with_scale(Vec2::splat(0.4).extend(1.)),
                ..Default::default()
            },
            Unit,
        ));
    }
}

fn unit_update(
    mut unit_query: Query<&mut Transform, With<Unit>>,
    time: Res<FixedTime>,
    t: Res<Time>,
) {
    for mut unit_transform in unit_query.iter_mut() {
        unit_transform.translation.x += time.period.as_secs_f32() * 100.;
        unit_transform.rotation = Quat::from_rotation_z((t.elapsed_seconds() * 19.).sin() * 0.05);
    }
}
