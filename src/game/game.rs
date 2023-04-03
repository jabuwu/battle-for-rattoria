use bevy::prelude::*;

use crate::{in_game_state, AppState, DialogueEvent, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_system(game_dialogue_events);
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(game_update.run_if(in_game_state()));
        }
    }
}

fn game_dialogue_events(
    mut dialogue_events: EventReader<DialogueEvent>,
    mut game_state: ResMut<GameState>,
) {
    for dialogue_event in dialogue_events.iter() {
        match dialogue_event {
            DialogueEvent::AddUnits(units) => {
                for (unit_kind, count) in units.iter() {
                    game_state
                        .available_army
                        .mutate_count(*unit_kind, |i| i + *count);
                }
            }
            _ => {}
        }
    }
}

fn game_update(mut next_app_state: ResMut<NextState<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Key0) {
        next_app_state.set(AppState::MainMenu);
    }
}
