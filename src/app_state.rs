use bevy::prelude::*;

use crate::cleanup_non_persistent_entities;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
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
