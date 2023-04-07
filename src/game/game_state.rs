use std::{collections::HashMap, mem::replace};

use bevy::prelude::*;

use crate::{Intel, Inventory, Item, Quest, UnitComposition};

#[derive(Resource, Clone)]
pub struct GameState {
    pub food: usize,
    pub available_army: UnitComposition,
    pub fed_army: UnitComposition,
    pub quest: Quest,
    pub intel: Intel,
    pub global_variables: HashMap<String, bool>,
    pub inventory: Inventory,
    pub consumed_items: Vec<Item>,
    checkpoint: Option<Box<GameState>>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            food: 0,
            available_army: UnitComposition {
                peasants: 0,
                warriors: 0,
                archers: 0,
                mages: 0,
                brutes: 0,
            },
            fed_army: UnitComposition::empty(),
            quest: Quest::default(),
            intel: Intel::default(),
            global_variables: HashMap::new(),
            inventory: Inventory::default(),
            consumed_items: vec![],
            checkpoint: None,
        }
    }
}

impl GameState {
    pub fn get_and_reset_fed_army(&mut self) -> UnitComposition {
        let fed_army = replace(&mut self.fed_army, UnitComposition::empty());
        self.available_army.add_units(&fed_army);
        fed_army
    }

    pub fn checkpoint(&mut self) {
        self.checkpoint = Some(Box::new(self.clone()));
    }

    pub fn load_checkpoint(&mut self) {
        if let Some(checkpoint) = self.checkpoint.take() {
            *self = *checkpoint;
        }
    }

    pub fn can_rewind(&self) -> bool {
        self.checkpoint
            .as_ref()
            .map(|checkpoint| checkpoint.checkpoint.is_some())
            .unwrap_or(false)
    }

    pub fn rewind(&mut self) {
        if let Some(mut checkpoint) = self.checkpoint.take() {
            if let Some(checkpoint2) = checkpoint.checkpoint.take() {
                *self = *checkpoint2;
            }
        }
    }
}
