use bevy::prelude::*;

use crate::{in_game_state, AppState, ArticyDialogueInstruction, DialogueEvent, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_system(game_dialogue_events);
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(game_update.run_if(in_game_state));
        }
    }
}

fn game_dialogue_events(
    mut dialogue_events: EventReader<DialogueEvent>,
    mut game_state: ResMut<GameState>,
) {
    for dialogue_event in dialogue_events.iter() {
        match &dialogue_event.instruction {
            ArticyDialogueInstruction::AddUnits(unit_kind, count) => {
                game_state
                    .available_army
                    .mutate_count(*unit_kind, |i| i + *count);
            }
            ArticyDialogueInstruction::AddFood(count) => {
                game_state.food += count;
            }
            ArticyDialogueInstruction::AddItem(name) => {
                println!("ADD ITEM {}", name);
            }
            ArticyDialogueInstruction::SetGlobalVariable(name, value) => {
                game_state.global_variables.insert(name.clone(), *value);
            } /*DialogueEvent::GainIntel(unit_kind) => {

                  game_state.intel.can_see[*unit_kind] = true;
              }*/
        }
    }
}

fn game_update(mut next_app_state: ResMut<NextState<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Key0) {
        next_app_state.set(AppState::MainMenu);
    }
}
