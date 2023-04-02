use bevy::prelude::*;

use crate::{AppState, BattleEndedEvent, PlanningEndedEvent};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum GameDirectorSystem {
    ChangeState,
}

pub struct GameDirectorPlugin;

impl Plugin for GameDirectorPlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(
                game_director_change_state
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(GameDirectorSystem::ChangeState),
            );
        }
    }
}

#[derive(Component)]
pub struct GameDirector;

fn game_director_change_state(
    mut battle_ended_events: EventReader<BattleEndedEvent>,
    mut planning_ended_events: EventReader<PlanningEndedEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    game_director: Query<&GameDirector>,
) {
    if game_director.get_single().is_ok() {
        for _ in battle_ended_events.iter() {
            next_state.set(AppState::GamePlanning);
        }
        for _ in planning_ended_events.iter() {
            next_state.set(AppState::GameBattle);
        }
    }
}
