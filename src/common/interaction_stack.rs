use bevy::prelude::*;
use enum_map::{enum_map, Enum, EnumMap};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct InteractionStackPlugin;

impl Plugin for InteractionStackPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InteractionStack>();
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Enum, EnumIter)]
pub enum InteractionMode {
    Dialogue,
    #[default]
    Game,
}

#[derive(Resource)]
pub struct InteractionStack {
    wants_interaction: EnumMap<InteractionMode, bool>,
}

impl Default for InteractionStack {
    fn default() -> Self {
        Self {
            wants_interaction: enum_map! { _ => true },
        }
    }
}

impl InteractionStack {
    pub fn set_wants_interaction(&mut self, mode: InteractionMode, wants_interaction: bool) {
        self.wants_interaction[mode] = wants_interaction;
    }

    pub fn can_interact(&self, mode: InteractionMode) -> bool {
        for other_mode in InteractionMode::iter() {
            if other_mode == mode {
                return self.wants_interaction[mode];
            }
            if self.wants_interaction[other_mode] {
                return false;
            }
        }
        unreachable!()
    }
}
