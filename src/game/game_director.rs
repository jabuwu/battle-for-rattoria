use bevy::prelude::*;

use crate::{
    AppState, BattleConfig, BattleEndedEvent, BattleStartEvent, GameState, Intel,
    PlanningEndedEvent, PlanningStartEvent,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum GameDirectorSystem {
    PlanningEnter,
    BattleEnter,
    ChangeState,
}

pub struct GameDirectorPlugin;

impl Plugin for GameDirectorPlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(
                game_director_planning_enter
                    .in_schedule(OnEnter(AppState::GamePlanning))
                    .in_set(GameDirectorSystem::PlanningEnter),
            )
            .add_system(
                game_director_battle_enter
                    .in_schedule(OnEnter(AppState::GameBattle))
                    .in_set(GameDirectorSystem::BattleEnter),
            )
            .add_system(
                game_director_change_state
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(GameDirectorSystem::ChangeState),
            );
        }
    }
}

#[derive(Component)]
pub struct GameDirector;

fn game_director_planning_enter(
    mut game_state: ResMut<GameState>,
    mut planning_start_events: EventWriter<PlanningStartEvent>,
) {
    game_state.food += 15;
    planning_start_events.send_default();
}

fn game_director_battle_enter(
    mut battle_start_events: EventWriter<BattleStartEvent>,
    mut game_state: ResMut<GameState>,
) {
    let friendly_units = game_state.get_and_reset_fed_army();
    battle_start_events.send(BattleStartEvent {
        config: BattleConfig {
            friendly_units,
            enemy_units: game_state.quest.enemy_unit_comp(),
        },
    });
    game_state.intel = Intel::default();
}

fn game_director_change_state(
    mut battle_ended_events: EventReader<BattleEndedEvent>,
    mut planning_ended_events: EventReader<PlanningEndedEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<GameState>,
    game_director: Query<&GameDirector>,
) {
    if game_director.get_single().is_ok() {
        for battle_ended_event in battle_ended_events.iter() {
            game_state
                .available_army
                .subtract_units(&battle_ended_event.report.dead_units);
            game_state.quest.battle += 1;
            next_state.set(AppState::GameIntermission);
        }
        for _ in planning_ended_events.iter() {
            next_state.set(AppState::GameBattle);
        }
    }
}
