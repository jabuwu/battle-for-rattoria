use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    AddFixedEvent, AppState, AssetLibrary, Depth, EventSet, FixedInput, Transform2, UnitSpawnEvent,
    DEPTH_BATTLE_TEXT,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum BattleSystem {
    Enter,
    Start,
    Update,
}

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(
                battle_enter
                    .in_schedule(OnEnter(AppState::GameBattle))
                    .in_set(BattleSystem::Enter),
            );
        }
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
            battling: true,
            battle_time: 1.5,
        }
    }
}

#[derive(Clone)]
pub struct BattleConfig {
    pub friendly_units: usize,
    pub enemy_units: usize,
}

pub struct BattleStartEvent {
    pub config: BattleConfig,
}

#[derive(Default)]
pub struct BattleEndedEvent {
    _private: (),
}

fn battle_enter(
    mut battle_state: ResMut<BattleState>,
    mut start_events: EventWriter<BattleStartEvent>,
) {
    *battle_state = BattleState::default();
    start_events.send(BattleStartEvent {
        config: BattleConfig {
            friendly_units: 10,
            enemy_units: 10,
        },
    });
}

fn battle_start(
    mut start_events: EventReader<BattleStartEvent>,
    mut unit_spawn_events: EventWriter<UnitSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for start_event in start_events.iter() {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    "Battling!",
                    TextStyle {
                        font: asset_library.font_placeholder.clone(),
                        font_size: 72.,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..Default::default()
            },
            Transform2::from_xy(0., 300.),
            Depth::from(DEPTH_BATTLE_TEXT),
        ));
        let mut rng = thread_rng();
        for _ in 0..start_event.config.friendly_units {
            let x = rng.gen_range(-400.0..-160.0);
            let y = rng.gen_range(-80.0..80.0);
            unit_spawn_events.send(UnitSpawnEvent {
                position: Vec2::new(x, y),
                moving_right: true,
            });
        }
        for _ in 0..start_event.config.enemy_units {
            let x = rng.gen_range(160.0..400.0);
            let y = rng.gen_range(-80.0..80.0);
            unit_spawn_events.send(UnitSpawnEvent {
                position: Vec2::new(x, y),
                moving_right: false,
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
