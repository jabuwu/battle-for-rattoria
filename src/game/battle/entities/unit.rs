use bevy::prelude::*;
use bevy_spine::prelude::*;
use bitflags::bitflags;
use enum_map::{Enum, EnumMap};
use rand::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    AddFixedEvent, AreaOfEffectTargeting, AssetLibrary, BattleModifier, BattlePhase, BattleState,
    CollisionShape, DamageInflictEvent, DamageKind, DamageModifier, DamageModifiers,
    DamageReceiveEvent, DamageSystem, DefenseKind, DefenseModifier, DefenseModifiers, Depth,
    DepthLayer, EventSet, Feeler, FramesToLive, Health, HealthDieEvent, HitBox, HurtBox,
    HurtBoxDespawner, Projectile, SpawnSet, SpineAttack, SpineFx, Target, Team, Transform2,
    UpdateSet, YOrder, DEPTH_BLOOD_FX, DEPTH_PROJECTILE,
};

const UNIT_SCALE: f32 = 0.7;
const UNIT_TRACK_WALK: usize = 0;
const UNIT_TRACK_ATTACK: usize = 1;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum UnitSystem {
    Spawn,
    Slow,
    DamageFx,
    Update,
    Attack,
    Die,
    Cowardly,
    UpdateSpriteDirection,
    UpdateAnimations,
    UpdateFeeler,
    Combust,
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
            )
            .add_system(
                unit_cowardly
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(UnitSystem::Cowardly)
                    .in_set(UpdateSet)
                    .after(EventSet::<DamageReceiveEvent>::Sender)
                    .before(UnitSystem::Update),
            )
            .add_system(
                unit_update_sprite_direction
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(UnitSystem::UpdateSpriteDirection)
                    .in_set(UpdateSet)
                    .before(UnitSystem::Update),
            )
            .add_system(
                unit_update_animations
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(UnitSystem::UpdateAnimations)
                    .in_set(UpdateSet)
                    .before(UnitSystem::Update),
            )
            .add_system(
                unit_update_feeler
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(UnitSystem::UpdateFeeler)
                    .in_set(UpdateSet)
                    .before(UnitSystem::Update),
            )
            .add_system(
                unit_combust
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(UnitSystem::Combust)
                    .in_set(UpdateSet)
                    .in_set(EventSet::<DamageInflictEvent>::Sender)
                    .before(UnitSystem::Update),
            );
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Enum, EnumIter)]
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
                speed_slow: 150.,
                health: 15.,
                attack: Attack::Claw,
                defense_kind: DefenseKind::Flesh,
                spawn_distance_min: 0.,
                spawn_distance_max: 900.,
                hit_box_size: Vec2::new(100., 400.),
                feeler_size: Vec2::new(200., 400.),
                retreat_chance: 0.01,
                attributes: Attributes::empty(),
            },
            UnitKind::Warrior => UnitStats {
                cost: 5,
                speed: 200.,
                speed_slow: 70.,
                health: 30.,
                attack: Attack::Sword,
                defense_kind: DefenseKind::Armor,
                spawn_distance_min: 0.,
                spawn_distance_max: 500.,
                hit_box_size: Vec2::new(300., 400.),
                feeler_size: Vec2::new(400., 400.),
                retreat_chance: 0.1,
                attributes: Attributes::empty(),
            },
            UnitKind::Archer => UnitStats {
                cost: 3,
                speed: 10.,
                speed_slow: 10.,
                health: 5.,
                attack: Attack::Arrow,
                defense_kind: DefenseKind::Flesh,
                spawn_distance_min: 400.,
                spawn_distance_max: 600.,
                hit_box_size: Vec2::new(100., 400.),
                feeler_size: Vec2::new(2200., 400.),
                retreat_chance: 0.33,
                attributes: Attributes::MAY_RETREAT,
            },
            UnitKind::Mage => UnitStats {
                cost: 10,
                speed: 0.,
                speed_slow: 0.,
                health: 10.,
                attack: Attack::Magic,
                defense_kind: DefenseKind::Flesh,
                spawn_distance_min: 600.,
                spawn_distance_max: 800.,
                hit_box_size: Vec2::new(100., 400.),
                feeler_size: Vec2::new(1400., 400.),
                retreat_chance: 1.,
                attributes: Attributes::MAY_RETREAT,
            },
            UnitKind::Brute => UnitStats {
                cost: 15,
                speed: 80.,
                speed_slow: 50.,
                health: 300.,
                attack: Attack::Axe,
                defense_kind: DefenseKind::Armor,
                spawn_distance_min: 150.,
                spawn_distance_max: 250.,
                hit_box_size: Vec2::new(300., 500.),
                feeler_size: Vec2::new(400., 400.),
                retreat_chance: 0.01,
                attributes: Attributes::MAY_FRIENDLY_FIRE,
            },
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            UnitKind::Peasant => "Mobling",
            UnitKind::Warrior => "Stabby-Rat",
            UnitKind::Archer => "Shooty-Rat",
            UnitKind::Mage => "Blasty-Rat",
            UnitKind::Brute => "Bigg-Rat",
        }
    }

    pub fn name_plural(&self) -> &'static str {
        match self {
            UnitKind::Peasant => "Moblings",
            UnitKind::Warrior => "Stabby-Rats",
            UnitKind::Archer => "Shooty-Rats",
            UnitKind::Mage => "Blasty-Rats",
            UnitKind::Brute => "Bigg-Rats",
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
    pub hit_box_size: Vec2,
    pub feeler_size: Vec2,
    pub retreat_chance: f32,
    pub attributes: Attributes,
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
                hit_count: 3,
                hurt_box_kind: AttackHurtBoxKind::OffsetRect {
                    offset: 100.,
                    size: Vec2::new(200., 300.),
                },
            },
            Attack::Sword => AttackStats {
                damage: 5.,
                damage_kind: DamageKind::Sword,
                hit_count: 5,
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
                damage: 1.,
                damage_kind: DamageKind::Magic,
                hit_count: 20,
                hurt_box_kind: AttackHurtBoxKind::AreaOfEffect {
                    size: Vec2::new(400., 400.),
                },
            },
            Attack::Axe => AttackStats {
                damage: 15.,
                damage_kind: DamageKind::Sword,
                hit_count: 3,
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
    pub kind: UnitKind,
    pub stats: UnitStats,
    pub slow_timer: f32,
    pub retreating: bool,
    pub blind: bool,
    pub attributes: Attributes,
}

#[derive(Component)]
pub struct UnitFire;

impl Unit {
    pub fn can_attack(&self) -> bool {
        !self.retreating
    }

    pub fn move_direction(&self) -> f32 {
        let mut move_dir = self.team.move_direction();
        if self.retreating {
            move_dir *= -1.;
        }
        move_dir
    }

    pub fn speed(&self) -> f32 {
        if self.retreating {
            300.
        } else if self.slow_timer > 0. {
            self.stats.speed_slow
        } else {
            self.stats.speed
        }
    }
}

bitflags! {
    pub struct Attributes: u32 {
        const MAY_RETREAT = 0b00000001;
        const MAY_FRIENDLY_FIRE = 0b00000010;
        const ON_FIRE = 0b00000100;
    }
}

pub struct UnitSpawnEvent {
    pub kind: UnitKind,
    pub position: Vec2,
    pub team: Team,
}

fn unit_spawn(
    mut commands: Commands,
    mut spawn_events: EventReader<UnitSpawnEvent>,
    battle_state: Res<BattleState>,
    asset_library: Res<AssetLibrary>,
) {
    let mut rng = thread_rng();
    for spawn_event in spawn_events.iter() {
        let team = spawn_event.team;
        let team_modifiers = battle_state.get_modifiers(team);
        let mut defense_modifiers = DefenseModifiers::default();
        if team_modifiers[BattleModifier::Fire] {
            defense_modifiers[DefenseModifier::Fire] = true;
        }
        if team_modifiers[BattleModifier::Ice] {
            defense_modifiers[DefenseModifier::Ice] = true;
        }
        if team_modifiers[BattleModifier::Wet] {
            defense_modifiers[DefenseModifier::Wet] = true;
        }
        let mut stats = spawn_event.kind.stats();
        if team_modifiers[BattleModifier::ExtraSpeed] {
            stats.speed *= 3.;
            stats.speed_slow *= 1.5;
        }
        if team_modifiers[BattleModifier::Slowness] {
            stats.speed *= 0.5;
            stats.speed_slow *= 0.5;
        }
        commands
            .spawn((
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
                    shape: CollisionShape::Rect {
                        offset: Vec2::ZERO,
                        size: stats.hit_box_size,
                    },
                    defense: if team_modifiers[BattleModifier::ExtraDefense] {
                        3.
                    } else {
                        1.
                    },
                    defense_kind: stats.defense_kind,
                    defense_modifiers,
                },
                YOrder,
                Target { team },
                Feeler {
                    shape: CollisionShape::None,
                    flags: team.hurt_flags(),
                    ..Default::default()
                },
                Unit {
                    team,
                    kind: spawn_event.kind,
                    stats,
                    slow_timer: 0.,
                    retreating: false,
                    blind: team_modifiers[BattleModifier::Blindness] && rng.gen_bool(0.5),
                    attributes: stats.attributes,
                },
            ))
            .with_children(|parent| {
                if team_modifiers[BattleModifier::Fire] {
                    parent.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::RED,
                                custom_size: Some(Vec2::splat(20.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Transform2::from_xy(0., 0.),
                        Depth::Inherit(0.01),
                    ));
                }
                if team_modifiers[BattleModifier::Ice] {
                    parent.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::TURQUOISE,
                                custom_size: Some(Vec2::splat(20.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Transform2::from_xy(0., 40.),
                        Depth::Inherit(0.01),
                    ));
                }
                if team_modifiers[BattleModifier::Wet] {
                    parent.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::BLUE,
                                custom_size: Some(Vec2::splat(20.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Transform2::from_xy(0., -40.),
                        Depth::Inherit(0.01),
                    ));
                }
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(1., 0., 0., 0.4),
                            custom_size: Some(Vec2::new(200., 400.)),
                            ..Default::default()
                        },
                        visibility: Visibility::Hidden,
                        ..Default::default()
                    },
                    Transform2::default(),
                    Depth::Inherit(0.01),
                    UnitFire,
                ));
            });
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
            if rng.gen_bool(0.25) {
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
}

fn unit_update(
    mut unit_query: Query<(&mut Transform2, &Unit)>,
    time: Res<FixedTime>,
    battle_state: Res<BattleState>,
) {
    if battle_state.phase() != BattlePhase::PreBattle {
        for (mut unit_transform, unit) in unit_query.iter_mut() {
            unit_transform.translation.x +=
                time.period.as_secs_f32() * unit.speed() * unit_transform.scale.x.signum();
        }
    }
}

fn unit_attack(
    mut commands: Commands,
    mut spine_events: EventReader<SpineEvent>,
    battle_state: Res<BattleState>,
    unit_query: Query<(Entity, &Unit, &GlobalTransform)>,
    asset_library: Res<AssetLibrary>,
    area_of_effect_targeting: Res<AreaOfEffectTargeting>,
) {
    let mut rng = thread_rng();
    for spine_event in spine_events.iter() {
        if let SpineEvent::Event {
            entity: spine_event_entity,
            name: spine_event_name,
            ..
        } = spine_event
        {
            if spine_event_name == "attack" {
                if let Ok((unit_entity, unit, unit_transform)) = unit_query.get(*spine_event_entity)
                {
                    let team_modifiers = battle_state.get_modifiers(unit.team);
                    let damage_multiplier = if team_modifiers[BattleModifier::ExtraAttack] {
                        1.5
                    } else {
                        1.
                    };
                    let mut damage_modifiers = DamageModifiers::default();
                    if team_modifiers[BattleModifier::Fire] {
                        damage_modifiers[DamageModifier::Fire] = true;
                    }
                    if team_modifiers[BattleModifier::Ice] {
                        damage_modifiers[DamageModifier::Ice] = true;
                    }
                    if team_modifiers[BattleModifier::Wet] {
                        damage_modifiers[DamageModifier::Wet] = true;
                    }
                    let friendly_fire = if (team_modifiers[BattleModifier::FriendlyFire]
                        || unit.attributes.contains(Attributes::MAY_FRIENDLY_FIRE))
                        && rng.gen_bool(0.25)
                    {
                        true
                    } else {
                        false
                    };
                    let attack_stats = unit.stats.attack.stats();
                    let mut hurt_flags = unit.team.hurt_flags();
                    if friendly_fire {
                        hurt_flags |= unit.team.hit_flags();
                    }
                    match attack_stats.hurt_box_kind {
                        AttackHurtBoxKind::OffsetRect {
                            offset: hurt_box_offset,
                            size: hurt_box_size,
                        } => {
                            commands.spawn((
                                HurtBox {
                                    flags: hurt_flags,
                                    shape: CollisionShape::Rect {
                                        offset: Vec2::ZERO,
                                        size: hurt_box_size,
                                    },
                                    damage: attack_stats.damage * damage_multiplier,
                                    damage_kind: attack_stats.damage_kind,
                                    damage_modifiers,
                                    max_hits: attack_stats.hit_count,
                                    ignore_entity: unit_entity,
                                },
                                TransformBundle::default(),
                                Transform2::from_translation(
                                    unit_transform.translation().truncate()
                                        + Vec2::new(hurt_box_offset * unit.move_direction(), 0.),
                                ),
                                FramesToLive::new(2),
                            ));
                        }
                        AttackHurtBoxKind::AreaOfEffect {
                            size: hurt_box_size,
                        } => {
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
                                            flags: hurt_flags,
                                            shape: CollisionShape::Rect {
                                                offset: Vec2::ZERO,
                                                size: hurt_box_size,
                                            },
                                            damage: attack_stats.damage * damage_multiplier,
                                            damage_kind: attack_stats.damage_kind,
                                            damage_modifiers,
                                            max_hits: attack_stats.hit_count,
                                            ignore_entity: unit_entity,
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
                                    flags: hurt_flags,
                                    shape: CollisionShape::Rect {
                                        offset: Vec2::ZERO,
                                        size: Vec2::new(60., 10.),
                                    },
                                    damage: attack_stats.damage * damage_multiplier,
                                    damage_kind: attack_stats.damage_kind,
                                    damage_modifiers,
                                    max_hits: attack_stats.hit_count,
                                    ignore_entity: unit_entity,
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
                                    velocity: Vec2::new(unit.move_direction() * 2500., 300.),
                                },
                                FramesToLive::new(100),
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

fn unit_cowardly(
    mut unit_query: Query<&mut Unit>,
    mut damage_receive_events: EventReader<DamageReceiveEvent>,
    battle_state: Res<BattleState>,
) {
    let mut rng = thread_rng();
    for damage_receive_event in damage_receive_events.iter() {
        if let Ok(mut unit) = unit_query.get_mut(damage_receive_event.entity) {
            if rng.gen_bool(unit.stats.retreat_chance as f64)
                && (unit.attributes.contains(Attributes::MAY_RETREAT)
                    || battle_state.get_modifiers(unit.team)[BattleModifier::Cowardly])
            {
                unit.retreating = true;
            }
        }
    }
}

fn unit_update_sprite_direction(mut unit_query: Query<(&mut Transform2, &Unit)>) {
    for (mut unit_transform, unit) in unit_query.iter_mut() {
        unit_transform.scale.x = UNIT_SCALE * unit.move_direction();
    }
}

fn unit_update_animations(
    mut unit_query: Query<(&mut Spine, &Unit, &Feeler)>,
    battle_state: Res<BattleState>,
) {
    let mut rng = thread_rng();
    for (mut unit_spine, unit, unit_feeler) in unit_query.iter_mut() {
        if battle_state.phase() != BattlePhase::PreBattle {
            if unit_spine
                .animation_state
                .track_at_index(UNIT_TRACK_WALK)
                .is_none()
            {
                if let Ok(mut track) = unit_spine.animation_state.set_animation_by_name(
                    UNIT_TRACK_WALK as i32,
                    "animation",
                    true,
                ) {
                    track.set_timescale(rng.gen_range(0.9..1.1));
                }
            }
        } else {
            if unit_spine
                .animation_state
                .track_at_index(UNIT_TRACK_WALK)
                .is_some()
            {
                unit_spine
                    .animation_state
                    .clear_track(UNIT_TRACK_WALK as i32);
            }
        }
        if (unit.can_attack()
            && unit_feeler.feeling
            && battle_state.phase() == BattlePhase::Battling)
            || unit.blind
        {
            if unit_spine
                .animation_state
                .track_at_index(UNIT_TRACK_ATTACK)
                .is_none()
            {
                if let Ok(mut track) = unit_spine.animation_state.set_animation_by_name(
                    UNIT_TRACK_ATTACK as i32,
                    "attack",
                    true,
                ) {
                    let slowness = battle_state.get_modifiers(unit.team)[BattleModifier::Slowness];
                    let quick_attack =
                        battle_state.get_modifiers(unit.team)[BattleModifier::QuickAttack];
                    if slowness && !quick_attack {
                        track.set_timescale(0.5);
                    } else if !slowness && quick_attack {
                        track.set_timescale(2.);
                    } else {
                        track.set_timescale(1.);
                    }
                }
            }
        } else {
            if unit_spine
                .animation_state
                .track_at_index(UNIT_TRACK_ATTACK)
                .is_some()
            {
                unit_spine
                    .animation_state
                    .clear_track(UNIT_TRACK_ATTACK as i32);
            }
        }
    }
}

fn unit_update_feeler(mut unit_query: Query<(&mut Feeler, &Unit)>) {
    for (mut unit_feeler, unit) in unit_query.iter_mut() {
        unit_feeler.shape = CollisionShape::Rect {
            offset: Vec2::new(unit.stats.feeler_size.x * 0.5 * unit.move_direction(), 0.),
            size: unit.stats.feeler_size,
        };
    }
}

#[derive(Default)]
struct UnitCombustion {
    time_since_last_combustion: EnumMap<Team, f32>,
    time_until_next_combustion: EnumMap<Team, f32>,
}

fn unit_combust(
    mut local: Local<UnitCombustion>,
    mut unit_query: Query<(Entity, &mut Unit, &Children)>,
    mut unit_fire_query: Query<&mut Visibility, With<UnitFire>>,
    mut damage_inflict_events: EventWriter<DamageInflictEvent>,
    battle_state: Res<BattleState>,
    time: Res<FixedTime>,
) {
    let mut rng = thread_rng();
    for (unit_entity, unit, _) in unit_query.iter_mut() {
        if unit.attributes.contains(Attributes::ON_FIRE) {
            damage_inflict_events.send(DamageInflictEvent {
                entity: unit_entity,
                damage: time.period.as_secs_f32() * 5.,
            });
        }
    }
    for team in Team::iter() {
        if battle_state.battling() && battle_state.phase() == BattlePhase::Battling {
            if local.time_until_next_combustion[team] == 0. {
                local.time_until_next_combustion[team] = rng.gen_range(0.5..1.);
            }
            if battle_state.get_modifiers(team)[BattleModifier::Combustion] {
                let mut combust = false;
                if local.time_since_last_combustion[team] > local.time_until_next_combustion[team] {
                    combust = true;
                    local.time_since_last_combustion[team] = 0.;
                    local.time_until_next_combustion[team] = rng.gen_range(0.5..2.0);
                }
                local.time_since_last_combustion[team] += time.period.as_secs_f32();
                if combust {
                    let mut units = unit_query
                        .iter_mut()
                        .filter(|(_, unit, _)| {
                            unit.team == team && !unit.attributes.contains(Attributes::ON_FIRE)
                        })
                        .collect::<Vec<_>>();
                    units.shuffle(&mut rng);
                    if let Some((_, mut unit, unit_children)) = units.into_iter().nth(0) {
                        for child in unit_children.iter() {
                            if let Ok(mut unit_fire_visibility) = unit_fire_query.get_mut(*child) {
                                *unit_fire_visibility = Visibility::Visible;
                            }
                        }
                        unit.attributes |= Attributes::ON_FIRE;
                    }
                }
            }
        } else {
            local.time_until_next_combustion[team] = 0.;
            local.time_since_last_combustion[team] = 0.;
        }
    }
}
