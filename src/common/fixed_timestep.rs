use std::{hash::Hash, marker::PhantomData};

use bevy::{
    input::InputSystem,
    prelude::*,
    reflect::Reflect,
    transform::systems::{propagate_transforms, sync_simple_transforms},
};

use crate::transform2_propagate;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct FixedInputSystem;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum FixedTransformSystem {
    Transform2Propagate,
    TransformPropagate,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
#[system_set(base)]
pub enum FixedSet {
    Update,
    UpdateFlush,
    PostUpdate,
}

pub struct FixedTimestepPlugin;

impl Plugin for FixedTimestepPlugin {
    fn build(&self, app: &mut App) {
        {
            let schedule = app.get_schedule_mut(CoreSchedule::FixedUpdate).unwrap();
            schedule
                .set_default_base_set(FixedSet::Update)
                .configure_set(FixedSet::Update.before(FixedSet::UpdateFlush))
                .configure_set(FixedSet::UpdateFlush.before(FixedSet::PostUpdate));
        }
        app.add_fixed_input::<KeyCode>()
            .add_fixed_input::<ScanCode>()
            .add_fixed_input::<MouseButton>()
            .add_fixed_input::<GamepadButton>()
            .add_system(
                apply_system_buffers
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_base_set(FixedSet::UpdateFlush),
            )
            .add_systems((
                transform2_propagate
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(FixedTransformSystem::Transform2Propagate)
                    .in_base_set(FixedSet::PostUpdate)
                    .before(FixedTransformSystem::TransformPropagate),
                sync_simple_transforms
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(FixedTransformSystem::TransformPropagate)
                    .in_base_set(FixedSet::PostUpdate),
                propagate_transforms
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(FixedTransformSystem::TransformPropagate)
                    .in_base_set(FixedSet::PostUpdate),
            ));
    }
}

trait AddFixedInput {
    fn add_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(&mut self) -> &mut Self;
}

impl AddFixedInput for App {
    fn add_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(&mut self) -> &mut Self {
        self.init_resource::<FixedInput<T>>().add_systems((
            fixed_input_update::<T>
                .in_base_set(CoreSet::PreUpdate)
                .after(InputSystem),
            fixed_input_clear::<T>
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(FixedInputSystem)
                .in_base_set(FixedSet::PostUpdate),
        ));
        self
    }
}

#[derive(Clone, Deref, DerefMut, Debug, Reflect, Resource)]
#[reflect(Default)]
pub struct FixedInput<T: Copy + Eq + Hash + Send + Sync + 'static>(Input<T>);

impl<T: Copy + Eq + Hash + Send + Sync + 'static> Default for FixedInput<T> {
    fn default() -> Self {
        Self(Input::default())
    }
}

fn fixed_input_update<T: Copy + Eq + Hash + Send + Sync + 'static>(
    mut fixed_input: ResMut<FixedInput<T>>,
    input: Res<Input<T>>,
) {
    for pressed in input.get_just_pressed() {
        fixed_input.press(*pressed);
    }
    for released in input.get_just_released() {
        fixed_input.release(*released);
    }
}

fn fixed_input_clear<T: Copy + Eq + Hash + Send + Sync + 'static>(
    mut fixed_input: ResMut<FixedInput<T>>,
) {
    fixed_input.clear();
}

pub trait AddFixedEvent {
    fn add_fixed_event<T: Event>(&mut self) -> &mut Self;
}

impl AddFixedEvent for App {
    fn add_fixed_event<T: Event>(&mut self) -> &mut Self {
        self.init_resource::<EventClearFlag<T>>()
            .init_resource::<Events<T>>()
            .add_systems((fixed_events_clear_flag::<T>,).in_schedule(CoreSchedule::FixedUpdate))
            .add_systems((fixed_events_clear::<T>,).in_base_set(CoreSet::Last));
        self
    }
}

#[derive(Resource)]
struct EventClearFlag<T: Event> {
    clear: bool,
    _marker: PhantomData<T>,
}

impl<T: Event> Default for EventClearFlag<T> {
    fn default() -> Self {
        Self {
            clear: false,
            _marker: PhantomData,
        }
    }
}

fn fixed_events_clear_flag<T: Event>(mut event_clear_flag: ResMut<EventClearFlag<T>>) {
    event_clear_flag.clear = true;
}

fn fixed_events_clear<T: Event>(
    mut event_clear_flag: ResMut<EventClearFlag<T>>,
    mut fixed_events: ResMut<Events<T>>,
) {
    if event_clear_flag.clear {
        fixed_events.update();
        event_clear_flag.clear = false;
    }
}
