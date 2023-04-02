use bevy::prelude::*;

use crate::{in_game_state, AppState, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>();
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(game_update.run_if(in_game_state()));
        }
    }
}

fn game_update(mut next_app_state: ResMut<NextState<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Key0) {
        next_app_state.set(AppState::MainMenu);
    }
}
