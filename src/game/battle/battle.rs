use std::mem::take;

use bevy::prelude::*;
use enum_map::{Enum, EnumMap};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    AddFixedEvent, BannerSpawnEvent, BattleSplashEndedEvent, BattleSplashKind,
    BattleSplashPlayEvent, BattleSplashSpawnEvent, BattlefieldSpawnEvent, DamageReceiveEvent,
    EventSet, HealthDieEvent, Sfx, SfxKind, SpawnSet, Team, Unit, UnitKind, UnitSpawnEvent,
    UpdateSet,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum BattleSystem {
    Start,
    UnitDie,
    EndDetection,
    SplashEnded,
}

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BattleState>()
            .add_fixed_event::<BattleStartEvent>()
            .add_fixed_event::<BattleEndedEvent>()
            .add_system(
                battle_start
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(BattleSystem::Start)
                    .in_set(SpawnSet)
                    .in_set(EventSet::<BattlefieldSpawnEvent>::Sender)
                    .in_set(EventSet::<BattleSplashSpawnEvent>::Sender)
                    .in_set(EventSet::<UnitSpawnEvent>::Sender)
                    .in_set(EventSet::<BannerSpawnEvent>::Sender)
                    .after(EventSet::<BattleStartEvent>::Sender),
            )
            .add_system(
                battle_unit_die
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(BattleSystem::UnitDie)
                    .in_set(UpdateSet)
                    .after(EventSet::<HealthDieEvent>::Sender),
            )
            .add_system(
                battle_end_detection
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(BattleSystem::EndDetection)
                    .in_set(UpdateSet)
                    .in_set(EventSet::<BattleSplashPlayEvent>::Sender)
                    .after(EventSet::<DamageReceiveEvent>::Sender),
            )
            .add_system(
                battle_splash_ended
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(BattleSystem::SplashEnded)
                    .in_set(UpdateSet)
                    .in_set(EventSet::<BattleEndedEvent>::Sender)
                    .after(EventSet::<BattleSplashEndedEvent>::Sender),
            );
    }
}

#[derive(Resource)]
pub struct BattleState {
    battling: bool,
    phase: BattlePhase,
    report: BattleReport,
    end_timer: f32,
    damage_inflicted: bool,
    time_since_last_damage: f32,
    friendly_modifiers: BattleModifiers,
    enemy_modifiers: BattleModifiers,
}

impl BattleState {
    pub fn phase(&self) -> BattlePhase {
        self.phase
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BattlePhase {
    PreBattle,
    Battling,
    End { victory: bool },
    Results,
}

impl Default for BattleState {
    fn default() -> Self {
        Self {
            battling: false,
            phase: BattlePhase::PreBattle,
            report: BattleReport::default(),
            end_timer: 0.,
            damage_inflicted: false,
            time_since_last_damage: 0.,
            friendly_modifiers: BattleModifiers::default(),
            enemy_modifiers: BattleModifiers::default(),
        }
    }
}

impl BattleState {
    pub fn stop(&mut self) {
        self.battling = false;
    }

    pub fn battling(&self) -> bool {
        self.battling
    }

    pub fn get_modifiers(&self, team: Team) -> &BattleModifiers {
        match team {
            Team::Friendly => &self.friendly_modifiers,
            Team::Enemy => &self.enemy_modifiers,
        }
    }
}

#[derive(Default)]
pub struct BattleReport {
    pub dead_units: UnitComposition,
    pub victory: bool,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct BattleConfig {
    pub friendly_units: UnitComposition,
    pub friendly_modifiers: BattleModifiers,
    pub friendly_banner: Banner,
    pub enemy_units: UnitComposition,
    pub enemy_modifiers: BattleModifiers,
    pub enemy_banner: Banner,
}

impl BattleConfig {
    pub fn get_units(&self, team: Team) -> &UnitComposition {
        match team {
            Team::Friendly => &self.friendly_units,
            Team::Enemy => &self.enemy_units,
        }
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Banner {
    #[default]
    Player,
    WarChef1,
    WarChef2,
    WarChef3,
    WarChef4,
    WarChef5,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct UnitComposition {
    pub peasants: usize,
    pub warriors: usize,
    pub archers: usize,
    pub mages: usize,
    pub brutes: usize,
}

impl UnitComposition {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn get_count(&self, unit_kind: UnitKind) -> usize {
        match unit_kind {
            UnitKind::Peasant => self.peasants,
            UnitKind::Warrior => self.warriors,
            UnitKind::Archer => self.archers,
            UnitKind::Mage => self.mages,
            UnitKind::Brute => self.brutes,
        }
    }

    pub fn set_count(&mut self, unit_kind: UnitKind, i: usize) {
        match unit_kind {
            UnitKind::Peasant => self.peasants = i,
            UnitKind::Warrior => self.warriors = i,
            UnitKind::Archer => self.archers = i,
            UnitKind::Mage => self.mages = i,
            UnitKind::Brute => self.brutes = i,
        }
    }

    pub fn mutate_count(&mut self, unit_kind: UnitKind, f: impl Fn(usize) -> usize) {
        let new_count = f(self.get_count(unit_kind));
        self.set_count(unit_kind, new_count);
    }

    pub fn add_units(&mut self, other: &UnitComposition) {
        for unit_kind in UnitKind::iter() {
            self.mutate_count(unit_kind, |i| i + other.get_count(unit_kind));
        }
    }

    pub fn subtract_units(&mut self, other: &UnitComposition) {
        for unit_kind in UnitKind::iter() {
            self.mutate_count(unit_kind, |i| {
                if other.get_count(unit_kind) > i {
                    0
                } else {
                    i - other.get_count(unit_kind)
                }
            });
        }
    }

    pub fn total_units(&self) -> usize {
        let mut total = 0;
        for unit_kind in UnitKind::iter() {
            total += self.get_count(unit_kind);
        }
        total
    }
}

pub type BattleModifiers = EnumMap<BattleModifier, bool>;

#[derive(Clone, Copy, PartialEq, Eq, Enum, EnumIter, Serialize, Deserialize)]
pub enum BattleModifier {
    ExtraDefense,
    ExtraAttack,
    QuickAttack,
    ExtraSpeed,
    Fire,
    Ice,
    Wet,
    FriendlyFire,
    Cowardly,
    Sickness,
    Explosive,
    Combustion,
    Blindness,
    Slowness,
}

impl BattleModifier {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ExtraDefense => "Extra Defense",
            Self::ExtraAttack => "Extra Attack",
            Self::QuickAttack => "Quick Attack",
            Self::ExtraSpeed => "Extra Speed",
            Self::Fire => "Fire",
            Self::Ice => "Ice",
            Self::Wet => "Wet",
            Self::FriendlyFire => "Friendly Fire",
            Self::Cowardly => "Cowardly",
            Self::Sickness => "Sickness",
            Self::Explosive => "Explosive",
            Self::Combustion => "Combustion",
            Self::Blindness => "Blindness",
            Self::Slowness => "Slowness",
        }
    }
}

pub struct BattleStartEvent {
    pub config: BattleConfig,
    pub sandbox: bool,
}

#[derive(Default)]
pub struct BattleEndedEvent {
    pub report: BattleReport,
    _private: (),
}

fn battle_start(
    mut start_events: EventReader<BattleStartEvent>,
    mut battle_state: ResMut<BattleState>,
    mut battlefield_spawn_events: EventWriter<BattlefieldSpawnEvent>,
    mut battle_splash_spawn_events: EventWriter<BattleSplashSpawnEvent>,
    mut unit_spawn_events: EventWriter<UnitSpawnEvent>,
    mut banner_spawn_events: EventWriter<BannerSpawnEvent>,
) {
    for start_event in start_events.iter() {
        *battle_state = BattleState::default();
        battle_state.battling = true;
        battle_state.friendly_modifiers = start_event.config.friendly_modifiers;
        battle_state.enemy_modifiers = start_event.config.enemy_modifiers;
        battlefield_spawn_events.send_default();
        battle_splash_spawn_events.send(BattleSplashSpawnEvent {
            play_battle_start: !start_event.sandbox,
        });
        if start_event.sandbox {
            battle_state.phase = BattlePhase::Battling;
        }

        const X_DISTANCE: f32 = 400.;
        const Y_MIN: f32 = -400.;
        const Y_MAX: f32 = -200.;
        let mut rng = thread_rng();
        for team in Team::iter() {
            let units = start_event.config.get_units(team);
            for unit_kind in UnitKind::iter() {
                let unit_stats = unit_kind.stats();
                for _ in 0..units.get_count(unit_kind) {
                    let x = (rng
                        .gen_range(unit_stats.spawn_distance_min..unit_stats.spawn_distance_max)
                        + X_DISTANCE)
                        * -team.move_direction();
                    let y = rng.gen_range(Y_MIN..Y_MAX);
                    unit_spawn_events.send(UnitSpawnEvent {
                        kind: unit_kind,
                        position: Vec2::new(x, y),
                        team,
                    });
                }
            }
        }

        banner_spawn_events.send(BannerSpawnEvent {
            banner: start_event.config.friendly_banner,
            position: Vec2::new(-950., -270.),
        });
        banner_spawn_events.send(BannerSpawnEvent {
            banner: start_event.config.enemy_banner,
            position: Vec2::new(850., -270.),
        });
    }
}

fn battle_unit_die(
    mut health_die_events: EventReader<HealthDieEvent>,
    mut battle_state: ResMut<BattleState>,
    unit_query: Query<&Unit>,
) {
    if !battle_state.battling {
        return;
    }
    for health_die_event in health_die_events.iter() {
        if let Ok(unit) = unit_query.get(health_die_event.entity) {
            if unit.team == Team::Friendly {
                battle_state
                    .report
                    .dead_units
                    .mutate_count(unit.kind, |i| i + 1);
            }
        }
    }
}

fn battle_end_detection(
    mut battle_state: ResMut<BattleState>,
    mut damage_receive_events: EventReader<DamageReceiveEvent>,
    mut battle_splash_play_events: EventWriter<BattleSplashPlayEvent>,
    unit_query: Query<&Unit>,
    time: Res<FixedTime>,
) {
    if battle_state.phase != BattlePhase::Battling {
        return;
    }
    let mut friendly_count = 0;
    let mut enemy_count = 0;
    let mut enemy_has_brute = false;
    for unit in unit_query.iter() {
        if !unit.retreating {
            if unit.team == Team::Friendly {
                friendly_count += 1;
            } else if unit.team == Team::Enemy {
                if unit.kind == UnitKind::Brute {
                    enemy_has_brute = true;
                }
                enemy_count += 1;
            }
        }
    }
    for _ in damage_receive_events.iter() {
        battle_state.damage_inflicted = true;
        battle_state.time_since_last_damage = 0.;
    }
    battle_state.time_since_last_damage += time.period.as_secs_f32();
    if friendly_count == 0 || enemy_count == 0 {
        battle_state.end_timer += time.period.as_secs_f32();
    } else {
        battle_state.end_timer = 0.;
    }
    if battle_state.end_timer > 2.
        || (battle_state.damage_inflicted && battle_state.time_since_last_damage > 4.)
        || (!battle_state.damage_inflicted && battle_state.time_since_last_damage > 8.)
    {
        let victory = !enemy_has_brute && friendly_count > enemy_count;
        battle_state.report.victory = victory;
        battle_state.phase = BattlePhase::End { victory };
        battle_splash_play_events.send(BattleSplashPlayEvent {
            kind: if victory {
                BattleSplashKind::Victory
            } else {
                BattleSplashKind::Defeat
            },
        });
    }
}

pub fn battle_splash_ended(
    mut battle_splash_ended_events: EventReader<BattleSplashEndedEvent>,
    mut battle_state: ResMut<BattleState>,
    mut battle_ended_events: EventWriter<BattleEndedEvent>,
    mut sfx: ResMut<Sfx>,
) {
    for _ in battle_splash_ended_events.iter() {
        if battle_state.phase == BattlePhase::PreBattle {
            sfx.play(SfxKind::Mayhem);
            battle_state.phase = BattlePhase::Battling;
        } else if matches!(battle_state.phase, BattlePhase::End { .. }) {
            battle_ended_events.send(BattleEndedEvent {
                report: take(&mut battle_state.report),
                _private: (),
            });
            battle_state.battling = false;
        }
    }
}
