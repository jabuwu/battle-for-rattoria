use std::fs::read_to_string;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::write;

use crate::{
    cleanup_non_persistent_entities, AppState, BattleConfig, BattleModifier, BattleModifiers,
    BattleStartEvent, BattleState, EventSet, UnitComposition, UnitKind,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Ui},
    EguiContexts,
};
use strum::IntoEnumIterator;

pub struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        let battle_config: BattleConfig = if let Ok(file_contents) = read_to_string("sandbox.ron") {
            ron::from_str(&file_contents).unwrap_or(BattleConfig::default())
        } else {
            BattleConfig::default()
        };

        if app.world.contains_resource::<State<AppState>>() {
            app.insert_resource(SandboxState {
                cleanup: false,
                start_battle: false,
                battle_config,
            })
            .add_system(sandbox_pre_update.in_base_set(CoreSet::PreUpdate))
            .add_system(
                sandbox_fixed_update
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(EventSet::<BattleStartEvent>::Sender),
            )
            .add_system(sandbox_ui.run_if(in_state(AppState::Sandbox)))
            .add_system(
                cleanup_non_persistent_entities
                    .in_base_set(CoreSet::First)
                    .run_if(sandbox_should_cleanup),
            )
            .add_system(sandbox_exit.run_if(in_state(AppState::Sandbox)));
        }
    }
}

#[derive(Resource)]
pub struct SandboxState {
    cleanup: bool,
    start_battle: bool,
    battle_config: BattleConfig,
}

fn sandbox_pre_update(mut example_state: ResMut<SandboxState>) {
    example_state.cleanup = false;
}

fn sandbox_should_cleanup(example_state: Res<SandboxState>) -> bool {
    example_state.cleanup
}

fn sandbox_fixed_update(
    mut example_state: ResMut<SandboxState>,
    mut battle_start_events: EventWriter<BattleStartEvent>,
) {
    if example_state.start_battle {
        battle_start_events.send(BattleStartEvent {
            config: example_state.battle_config.clone(),
            sandbox: false,
        });
        example_state.start_battle = false;
    }
}

fn sandbox_ui(
    mut contexts: EguiContexts,
    mut sandbox_state: ResMut<SandboxState>,
    battle_state: Res<BattleState>,
) {
    if !battle_state.battling() {
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

            ui.allocate_space(egui::Vec2::new(620., 0.));
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Friendly Units");
                    unit_composition_ui(ui, &mut sandbox_state.battle_config.friendly_units);
                });
                ui.vertical(|ui| {
                    ui.label("Friendly Modifiers");
                    battle_modifiers_ui(ui, &mut sandbox_state.battle_config.friendly_modifiers);
                });

                ui.vertical(|ui| {
                    ui.label("Enemy Units");
                    unit_composition_ui(ui, &mut sandbox_state.battle_config.enemy_units);
                });
                ui.vertical(|ui| {
                    ui.label("Enemy Modifiers");
                    battle_modifiers_ui(ui, &mut sandbox_state.battle_config.enemy_modifiers);
                });
            });

            ui.add_space(16.);

            ui.horizontal(|ui| {
                #[cfg(not(target_arch = "wasm32"))]
                if ui.button("Save").clicked() {
                    write(
                        "sandbox.ron",
                        ron::to_string(&sandbox_state.battle_config).unwrap(),
                    )
                    .unwrap();
                }

                if ui.button("Reset").clicked() {
                    sandbox_state.battle_config = BattleConfig::default();
                }
            });
            ui.add_space(16.);
            if ui.button("Start Battle").clicked() {
                sandbox_state.cleanup = true;
                sandbox_state.start_battle = true;
            }
        });
    }
}

fn sandbox_exit(
    mut battle_state: ResMut<BattleState>,
    mut next_app_state: ResMut<NextState<AppState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        battle_state.stop();
    }
    if keys.just_pressed(KeyCode::Escape) {
        next_app_state.set(AppState::MainMenu);
    }
}
