#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_game::{
    AssetLibraryPlugin, CommonPlugins, Dialogue, DialogueEvent, DialogueLine, GamePlugins,
    Persistent, Script, UnitKind,
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
        .add_system(events)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Persistent));
}

#[derive(Hash)]
pub struct YesBlue;

#[derive(Hash)]
pub struct YesGreen;

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
                            DialogueEvent::Context(YesBlue.into()),
                            "yes, blue",
                            vec![DialogueLine::message("done its blue!")],
                        ),
                        (
                            DialogueEvent::Context(YesGreen.into()),
                            "yes, green",
                            vec![DialogueLine::message("done its green!")],
                        ),
                        (DialogueEvent::None, "no", vec![]),
                    ],
                ),
            ]));
        }
        if ui.button("queue events dialogue").clicked() {
            dialogue.queue(Script::new(vec![
                DialogueLine::message_and(
                    "added 5 peasants",
                    DialogueEvent::AddUnits(UnitKind::Peasant, 5),
                ),
                DialogueLine::message_and(
                    "added 2 archors and 3 mages",
                    DialogueEvent::AddUnits(UnitKind::Peasant, 2)
                        .and(DialogueEvent::AddUnits(UnitKind::Mage, 3)),
                ),
                DialogueLine::message_and(
                    "added 1 brute and changed background color to green",
                    DialogueEvent::AddUnits(UnitKind::Brute, 1)
                        .and(DialogueEvent::Context(YesGreen.into())),
                ),
            ]));
        }
        if ui.button("clear dialogue").clicked() {
            dialogue.clear();
        }
    });
}

fn choices(mut dialogue_events: EventReader<DialogueEvent>, mut clear_color: ResMut<ClearColor>) {
    for dialogue_event in dialogue_events.iter() {
        if dialogue_event.is(YesBlue) {
            clear_color.0 = Color::MIDNIGHT_BLUE;
        } else if dialogue_event.is(YesGreen) {
            clear_color.0 = Color::DARK_GREEN;
        }
    }
}

fn events(mut dialogue_events: EventReader<DialogueEvent>) {
    for dialogue_event in dialogue_events.iter() {
        match dialogue_event {
            DialogueEvent::None => unreachable!(),
            DialogueEvent::AddUnits(unit_kind, amount) => {
                println!("Add {} {} unit(s)", amount, unit_kind.name());
            }
            DialogueEvent::GainIntel(unit_kind) => println!("Gained intel on {}", unit_kind.name()),
            DialogueEvent::Context(_) => println!("Context Event"),
            DialogueEvent::Multiple(_) => unreachable!(),
        }
    }
}
