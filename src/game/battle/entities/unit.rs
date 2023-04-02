use bevy::prelude::*;
use bevy_spine::{Spine, SpineBundle, SpineEvent, SpineReadyEvent, SpineSet};
use rand::prelude::*;
use strum_macros::EnumIter;

use crate::{
    AddFixedEvent, AssetLibrary, CollisionShape, DamageKind, DamageReceiveEvent, DamageSystem,
    DefenseKind, Depth, DepthLayer, EventSet, FramesToLive, Health, HealthDieEvent, HitBox,
    HurtBox, SpawnSet, Team, Transform2, UpdateSet, YOrder,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum UnitSystem {
    Spawn,
    Slow,
    Update,
    Attack,
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
                    .in_set(SpawnSet)
                    .after(EventSet::<UnitSpawnEvent>::Sender),
            )
            .add_system(unit_spine_ready.in_set(SpineSet::OnReady))
            .add_system(
                unit_slow
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(UnitSystem::Slow)
                    .in_set(UpdateSet)
                    .before(UnitSystem::Update),
            )
            .add_system(
                unit_update
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(UnitSystem::Update)
                    .in_set(UpdateSet)
                    .after(EventSet::<DamageReceiveEvent>::Sender),
            )
            .add_system(
                unit_attack
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(UnitSystem::Attack)
                    .in_set(UpdateSet)
                    .before(DamageSystem::Update),
            )
            .add_system(
                unit_die
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(UnitSystem::Die)
                    .in_set(UpdateSet)
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
                speed: 400.,
                speed_slow: 300.,
                health: 20.,
                damage: 1.,
                damage_kind: DamageKind::Flesh,
                defense_kind: DefenseKind::Flesh,
                hitbox_offset: 100.,
                hitbox_size: Vec2::new(100., 300.),
            },
            UnitKind::Warrior => UnitStats {
                cost: 5,
                speed: 200.,
                speed_slow: 30.,
                health: 40.,
                damage: 5.,
                damage_kind: DamageKind::Sword,
                defense_kind: DefenseKind::Armor,
                hitbox_offset: 150.,
                hitbox_size: Vec2::new(200., 300.),
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
    pub damage_kind: DamageKind,
    pub defense_kind: DefenseKind,
    pub hitbox_offset: f32,
    pub hitbox_size: Vec2,
}

#[derive(Component)]
pub struct Unit {
    pub team: Team,
    pub stats: UnitStats,
    pub slow_timer: f32,
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
    const UNIT_SCALE: f32 = 0.7;
    for spawn_event in spawn_events.iter() {
        let team = spawn_event.team;
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
                    UNIT_SCALE
                } else {
                    -UNIT_SCALE
                },
                UNIT_SCALE,
            )),
            Depth::from(DepthLayer::YOrder(0.)),
            Health::new(stats.health),
            HitBox {
                flags: team.hit_flags(),
                shape: CollisionShape::Rect(Vec2::new(100., 300.)),
                defense_kind: stats.defense_kind,
            },
            YOrder,
            Unit {
                team,
                stats,
                slow_timer: 0.,
            },
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
            if let Ok(mut track) = spine
                .animation_state
                .set_animation_by_name(1, "attack", true)
            {
                track.set_track_time(rng.gen_range(0.0..1.0));
            }
        }
    }
}

fn unit_slow(
    mut unit_query: Query<&mut Unit>,
    mut damage_receive_events: EventReader<DamageReceiveEvent>,
    time: Res<FixedTime>,
) {
    for mut unit in unit_query.iter_mut() {
        unit.slow_timer -= time.period.as_secs_f32();
        if unit.slow_timer < 0. {
            unit.slow_timer = 0.;
        }
    }
    for damage_receive_event in damage_receive_events.iter() {
        if let Ok(mut unit) = unit_query.get_mut(damage_receive_event.entity) {
            unit.slow_timer = 0.5;
        }
    }
}

fn unit_update(mut unit_query: Query<(&mut Transform2, &Unit)>, time: Res<FixedTime>) {
    for (mut unit_transform, unit) in unit_query.iter_mut() {
        let speed = if unit.slow_timer > 0. {
            unit.stats.speed_slow
        } else {
            unit.stats.speed
        };
        unit_transform.translation.x +=
            time.period.as_secs_f32() * speed * unit_transform.scale.x.signum();
    }
}

fn unit_attack(
    mut commands: Commands,
    mut spine_events: EventReader<SpineEvent>,
    unit_query: Query<(&Unit, &GlobalTransform)>,
) {
    for spine_event in spine_events.iter() {
        if let SpineEvent::Event {
            entity: spine_event_entity,
            name: spine_event_name,
            ..
        } = spine_event
        {
            if spine_event_name == "attack" {
                if let Ok((unit, unit_transform)) = unit_query.get(*spine_event_entity) {
                    commands.spawn((
                        HurtBox {
                            flags: unit.team.hurt_flags(),
                            shape: CollisionShape::Rect(unit.stats.hitbox_size),
                            damage: unit.stats.damage,
                            damage_kind: unit.stats.damage_kind,
                            max_hits: 1,
                        },
                        TransformBundle::default(),
                        Transform2::from_translation(
                            unit_transform.translation().truncate()
                                + Vec2::new(
                                    unit.stats.hitbox_offset * unit.team.move_direction(),
                                    0.,
                                ),
                        ),
                        FramesToLive::new(2),
                    ));
                }
            }
        }
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
