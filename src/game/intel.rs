use bevy::prelude::*;
use enum_map::{enum_map, EnumMap};

use crate::UnitKind;

#[derive(Resource, Clone)]
pub struct Intel {
    pub can_see: EnumMap<UnitKind, bool>,
}

impl Default for Intel {
    fn default() -> Self {
        Self {
            can_see: enum_map! { _ => false},
        }
    }
}
