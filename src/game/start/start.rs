use bevy::prelude::*;

use crate::{AppState, GameState};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum StartSystem {
    Enter,
    Start,
    Update,
    Ui,
}

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(
                start_enter
                    .in_schedule(OnEnter(AppState::GameStart))
                    .in_set(StartSystem::Enter),
            );
        }
    }
}
fn start_enter(mut game_state: ResMut<GameState>, mut next_state: ResMut<NextState<AppState>>) {
    *game_state = GameState::default();
    next_state.set(AppState::GamePlanning);
}
