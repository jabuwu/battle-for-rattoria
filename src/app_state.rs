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
}

impl AppState {
    pub fn is_game_state(&self) -> bool {
        match self {
            AppState::MainMenu => false,
            AppState::GameStart => true,
            AppState::GameIntermission => true,
            AppState::GamePlanning => true,
            AppState::GameBattle => true,
        }
    }
}

pub fn in_game_state() -> impl FnMut(Res<State<AppState>>) -> bool + Clone {
    move |current_state: Res<State<AppState>>| current_state.0.is_game_state()
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