use bevy::prelude::*;
use bevy_spine::{SkeletonData, Spine, SpineBundle, SpineEvent, SpineReadyEvent, SpineSet};
use rand::prelude::*;
use strum_macros::EnumIter;

use crate::{
    AddFixedEvent, AreaOfEffectTargeting, AssetLibrary, CollisionShape, DamageKind,
    DamageReceiveEvent, DamageSystem, DefenseKind, Depth, DepthLayer, EventSet, FramesToLive,
    Health, HealthDieEvent, HitBox, HurtBox, HurtBoxDespawner, Projectile, SpawnSet, SpineAttack,
    SpineFx, Target, Team, Transform2, UpdateSet, YOrder, DEPTH_BLOOD_FX, DEPTH_PROJECTILE,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum UnitSystem {
    Spawn,
    Slow,
    DamageFx,
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
                    .before(UnitSystem::Update)
                    .before(EventSet::<DamageReceiveEvent>::Sender),
            )
            .add_system(
                unit_damage_fx
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(UnitSystem::DamageFx)
                    .in_set(UpdateSet)
                    .before(EventSet::<DamageReceiveEvent>::Sender),
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
    Archer,
    Mage,
    Brute,
}

impl UnitKind {
    pub fn stats(&self) -> UnitStats {
        match self {
            UnitKind::Peasant => UnitStats {
                cost: 1,
                speed: 300.,
                speed_slow: 100.,
                health: 10.,
                attack: Attack::Claw,
                defense_kind: DefenseKind::Flesh,
                spawn_distance_min: 0.,
                spawn_distance_max: 100.,
            },
            UnitKind::Warrior => UnitStats {
                cost: 5,
                speed: 200.,
                speed_slow: 30.,
                health: 40.,
                attack: Attack::Sword,
                defense_kind: DefenseKind::Armor,
                spawn_distance_min: 350.,
                spawn_distance_max: 450.,
            },
            UnitKind::Archer => UnitStats {
                cost: 3,
                speed: 10.,
                speed_slow: 10.,
                health: 20.,
                attack: Attack::Arrow,
                defense_kind: DefenseKind::Flesh,
                spawn_distance_min: 400.,
                spawn_distance_max: 500.,
            },
            UnitKind::Mage => UnitStats {
                cost: 10,
                speed: 10.,
                speed_slow: 100.,
                health: 30.,
                attack: Attack::Magic,
                defense_kind: DefenseKind::Flesh,
                spawn_distance_min: 600.,
                spawn_distance_max: 800.,
            },
            UnitKind::Brute => UnitStats {
                cost: 15,
                speed: 50.,
                speed_slow: 30.,
                health: 200.,
                attack: Attack::Axe,
                defense_kind: DefenseKind::Armor,
                spawn_distance_min: 150.,
                spawn_distance_max: 250.,
            },
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            UnitKind::Peasant => "Peasant",
            UnitKind::Warrior => "Warrior",
            UnitKind::Archer => "Archer",
            UnitKind::Mage => "Mage",
            UnitKind::Brute => "Brute",
        }
    }

    pub fn name_plural(&self) -> &'static str {
        match self {
            UnitKind::Peasant => "Peasants",
            UnitKind::Warrior => "Warriors",
            UnitKind::Archer => "Archers",
            UnitKind::Mage => "Mages",
            UnitKind::Brute => "Brutes",
        }
    }

    pub fn skeleton(&self, asset_library: &AssetLibrary) -> Handle<SkeletonData> {
        match self {
            UnitKind::Peasant => asset_library.spine_rat.clone(),
            UnitKind::Warrior => asset_library.spine_rat_warrior.clone(),
            UnitKind::Archer => asset_library.spine_rat_archer.clone(),
            UnitKind::Mage => asset_library.spine_rat_mage.clone(),
            UnitKind::Brute => asset_library.spine_rat_brute.clone(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct UnitStats {
    pub cost: usize,
    pub speed: f32,
    pub speed_slow: f32,
    pub health: f32,
    pub attack: Attack,
    pub defense_kind: DefenseKind,
    pub spawn_distance_min: f32,
    pub spawn_distance_max: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Attack {
    Claw,
    Sword,
    Arrow,
    Magic,
    Axe,
}

impl Attack {
    pub fn stats(&self) -> AttackStats {
        match self {
            Attack::Claw => AttackStats {
                damage: 0.5,
                damage_kind: DamageKind::Flesh,
                hit_count: 1,
                hurt_box_kind: AttackHurtBoxKind::OffsetRect {
                    offset: 100.,
                    size: Vec2::new(200., 300.),
                },
            },
            Attack::Sword => AttackStats {
                damage: 5.,
                damage_kind: DamageKind::Sword,
                hit_count: 1,
                hurt_box_kind: AttackHurtBoxKind::OffsetRect {
                    offset: 150.,
                    size: Vec2::new(200., 150.),
                },
            },
            Attack::Arrow => AttackStats {
                damage: 2.,
                damage_kind: DamageKind::Arrow,
                hit_count: 1,
                hurt_box_kind: AttackHurtBoxKind::Projectile,
            },
            Attack::Magic => AttackStats {
                damage: 2.,
                damage_kind: DamageKind::Magic,
                hit_count: 20,
                hurt_box_kind: AttackHurtBoxKind::AreaOfEffect {
                    size: Vec2::new(400., 400.),
                },
            },
            Attack::Axe => AttackStats {
                damage: 15.,
                damage_kind: DamageKind::Sword,
                hit_count: 5,
                hurt_box_kind: AttackHurtBoxKind::OffsetRect {
                    offset: 180.,
                    size: Vec2::new(300., 500.),
                },
            },
        }
    }
}

#[derive(Clone, Copy)]
pub struct AttackStats {
    pub damage: f32,
    pub damage_kind: DamageKind,
    pub hit_count: usize,
    pub hurt_box_kind: AttackHurtBoxKind,
}

#[derive(Clone, Copy)]
pub enum AttackHurtBoxKind {
    OffsetRect { offset: f32, size: Vec2 },
    AreaOfEffect { size: Vec2 },
    Projectile,
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
                skeleton: spawn_event.kind.skeleton(asset_library.as_ref()),
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
            Target { team },
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

fn unit_damage_fx(
    mut damage_receive_events: EventReader<DamageReceiveEvent>,
    mut commands: Commands,
    unit_query: Query<&GlobalTransform, With<Unit>>,
    asset_library: Res<AssetLibrary>,
) {
    let mut rng = thread_rng();
    for damage_receive_event in damage_receive_events.iter() {
        if let Ok(unit_transform) = unit_query.get(damage_receive_event.entity) {
            commands.spawn((
                SpineBundle {
                    skeleton: asset_library.spine_fx_blood_splat.clone(),
                    ..Default::default()
                },
                Transform2::from_translation(
                    unit_transform.translation().truncate()
                        + Vec2::new(rng.gen_range(-20.0..20.0), rng.gen_range(-70.0..70.0)),
                ),
                Depth::from(DEPTH_BLOOD_FX),
                SpineFx,
            ));
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
    asset_library: Res<AssetLibrary>,
    area_of_effect_targeting: Res<AreaOfEffectTargeting>,
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
                    let attack_stats = unit.stats.attack.stats();
                    match attack_stats.hurt_box_kind {
                        AttackHurtBoxKind::OffsetRect {
                            offset: hurt_box_offset,
                            size: hurt_box_size,
                        } => {
                            commands.spawn((
                                HurtBox {
                                    flags: unit.team.hurt_flags(),
                                    shape: CollisionShape::Rect(hurt_box_size),
                                    damage: attack_stats.damage,
                                    damage_kind: attack_stats.damage_kind,
                                    max_hits: attack_stats.hit_count,
                                },
                                TransformBundle::default(),
                                Transform2::from_translation(
                                    unit_transform.translation().truncate()
                                        + Vec2::new(
                                            hurt_box_offset * unit.team.move_direction(),
                                            0.,
                                        ),
                                ),
                                FramesToLive::new(2),
                            ));
                        }
                        AttackHurtBoxKind::AreaOfEffect {
                            size: hurt_box_size,
                        } => {
                            let mut rng = thread_rng();
                            if let Some(target_position) =
                                area_of_effect_targeting.get_target(unit.team.opposite_team())
                            {
                                commands.spawn((
                                    SpineBundle {
                                        skeleton: asset_library.spine_attack_magic.clone(),
                                        ..Default::default()
                                    },
                                    SpineAttack {
                                        hurt_box: HurtBox {
                                            flags: unit.team.hurt_flags(),
                                            shape: CollisionShape::Rect(hurt_box_size),
                                            damage: attack_stats.damage,
                                            damage_kind: attack_stats.damage_kind,
                                            max_hits: attack_stats.hit_count,
                                        },
                                    },
                                    SpineFx,
                                    Transform2::from_translation(
                                        target_position
                                            + Vec2::new(rng.gen_range(-200.0..200.0), 0.),
                                    ),
                                ));
                            }
                        }
                        AttackHurtBoxKind::Projectile => {
                            commands.spawn((
                                HurtBox {
                                    flags: unit.team.hurt_flags(),
                                    shape: CollisionShape::Rect(Vec2::new(60., 10.)),
                                    damage: attack_stats.damage,
                                    damage_kind: attack_stats.damage_kind,
                                    max_hits: attack_stats.hit_count,
                                },
                                HurtBoxDespawner,
                                SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(60., 10.)),
                                        color: Color::BLACK,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                Transform2::from_translation(
                                    unit_transform.translation().truncate(),
                                ),
                                Projectile {
                                    velocity: Vec2::new(unit.team.move_direction() * 2000., 300.),
                                },
                                FramesToLive::new(40),
                                Depth::from(DEPTH_PROJECTILE),
                            ));
                        }
                    }
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
