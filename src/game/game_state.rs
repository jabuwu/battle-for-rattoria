use std::{collections::HashMap, mem::replace};

use bevy::prelude::*;
use rand::prelude::*;
use strum::IntoEnumIterator;

use crate::{Intel, Inventory, Item, Quest, UnitComposition, UnitKind};

#[derive(Resource, Clone)]
pub struct GameState {
    pub food: usize,
    pub available_army: UnitComposition,
    pub fed_army: UnitComposition,
    pub sick_army: UnitComposition,
    pub quest: Quest,
    pub intel: Intel,
    pub global_variables: HashMap<String, bool>,
    pub inventory: Inventory,
    pub used_items: Vec<Item>,
    pub consumed_items: Vec<Item>,
    pub checkpoint: Option<Box<GameState>>,
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
            sick_army: UnitComposition::empty(),
            quest: Quest::default(),
            intel: Intel::default(),
            global_variables: HashMap::new(),
            inventory: Inventory::default(),
            used_items: vec![],
            consumed_items: vec![],
            checkpoint: None,
        }
    }
}

#[derive(Resource, Clone)]
pub struct PersistentGameState {
    pub show_rewind_screen_dialogue: bool,
}

impl Default for PersistentGameState {
    fn default() -> Self {
        Self {
            show_rewind_screen_dialogue: true,
        }
    }
}

impl GameState {
    pub fn get_and_reset_fed_army(&mut self) -> UnitComposition {
        let fed_army = replace(&mut self.fed_army, UnitComposition::empty());
        self.available_army.add_units(&fed_army);
        fed_army
    }

    pub fn apply_sickness(&mut self, sick: bool) {
        let mut rng = thread_rng();

        // undo previous sickness
        for unit_kind in UnitKind::iter() {
            let count = self.available_army.get_count(unit_kind);
            let sick = self.sick_army.get_count(unit_kind);
            self.available_army.set_count(unit_kind, count + sick);
            self.sick_army.set_count(unit_kind, 0);
        }
        // apply new sickness
        if sick {
            for unit_kind in UnitKind::iter() {
                let count = self.available_army.get_count(unit_kind);
                let sick = rng.gen_range(0..=(count / 2)).max(2).min(count.max(1) - 1);
                self.available_army.set_count(unit_kind, count - sick);
                self.sick_army.set_count(unit_kind, sick);
            }
        }
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
