use bevy::prelude::*;
use bevy_spine::{Spine, SpineBundle, SpineReadyEvent, SpineSet};
use rand::prelude::*;
use strum_macros::EnumIter;

use crate::{
    AddFixedEvent, AssetLibrary, CollisionShape, DamageReceiveEvent, Depth, DepthLayer, EventSet,
    Health, HealthDieEvent, HitBox, HurtBox, Team, Transform2, YOrder,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum UnitSystem {
    Spawn,
    Update,
    Die,
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
                    .in_set(UnitSystem::Update)
                    .after(EventSet::<DamageReceiveEvent>::Sender),
            )
            .add_system(
                unit_die
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(UnitSystem::Die)
                    .after(EventSet::<HealthDieEvent>::Sender),
            );
    }
}

#[derive(Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum UnitKind {
    Peasant,
    Warrior,
}

impl UnitKind {
    pub fn stats(&self) -> UnitStats {
        match self {
            UnitKind::Peasant => UnitStats {
                cost: 1,
                speed: 200.,
                speed_slow: 80.,
                health: 100.,
                damage: 1.,
            },
            UnitKind::Warrior => UnitStats {
                cost: 3,
                speed: 100.,
                speed_slow: 60.,
                health: 150.,
                damage: 3.,
            },
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            UnitKind::Peasant => "Peasant",
            UnitKind::Warrior => "Warrior",
        }
    }
}

#[derive(Clone, Copy)]
pub struct UnitStats {
    pub cost: usize,
    pub speed: f32,
    pub speed_slow: f32,
    pub health: f32,
    pub damage: f32,
}

#[derive(Component)]
pub struct Unit {
    pub stats: UnitStats,
}

pub struct UnitSpawnEvent {
    pub kind: UnitKind,
    pub position: Vec2,
    pub team: Team,
}

fn unit_spawn(
    mut commands: Commands,
    mut spawn_events: EventReader<UnitSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    for spawn_event in spawn_events.iter() {
        let stats = spawn_event.kind.stats();
        commands.spawn((
            SpineBundle {
                skeleton: if spawn_event.kind == UnitKind::Peasant {
                    asset_library.spine_rat.clone()
                } else {
                    asset_library.spine_rat_warrior.clone()
                },
                ..Default::default()
            },
            Transform2::from_translation(spawn_event.position).with_scale(Vec2::new(
                if spawn_event.team == Team::Friendly {
                    0.3
                } else {
                    -0.3
                },
                0.3,
            )),
            Depth::from(DepthLayer::YOrder(0.)),
            Health::new(stats.health),
            HitBox {
                flags: spawn_event.team.hit_flags(),
                shape: CollisionShape::Rect(Vec2::splat(100.)),
            },
            HurtBox {
                flags: spawn_event.team.hurt_flags(),
                shape: CollisionShape::Rect(Vec2::splat(100.)),
                damage: stats.damage,
            },
            YOrder,
            Unit { stats },
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

fn unit_update(
    mut unit_query: Query<(Entity, &mut Transform2, &Unit)>,
    mut damage_receive_events: EventReader<DamageReceiveEvent>,
    time: Res<FixedTime>,
) {
    let mut slow_entities = vec![];
    for damage_receive_event in damage_receive_events.iter() {
        if unit_query.contains(damage_receive_event.entity) {
            slow_entities.push(damage_receive_event.entity);
        }
    }
    for (unit_entity, mut unit_transform, unit) in unit_query.iter_mut() {
        let speed = if slow_entities.contains(&unit_entity) {
            unit.stats.speed_slow
        } else {
            unit.stats.speed
        };
        unit_transform.translation.x +=
            time.period.as_secs_f32() * speed * unit_transform.scale.x.signum();
    }
}

fn unit_die(
    mut health_die_events: EventReader<HealthDieEvent>,
    mut commands: Commands,
    unit_query: Query<&Unit>,
) {
    for health_die_event in health_die_events.iter() {
        if unit_query.contains(health_die_event.entity) {
            if let Some(entity) = commands.get_entity(health_die_event.entity) {
                entity.despawn_recursive();
            }
        }
    }
}