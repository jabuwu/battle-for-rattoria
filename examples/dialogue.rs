#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_game::{
    AssetLibraryPlugin, CommonPlugins, Dialogue, DialogueChoiceEvent, DialogueLine, GamePlugins,
    Persistent, Script,
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
        .add_system(choices)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Persistent));
}

#[derive(Hash)]
pub struct YesBlue;

#[derive(Hash)]
pub struct YesGreen;

#[derive(Hash)]
pub struct No;

fn ui(mut contexts: EguiContexts, mut dialogue: ResMut<Dialogue>) {
    egui::Window::new("Dialogue").show(contexts.ctx_mut(), |ui| {
        if ui.button("queue test dialogue").clicked() {
            dialogue.queue(Script::new(vec![
                DialogueLine::message("hello"),
                DialogueLine::message("world"),
                DialogueLine::branch(
                    "change the background color?",
                    vec![
                        (
                            YesBlue.into(),
                            "yes, blue",
                            vec![DialogueLine::message("done its blue!")],
                        ),
                        (
                            YesGreen.into(),
                            "yes, green",
                            vec![DialogueLine::message("done its green!")],
                        ),
                        (No.into(), "no", vec![]),
                    ],
                ),
            ]));
        }
        if ui.button("clear dialogue").clicked() {
            dialogue.clear();
        }
    });
}

fn choices(
    mut dialogue_choice_events: EventReader<DialogueChoiceEvent>,
    mut clear_color: ResMut<ClearColor>,
) {
    for dialogue_choice_event in dialogue_choice_events.iter() {
        if dialogue_choice_event.is(YesBlue) {
            clear_color.0 = Color::MIDNIGHT_BLUE;
        } else if dialogue_choice_event.is(YesGreen) {
            clear_color.0 = Color::DARK_GREEN;
        }
    }
}
