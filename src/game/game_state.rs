use std::mem::replace;

use bevy::prelude::*;

use crate::{Intel, Quest, UnitComposition};

#[derive(Resource)]
pub struct GameState {
    pub food: usize,
    pub available_army: UnitComposition,
    pub fed_army: UnitComposition,
    pub quest: Quest,
    pub intel: Intel,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            food: 0,
            available_army: UnitComposition {
                peasants: 15,
                warriors: 0,
                archers: 0,
                mages: 0,
                brutes: 0,
            },
            fed_army: UnitComposition::empty(),
            quest: Quest::default(),
            intel: Intel::default(),
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
