use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    in_game_state, AppState, ArticyDialogueInstruction, DebugDrawSettings, DialogueEvent, GameState,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_system(game_dialogue_events)
            .add_system(game_debug);
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
            ArticyDialogueInstruction::AddItem(item) => {
                game_state.inventory.add(*item);
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

fn game_debug(
    mut contexts: EguiContexts,
    mut debug_draw_settings: ResMut<DebugDrawSettings>,
    game_state: Res<GameState>,
) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        ui.checkbox(&mut debug_draw_settings.draw_hit_boxes, "Draw Hitboxes");
        ui.checkbox(&mut debug_draw_settings.draw_hurt_boxes, "Draw Hurtboxes");
        ui.checkbox(&mut debug_draw_settings.draw_feelers, "Draw Feelers");
        ui.collapsing("Variables", |ui| {
            for (name, value) in game_state.global_variables.iter() {
                ui.label(format!("{}: {}", name, *value));
            }
        });
    });
}
