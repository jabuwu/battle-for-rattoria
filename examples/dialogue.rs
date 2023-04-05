#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_game::{
    Articy, ArticyDialogueInstruction, AssetLibraryPlugin, CommonPlugins, Dialogue, DialogueEvent,
    GamePlugins, GameState, Persistent, Script,
};

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dialogue".to_owned(),
                resolution: (1280., 768.).into(),
                canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(AssetLibraryPlugin)
        .add_plugins(CommonPlugins)
        .add_plugins(GamePlugins)
        .add_startup_system(setup)
        .add_system(ui)
        .add_system(events)
        .run();
}

fn setup(mut commands: Commands, mut game_state: ResMut<GameState>, articy: Res<Articy>) {
    for (name, value) in articy.global_variables.iter() {
        game_state.global_variables.insert(name.clone(), *value);
    }
    commands.spawn((Camera2dBundle::default(), Persistent));
}

#[derive(Hash)]
pub struct YesBlue;

#[derive(Hash)]
pub struct YesGreen;

fn ui(
    mut contexts: EguiContexts,
    mut dialogue: ResMut<Dialogue>,
    mut game_state: ResMut<GameState>,
    articy: Res<Articy>,
) {
    egui::Window::new("Dialogue").show(contexts.ctx_mut(), |ui| {
        for dialogue_str in [
            "WC1B1", "WC1B2", "WC1B3", "WC2B1", "WC2B2", "WC2B3", "WC3B1", "WC3B2", "WC3B3",
            "WC3B4",
        ] {
            if ui.button(dialogue_str).clicked() {
                dialogue.queue(
                    Script::new(articy.dialogues[dialogue_str].clone()),
                    game_state.as_mut(),
                );
            }
        }
        if ui.button("clear dialogue").clicked() {
            dialogue.clear();
        }
    });
}

fn events(mut dialogue_events: EventReader<DialogueEvent>) {
    for dialogue_event in dialogue_events.iter() {
        match &dialogue_event.instruction {
            ArticyDialogueInstruction::AddUnits(unit_kind, amount) => {
                println!("Add {} {} unit(s)", amount, unit_kind.name());
            }
            ArticyDialogueInstruction::AddFood(amount) => {
                println!("Add {} food", amount);
            }
            ArticyDialogueInstruction::AddItem(name) => {
                println!("Add item: {}", name);
            }
            ArticyDialogueInstruction::SetGlobalVariable(name, value) => {
                println!("Set global variable: {} {:?}", name, value);
            }
        }
    }
}
