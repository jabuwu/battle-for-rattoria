use std::mem::take;

use bevy::prelude::*;
use enum_map::{Enum, EnumMap};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    AddFixedEvent, AssetLibrary, BattlefieldSpawnEvent, DamageReceiveEvent, Depth, EventSet,
    HealthDieEvent, SpawnSet, Team, Transform2, Unit, UnitKind, UnitSpawnEvent, UpdateSet,
    DEPTH_BATTLE_TEXT,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum BattleSystem {
    Start,
    UnitDie,
    EndDetection,
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
                    .in_set(EventSet::<UnitSpawnEvent>::Sender)
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
                    .in_set(EventSet::<BattleEndedEvent>::Sender)
                    .after(EventSet::<DamageReceiveEvent>::Sender),
            );
    }
}

#[derive(Resource)]
pub struct BattleState {
    battling: bool,
    report: BattleReport,
    end_timer: f32,
    damage_inflicted: bool,
    time_since_last_damage: f32,
    friendly_modifiers: BattleModifiers,
    enemy_modifiers: BattleModifiers,
}

impl Default for BattleState {
    fn default() -> Self {
        Self {
            battling: false,
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
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct BattleConfig {
    pub friendly_units: UnitComposition,
    pub friendly_modifiers: BattleModifiers,
    pub enemy_units: UnitComposition,
    pub enemy_modifiers: BattleModifiers,
}

impl BattleConfig {
    pub fn get_units(&self, team: Team) -> &UnitComposition {
        match team {
            Team::Friendly => &self.friendly_units,
            Team::Enemy => &self.enemy_units,
        }
    }
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
    QuickAttack, // TODO
    ExtraSpeed,
    Fire, // TODO
    Ice,  // TODO
    Wet,  // TODO
    FriendlyFire,
    Cowardly,
    Sickness,   // TODO
    Explosive,  // TODO
    Combustion, // TODO
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
    mut unit_spawn_events: EventWriter<UnitSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for start_event in start_events.iter() {
        *battle_state = BattleState::default();
        battle_state.battling = true;
        battle_state.friendly_modifiers = start_event.config.friendly_modifiers;
        battle_state.enemy_modifiers = start_event.config.enemy_modifiers;
        commands.spawn((
            Text2dBundle {
                text: Text::from_sections([TextSection {
                    value: "Battling!".to_owned(),
                    style: TextStyle {
                        font: asset_library.font_placeholder.clone(),
                        font_size: 128.,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                }])
                .with_alignment(TextAlignment::Center),
                ..Default::default()
            },
            Transform2::from_xy(0., 600.),
            Depth::from(DEPTH_BATTLE_TEXT),
        ));
        battlefield_spawn_events.send_default();

        const X_DISTANCE: f32 = 400.;
        const Y_MIN: f32 = -300.;
        const Y_MAX: f32 = -100.;
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
    mut battle_ended_events: EventWriter<BattleEndedEvent>,
    mut damage_receive_events: EventReader<DamageReceiveEvent>,
    unit_query: Query<&Unit>,
    time: Res<FixedTime>,
) {
    if !battle_state.battling {
        return;
    }
    let mut friendly_exists = false;
    let mut enemy_exists = false;
    for unit in unit_query.iter() {
        if unit.team == Team::Friendly {
            friendly_exists = true;
        } else if unit.team == Team::Enemy {
            enemy_exists = true;
        }
    }
    for _ in damage_receive_events.iter() {
        battle_state.damage_inflicted = true;
        battle_state.time_since_last_damage = 0.;
    }
    battle_state.time_since_last_damage += time.period.as_secs_f32();
    if !friendly_exists || !enemy_exists {
        battle_state.end_timer += time.period.as_secs_f32();
    } else {
        battle_state.end_timer = 0.;
    }
    if battle_state.end_timer > 2.
        || (battle_state.damage_inflicted && battle_state.time_since_last_damage > 4.)
        || (!battle_state.damage_inflicted && battle_state.time_since_last_damage > 8.)
    {
        battle_ended_events.send(BattleEndedEvent {
            report: take(&mut battle_state.report),
            _private: (),
        });
        battle_state.battling = false;
    }
}
