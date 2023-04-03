use bevy::prelude::*;

use crate::{AppState, Dialogue, GameState};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum IntermissionSystem {
    Enter,
    Update,
}

pub struct IntermissionPlugin;

impl Plugin for IntermissionPlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(
                intermission_enter
                    .in_schedule(OnEnter(AppState::GameIntermission))
                    .in_set(IntermissionSystem::Enter),
            )
            .add_system(
                intermission_update
                    .run_if(in_state(AppState::GameIntermission))
                    .in_set(IntermissionSystem::Update),
            );
        }
    }
}

fn intermission_enter(mut dialogue: ResMut<Dialogue>, game_state: Res<GameState>) {
    if let Some(script) = game_state.quest.preplanning_script() {
        dialogue.queue(script);
    }
}

fn intermission_update(mut next_state: ResMut<NextState<AppState>>, dialogue: Res<Dialogue>) {
    if !dialogue.active() {
        next_state.set(AppState::GamePlanning);
    }
}
