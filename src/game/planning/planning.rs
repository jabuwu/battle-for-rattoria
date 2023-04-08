use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use strum::IntoEnumIterator;

use crate::{
    AddFixedEvent, AppState, Articy, Dialogue, GameState, InteractionMode, InteractionStack,
    PersistentGameState, Script, SpawnSet, UnitKind, UpdateSet,
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
    skip: bool,
    rewind: bool,
}

impl PlanningState {
    pub fn stop(&mut self) {
        self.planning = false;
    }
}

impl Default for PlanningState {
    fn default() -> Self {
        Self {
            planning: false,
            start: false,
            skip: false,
            rewind: false,
        }
    }
}

#[derive(Default)]
pub struct PlanningStartEvent;

#[derive(Default)]
pub struct PlanningEndedEvent {
    pub skip: bool,
    pub rewind: bool,
    _private: (),
}

fn planning_enter(mut start_events: EventWriter<PlanningStartEvent>) {
    start_events.send_default();
}

fn planning_start(
    mut start_events: EventReader<PlanningStartEvent>,
    mut planning_state: ResMut<PlanningState>,
    mut dialogue: ResMut<Dialogue>,
    mut persistent_game_state: ResMut<PersistentGameState>,
    mut game_state: ResMut<GameState>,
    articy: Res<Articy>,
) {
    for _ in start_events.iter() {
        planning_state.planning = true;
        if let Some(tutorial_index) = match game_state.quest.war_chef {
            0 => match game_state.quest.battle {
                0 => Some(0),
                1 => Some(1),
                2 => Some(2),
                _ => None,
            },
            1 => match game_state.quest.battle {
                0 => Some(3),
                _ => None,
            },
            _ => None,
        } {
            let tutorials = &["Tutorial1", "Tutorial2", "Tutorial3", "Tutorial4"];
            if persistent_game_state.show_tutorial[tutorial_index] {
                persistent_game_state.show_tutorial[tutorial_index] = false;
                dialogue.queue(
                    Script::new(
                        articy
                            .dialogues
                            .get(tutorials[tutorial_index])
                            .unwrap()
                            .clone(),
                    ),
                    game_state.as_mut(),
                );
            }
        }
    }
}

fn planning_update(
    mut planning_state: ResMut<PlanningState>,
    mut planning_ended_events: EventWriter<PlanningEndedEvent>,
) {
    if planning_state.planning
        && (planning_state.start || planning_state.skip || planning_state.rewind)
    {
        planning_ended_events.send(PlanningEndedEvent {
            skip: planning_state.skip && !planning_state.rewind,
            rewind: planning_state.rewind,
            _private: (),
        });
        planning_state.planning = false;
        planning_state.start = false;
        planning_state.skip = false;
        planning_state.rewind = false;
    }
}

fn planning_ui(
    mut contexts: EguiContexts,
    mut game_state: ResMut<GameState>,
    mut planning_state: ResMut<PlanningState>,
    mut dialogue: ResMut<Dialogue>,
    interaction_stack: Res<InteractionStack>,
    articy: Res<Articy>,
) {
    if planning_state.planning && interaction_stack.can_interact(InteractionMode::Game) {
        egui::Window::new("Planning").show(contexts.ctx_mut(), |ui| {
            ui.label(format!("Food remaining: {}", game_state.food));

            for unit_kind in UnitKind::iter() {
                let unit_cost = unit_kind.stats().cost;
                if ui
                    .button(format!(
                        "Feed {} ({} available, {} ready, {} sick, cost: {})",
                        unit_kind.name(),
                        game_state.available_army.get_count(unit_kind),
                        game_state.fed_army.get_count(unit_kind),
                        game_state.sick_army.get_count(unit_kind),
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

            ui.add_space(16.);

            ui.label("Inventory");
            if game_state.inventory.is_empty() {
                ui.label("NO ITEMS");
            } else {
                let mut remove_item = None;
                for (item_index, item) in
                    game_state.inventory.items().clone().into_iter().enumerate()
                {
                    if ui.button(format!("Use {}", item.name())).clicked() {
                        game_state.consumed_items.push(item);
                        remove_item = Some(item_index);
                    }
                }
                if let Some(remove_item) = remove_item {
                    game_state.inventory.remove(remove_item);
                }
            }

            ui.add_space(16.);

            if ui.button("Start Battle").clicked() {
                if let Some(tutorial_dialogue) = tutorial_dialogue(game_state.as_ref()) {
                    dialogue.queue(
                        Script::new(articy.dialogues.get(tutorial_dialogue).unwrap().clone()),
                        game_state.as_mut(),
                    );
                } else {
                    planning_state.start = true;
                }
            }

            ui.add_space(32.);

            ui.horizontal(|ui| {
                if ui.button("Skip Battle").clicked() {
                    planning_state.skip = true;
                }

                if game_state.can_rewind() {
                    if ui.button("Rewind to Previous Battle").clicked() {
                        planning_state.rewind = true;
                    }
                }
            });
        });
        egui::Window::new("Intel").show(contexts.ctx_mut(), |ui| {
            ui.label("Enemy's Army");
            let enemy_comp = game_state.quest.enemy_unit_composition();
            for unit_kind in UnitKind::iter() {
                if game_state.intel.can_see[unit_kind] {
                    ui.label(format!(
                        "{}: {}",
                        unit_kind.name_plural(),
                        enemy_comp.get_count(unit_kind)
                    ));
                } else {
                    ui.label(format!("{}: ???", unit_kind.name_plural()));
                }
            }
        });
    }
}

fn tutorial_dialogue(game_state: &GameState) -> Option<&'static str> {
    if game_state.quest.war_chef == 0 {
        match game_state.quest.battle {
            0 => {
                if game_state.available_army.peasants != 0 {
                    return Some("Tutorial1");
                }
            }
            1 => {
                if game_state.available_army.peasants != 0
                    || game_state.available_army.warriors != 0
                {
                    return Some("Tutorial2");
                }
            }
            2 => {
                if game_state.available_army.peasants != 0
                    || game_state.available_army.warriors != 0
                    || !game_state.inventory.is_empty()
                {
                    return Some("Tutorial3");
                }
            }
            _ => {}
        }
    }
    if game_state.fed_army.total_units() == 0 {
        Some("MustFeedUnits")
    } else {
        None
    }
}
