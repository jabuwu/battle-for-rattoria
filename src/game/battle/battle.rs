use bevy::prelude::*;
use rand::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    AddFixedEvent, AssetLibrary, Depth, EventSet, FixedInput, Team, Transform2, UnitKind,
    UnitSpawnEvent, DEPTH_BATTLE_TEXT,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum BattleSystem {
    Start,
    Update,
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
                    .in_set(EventSet::<UnitSpawnEvent>::Sender)
                    .after(EventSet::<BattleStartEvent>::Sender),
            )
            .add_system(
                battle_update
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(BattleSystem::Update)
                    .in_set(EventSet::<BattleEndedEvent>::Sender),
            );
    }
}

#[derive(Resource)]
pub struct BattleState {
    battling: bool,
    battle_time: f32,
}

impl Default for BattleState {
    fn default() -> Self {
        Self {
            battling: false,
            battle_time: 12.,
        }
    }
}

#[derive(Clone)]
pub struct BattleConfig {
    pub friendly_units: UnitComposition,
    pub enemy_units: UnitComposition,
}

#[derive(Default, Clone)]
pub struct UnitComposition {
    pub peasants: usize,
    pub warriors: usize,
}

impl UnitComposition {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn get_count(&self, unit_kind: UnitKind) -> usize {
        match unit_kind {
            UnitKind::Peasant => self.peasants,
            UnitKind::Warrior => self.warriors,
        }
    }

    pub fn set_count(&mut self, unit_kind: UnitKind, i: usize) {
        match unit_kind {
            UnitKind::Peasant => self.peasants = i,
            UnitKind::Warrior => self.warriors = i,
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

    pub fn total_units(&self) -> usize {
        let mut total = 0;
        for unit_kind in UnitKind::iter() {
            total += self.get_count(unit_kind);
        }
        total
    }
}

pub struct BattleStartEvent {
    pub config: BattleConfig,
}

#[derive(Default)]
pub struct BattleEndedEvent {
    _private: (),
}

fn battle_start(
    mut start_events: EventReader<BattleStartEvent>,
    mut battle_state: ResMut<BattleState>,
    mut unit_spawn_events: EventWriter<UnitSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for start_event in start_events.iter() {
        *battle_state = BattleState::default();
        battle_state.battling = true;
        commands.spawn((
            Text2dBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "Battling!".to_owned(),
                        style: TextStyle {
                            font: asset_library.font_placeholder.clone(),
                            font_size: 72.,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "\nPress space to skip".to_owned(),
                        style: TextStyle {
                            font: asset_library.font_placeholder.clone(),
                            font_size: 22.,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                ])
                .with_alignment(TextAlignment::Center),
                ..Default::default()
            },
            Transform2::from_xy(0., 300.),
            Depth::from(DEPTH_BATTLE_TEXT),
        ));
        let mut rng = thread_rng();
        for _ in 0..start_event.config.friendly_units.peasants {
            let x = rng.gen_range(-400.0..-160.0);
            let y = rng.gen_range(-40.0..40.0);
            unit_spawn_events.send(UnitSpawnEvent {
                kind: UnitKind::Peasant,
                position: Vec2::new(x, y),
                team: Team::Friendly,
            });
        }
        for _ in 0..start_event.config.friendly_units.warriors {
            let x = rng.gen_range(-700.0..-560.0);
            let y = rng.gen_range(-40.0..40.0);
            unit_spawn_events.send(UnitSpawnEvent {
                kind: UnitKind::Warrior,
                position: Vec2::new(x, y),
                team: Team::Friendly,
            });
        }
        for _ in 0..start_event.config.enemy_units.peasants {
            let x = rng.gen_range(160.0..400.0);
            let y = rng.gen_range(-40.0..40.0);
            unit_spawn_events.send(UnitSpawnEvent {
                kind: UnitKind::Peasant,
                position: Vec2::new(x, y),
                team: Team::Enemy,
            });
        }
        for _ in 0..start_event.config.enemy_units.warriors {
            let x = rng.gen_range(560.0..700.0);
            let y = rng.gen_range(-40.0..40.0);
            unit_spawn_events.send(UnitSpawnEvent {
                kind: UnitKind::Warrior,
                position: Vec2::new(x, y),
                team: Team::Enemy,
            });
        }
    }
}

fn battle_update(
    mut battle_state: ResMut<BattleState>,
    mut battle_ended_events: EventWriter<BattleEndedEvent>,
    time: Res<FixedTime>,
    keys: Res<FixedInput<KeyCode>>,
) {
    if !battle_state.battling {
        return;
    }
    battle_state.battle_time -= time.period.as_secs_f32();
    if battle_state.battle_time < 0. || keys.just_pressed(KeyCode::Space) {
        battle_ended_events.send(BattleEndedEvent { _private: () });
        battle_state.battling = false;
    }
}
