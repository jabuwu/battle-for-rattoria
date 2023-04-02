use bevy::prelude::*;

use crate::{AddFixedEvent, AppState, AssetLibrary, FixedInput};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum PlanningSystem {
    Enter,
    Start,
    Update,
    LeaveState,
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
        app.add_fixed_event::<PlanningStartEvent>()
            .add_fixed_event::<PlanningEndedEvent>()
            .add_system(
                planning_start
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(PlanningSystem::Start),
            )
            .add_system(
                planning_update
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(PlanningSystem::Update),
            );
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
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in start_events.iter() {
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                "Planning Phase\nPress space to battle",
                TextStyle {
                    font: asset_library.font_placeholder.clone(),
                    font_size: 72.,
                    color: Color::WHITE,
                    ..Default::default()
                },
            )
            .with_alignment(TextAlignment::Center),
            ..Default::default()
        });
    }
}

fn planning_update(
    mut planning_ended_events: EventWriter<PlanningEndedEvent>,
    keys: Res<FixedInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        planning_ended_events.send(PlanningEndedEvent { _private: () });
    }
}
