use std::mem::replace;

use bevy::prelude::*;

use crate::UnitComposition;

#[derive(Resource)]
pub struct GameState {
    pub food: usize,
    pub available_army: UnitComposition,
    pub fed_army: UnitComposition,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            food: 30,
            available_army: UnitComposition {
                peasants: 100,
                warriors: 20,
            },
            fed_army: UnitComposition {
                peasants: 0,
                warriors: 0,
            },
        }
    }
}

impl GameState {
    pub fn get_and_reset_fed_army(&mut self) -> UnitComposition {
        let fed_army = replace(&mut self.fed_army, UnitComposition::empty());
        self.available_army.add_units(&fed_army);
        fed_army
    }
}
