#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_game::{
    cleanup_non_persistent_entities, AssetLibraryPlugin, BattleConfig, BattleStartEvent,
    CommonPlugins, EventSet, GamePlugins, Persistent,
};

fn main() {
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
            battle_config: BattleConfig {
                friendly_units: 10,
                enemy_units: 10,
            },
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
        });
        example_state.start_battle = false;
    }
}

fn ui(mut contexts: EguiContexts, mut example_state: ResMut<ExampleState>) {
    egui::Window::new("Battle").show(contexts.ctx_mut(), |ui| {
        ui.label("Friendly Units");
        ui.add(
            egui::DragValue::new(&mut example_state.battle_config.friendly_units)
                .clamp_range(1..=100),
        );
        ui.label("Enemy Units");
        ui.add(
            egui::DragValue::new(&mut example_state.battle_config.enemy_units).clamp_range(1..=100),
        );
        if ui.button("Start Battle").clicked() {
            example_state.cleanup = true;
            example_state.start_battle = true;
        }
    });
}
