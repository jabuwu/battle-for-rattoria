use bevy::prelude::*;

use crate::cleanup_non_persistent_entities;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, States)]
pub enum AppState {
    #[default]
    MainMenu,
    GameStart,
    GameIntermission,
    GamePlanning,
    GameBattle,
    GameRewind,
    Sandbox,
}

impl AppState {
    pub fn is_game_state(&self) -> bool {
        match self {
            AppState::MainMenu => false,
            AppState::GameStart => true,
            AppState::GameIntermission => true,
            AppState::GamePlanning => true,
            AppState::GameBattle => true,
            AppState::GameRewind => true,
            AppState::Sandbox => false,
        }
    }
}

pub fn in_game_state(current_state: Res<State<AppState>>) -> bool {
    current_state.0.is_game_state()
}

pub fn not_in_game_state_or_sandbox(current_state: Res<State<AppState>>) -> bool {
    !current_state.0.is_game_state() && current_state.0 != AppState::Sandbox
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();
        for state in AppState::variants() {
            app.add_system(cleanup_non_persistent_entities.in_schedule(OnExit(state)));
        }
    }
}
