use std::mem::take;

use bevy::prelude::*;

use crate::{
    in_game_state, not_in_game_state_or_sandbox, AppState, Banner, BattleConfig, BattleEndedEvent,
    BattleModifier, BattleModifiers, BattleStartEvent, BattleState, GameState, Intel, Item,
    PlanningEndedEvent, PlanningStartEvent, PlanningState,
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
    let mut friendly_modifiers = BattleModifiers::default();
    let mut sick = false;
    game_state.used_items = vec![];
    for item in take(&mut game_state.consumed_items) {
        for modifier in item.modifiers() {
            friendly_modifiers[modifier] = true;
            if modifier == BattleModifier::Sickness {
                sick = true;
            }
        }
        if item == Item::BogHardWeeds {
            game_state
                .global_variables
                .insert("UsedBogHardWeed".to_owned(), true);
        }
        game_state.used_items.push(item);
    }
    game_state.apply_sickness(sick);
    battle_start_events.send(BattleStartEvent {
        config: BattleConfig {
            friendly_units,
            friendly_modifiers,
            friendly_banner: Banner::Player,
            enemy_units: game_state.quest.enemy_unit_composition(),
            enemy_modifiers: game_state.quest.enemy_modifiers(),
            enemy_banner: match game_state.quest.war_chef {
                0 => Banner::WarChef1,
                1 => Banner::WarChef2,
                2 => Banner::WarChef3,
                3 => Banner::WarChef4,
                _ => Banner::WarChef5,
            },
        },
        sandbox: false,
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
            if battle_ended_event.report.victory {
                game_state
                    .available_army
                    .subtract_units(&battle_ended_event.report.dead_units);
                if game_state.quest.next() {
                    game_state.checkpoint();
                    next_state.set(AppState::GameIntermission);
                } else {
                    next_state.set(AppState::GameOutro);
                }
            } else {
                next_state.set(AppState::GameRewind);
            }
        }
        for planning_ended_event in planning_ended_events.iter() {
            if planning_ended_event.rewind {
                game_state.rewind();
                game_state.checkpoint();
                next_state.set(AppState::GameRewind);
            } else if planning_ended_event.skip {
                game_state.quest.next();
                game_state.checkpoint();
                next_state.set(AppState::GameIntermission);
            } else {
                next_state.set(AppState::GameBattle);
            }
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
