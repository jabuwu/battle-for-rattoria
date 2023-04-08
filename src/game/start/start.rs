use bevy::prelude::*;

use crate::{AppState, Articy, GameState};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum StartSystem {
    Enter,
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
fn start_enter(
    mut game_state: ResMut<GameState>,
    mut next_state: ResMut<NextState<AppState>>,
    articy: Res<Articy>,
) {
    *game_state = GameState::default();
    for (name, value) in articy.global_variables.iter() {
        game_state.global_variables.insert(name.clone(), *value);
    }
    game_state.checkpoint();
    next_state.set(AppState::GameIntermission);
}
