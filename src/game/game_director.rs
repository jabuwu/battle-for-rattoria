use bevy::prelude::*;

use crate::{
    in_game_state, not_in_game_state_or_sandbox, AppState, BattleConfig, BattleEndedEvent,
    BattleModifiers, BattleStartEvent, BattleState, GameState, Intel, PlanningEndedEvent,
    PlanningStartEvent, PlanningState,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum GameDirectorSystem {
    PlanningEnter,
    BattleEnter,
    ChangeState,
    NotInGame,
}

pub struct GameDirectorPlugin;

impl Plugin for GameDirectorPlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(
                game_director_planning_enter
                    .run_if(in_game_state)
                    .in_schedule(OnEnter(AppState::GamePlanning))
                    .in_set(GameDirectorSystem::PlanningEnter),
            )
            .add_system(
                game_director_battle_enter
                    .run_if(in_game_state)
                    .in_schedule(OnEnter(AppState::GameBattle))
                    .in_set(GameDirectorSystem::BattleEnter),
            )
            .add_system(
                game_director_change_state
                    .run_if(in_game_state)
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(GameDirectorSystem::ChangeState),
            )
            .add_system(
                game_director_not_in_game
                    .run_if(not_in_game_state_or_sandbox)
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(GameDirectorSystem::NotInGame),
            );
        }
    }
}

#[derive(Component)]
pub struct GameDirector;

fn game_director_planning_enter(mut planning_start_events: EventWriter<PlanningStartEvent>) {
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
            friendly_modifiers: BattleModifiers::default(),
            enemy_units: game_state.quest.enemy_unit_comp(),
            enemy_modifiers: BattleModifiers::default(),
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

fn game_director_not_in_game(
    mut battle_state: ResMut<BattleState>,
    mut planning_state: ResMut<PlanningState>,
) {
    battle_state.stop();
    planning_state.stop();
}
