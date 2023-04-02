use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use strum::IntoEnumIterator;

use crate::{
    AddFixedEvent, AppState, Dialogue, DialogueLine, GameState, InteractionMode, InteractionStack,
    Script, SpawnSet, UnitKind, UpdateSet,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum PlanningSystem {
    Enter,
    Start,
    Update,
    Ui,
}

pub struct PlanningPlugin;

impl Plugin for PlanningPlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(
                planning_enter
                    .in_schedule(OnEnter(AppState::GamePlanning))
                    .in_set(PlanningSystem::Enter),
            );
        }
        app.init_resource::<PlanningState>()
            .add_fixed_event::<PlanningStartEvent>()
            .add_fixed_event::<PlanningEndedEvent>()
            .add_system(
                planning_start
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(PlanningSystem::Start)
                    .in_set(SpawnSet),
            )
            .add_system(
                planning_update
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(PlanningSystem::Update)
                    .in_set(UpdateSet),
            )
            .add_system(planning_ui.in_set(PlanningSystem::Ui));
    }
}

#[derive(Resource)]
pub struct PlanningState {
    planning: bool,
    start: bool,
}

impl Default for PlanningState {
    fn default() -> Self {
        Self {
            planning: false,
            start: false,
        }
    }
}

#[derive(Default)]
pub struct PlanningStartEvent;

#[derive(Default)]
pub struct PlanningEndedEvent {
    _private: (),
}

fn planning_enter(mut start_events: EventWriter<PlanningStartEvent>) {
    start_events.send_default();
}

fn planning_start(
    mut start_events: EventReader<PlanningStartEvent>,
    mut planning_state: ResMut<PlanningState>,
) {
    for _ in start_events.iter() {
        planning_state.planning = true;
    }
}

fn planning_update(
    mut planning_state: ResMut<PlanningState>,
    mut planning_ended_events: EventWriter<PlanningEndedEvent>,
) {
    if planning_state.planning && planning_state.start {
        planning_ended_events.send(PlanningEndedEvent { _private: () });
        planning_state.planning = false;
        planning_state.start = false;
    }
}

fn planning_ui(
    mut contexts: EguiContexts,
    mut game_state: ResMut<GameState>,
    mut planning_state: ResMut<PlanningState>,
    mut dialogue: ResMut<Dialogue>,
    interaction_stack: Res<InteractionStack>,
) {
    if planning_state.planning && interaction_stack.can_interact(InteractionMode::Game) {
        egui::Window::new("Planning").show(contexts.ctx_mut(), |ui| {
            ui.label(format!("Food remaining: {}", game_state.food));

            for unit_kind in UnitKind::iter() {
                let unit_cost = unit_kind.stats().cost;
                if ui
                    .button(format!(
                        "Feed {} ({} available, cost: {})",
                        unit_kind.name(),
                        game_state.available_army.get_count(unit_kind),
                        unit_cost,
                    ))
                    .clicked()
                    && game_state.available_army.get_count(unit_kind) > 0
                    && game_state.food >= unit_cost
                {
                    game_state.fed_army.mutate_count(unit_kind, |i| i + 1);
                    game_state.available_army.mutate_count(unit_kind, |i| i - 1);
                    game_state.food -= unit_cost;
                }
            }

            if ui.button("Start Battle").clicked() {
                if game_state.fed_army.total_units() > 0 {
                    planning_state.start = true;
                } else {
                    dialogue.queue(Script::new(vec![DialogueLine::message(
                        "You must feed some units first",
                    )]));
                }
            }
        });
    }
}
