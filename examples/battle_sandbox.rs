#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{read_to_string, write};

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Ui},
    EguiContexts,
};
use bevy_game::{
    cleanup_non_persistent_entities, AssetLibraryPlugin, BattleConfig, BattleModifier,
    BattleModifiers, BattleStartEvent, CommonPlugins, EventSet, GamePlugins, Persistent,
    UnitComposition, UnitKind,
};
use strum::IntoEnumIterator;

fn main() {
    let battle_config: BattleConfig = if let Ok(file_contents) = read_to_string("sandbox.ron") {
        ron::from_str(&file_contents).unwrap_or(BattleConfig::default())
    } else {
        BattleConfig::default()
    };

    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Battle Sandbox".to_owned(),
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
        .insert_resource(ExampleState {
            cleanup: false,
            start_battle: false,
            battle_config,
        })
        .add_system(setup.in_schedule(CoreSchedule::Startup))
        .add_system(pre_update.in_base_set(CoreSet::PreUpdate))
        .add_system(
            fixed_update
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(EventSet::<BattleStartEvent>::Sender),
        )
        .add_system(ui)
        .add_system(
            cleanup_non_persistent_entities
                .in_base_set(CoreSet::First)
                .run_if(should_cleanup),
        )
        .run();
}

#[derive(Resource)]
pub struct ExampleState {
    cleanup: bool,
    start_battle: bool,
    battle_config: BattleConfig,
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Persistent));
}

fn pre_update(mut example_state: ResMut<ExampleState>) {
    example_state.cleanup = false;
}

fn should_cleanup(example_state: Res<ExampleState>) -> bool {
    example_state.cleanup
}

fn fixed_update(
    mut example_state: ResMut<ExampleState>,
    mut battle_start_events: EventWriter<BattleStartEvent>,
) {
    if example_state.start_battle {
        battle_start_events.send(BattleStartEvent {
            config: example_state.battle_config.clone(),
            sandbox: true,
        });
        example_state.start_battle = false;
    }
}

fn ui(mut contexts: EguiContexts, mut example_state: ResMut<ExampleState>) {
    egui::Window::new("Battle").show(contexts.ctx_mut(), |ui| {
        fn unit_composition_ui(ui: &mut Ui, unit_composition: &mut UnitComposition) {
            for unit_kind in UnitKind::iter() {
                ui.horizontal(|ui| {
                    let mut count = unit_composition.get_count(unit_kind);
                    if ui.button("-").clicked() && count > 0 {
                        count -= 1;
                    }
                    ui.add(egui::DragValue::new(&mut count).clamp_range(0..=100));
                    if ui.button("+").clicked() && count < 100 {
                        count += 1;
                    }
                    unit_composition.set_count(unit_kind, count);
                    ui.label(unit_kind.name_plural());
                });
            }
        }
        fn battle_modifiers_ui(ui: &mut Ui, battle_modifiers: &mut BattleModifiers) {
            for battle_modifier in BattleModifier::iter() {
                let mut checked = battle_modifiers[battle_modifier];
                ui.checkbox(&mut checked, battle_modifier.name());
                battle_modifiers[battle_modifier] = checked;
            }
        }

        ui.add_space(16.);

        ui.allocate_space(egui::Vec2::new(550., 0.));
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Friendly Units");
                unit_composition_ui(ui, &mut example_state.battle_config.friendly_units);
            });
            ui.vertical(|ui| {
                ui.label("Friendly Modifiers");
                battle_modifiers_ui(ui, &mut example_state.battle_config.friendly_modifiers);
            });

            ui.vertical(|ui| {
                ui.label("Enemy Units");
                unit_composition_ui(ui, &mut example_state.battle_config.enemy_units);
            });
            ui.vertical(|ui| {
                ui.label("Enemy Modifiers");
                battle_modifiers_ui(ui, &mut example_state.battle_config.enemy_modifiers);
            });
        });

        ui.add_space(16.);

        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                write(
                    "sandbox.ron",
                    ron::to_string(&example_state.battle_config).unwrap(),
                )
                .unwrap();
            }

            if ui.button("Reset").clicked() {
                example_state.battle_config = BattleConfig::default();
            }
        });
    });
    egui::Window::new("Start").show(contexts.ctx_mut(), |ui| {
        if ui.button("Start Battle").clicked() {
            example_state.cleanup = true;
            example_state.start_battle = true;
        }
    });
}
