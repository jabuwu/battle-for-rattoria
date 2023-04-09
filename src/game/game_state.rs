use std::{collections::HashMap, mem::replace};

use bevy::prelude::*;
use enum_map::EnumMap;
use rand::prelude::*;
use strum::IntoEnumIterator;

use crate::{AssetLibrary, Intel, Inventory, Item, Quest, UnitComposition, UnitKind};

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
    pub loot: Loot,
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
            loot: Loot::default(),
            checkpoint: None,
        }
    }
}

#[derive(Default, Clone)]
pub struct Loot {
    food: isize,
    units: EnumMap<UnitKind, isize>,
    items: EnumMap<Item, isize>,
}

impl Loot {
    pub fn reset(&mut self) {
        self.food = 0;
        self.units = EnumMap::default();
        self.items = EnumMap::default();
    }

    pub fn add_food(&mut self, food: isize) {
        self.food += food;
    }

    pub fn add_items(&mut self, item: Item, count: isize) {
        self.items[item] += count;
    }

    pub fn add_units(&mut self, unit: UnitKind, count: isize) {
        self.units[unit] += count;
    }

    pub fn summary(&self, asset_library: &AssetLibrary) -> Vec<TextSection> {
        let mut loot_entries = vec![];
        if self.food != 0 {
            loot_entries.push(("Food", self.food));
        }
        for unit in UnitKind::iter() {
            if self.units[unit] != 0 {
                loot_entries.push((unit.name_plural(), self.units[unit]));
            }
        }
        for item in Item::iter() {
            if self.items[item] != 0 {
                loot_entries.push((item.name(), self.items[item]));
            }
        }

        if !loot_entries.is_empty() {
            let mut summary_sections = vec![TextSection {
                value: "Loot Get\n".to_owned(),
                style: TextStyle {
                    font: asset_library.font_heading.clone(),
                    font_size: 80.,
                    color: Color::WHITE,
                },
            }];
            for loot_entry in loot_entries.iter() {
                summary_sections.push(TextSection {
                    value: format!("{}: ", loot_entry.0),
                    style: TextStyle {
                        font: asset_library.font_bold.clone(),
                        font_size: 40.,
                        color: Color::WHITE,
                    },
                });
                if loot_entry.1 > 0 {
                    summary_sections.push(TextSection {
                        value: format!("+{}\n", loot_entry.1),
                        style: TextStyle {
                            font: asset_library.font_normal.clone(),
                            font_size: 40.,
                            color: Color::WHITE,
                        },
                    });
                } else {
                    summary_sections.push(TextSection {
                        value: format!("-{}\n", loot_entry.1 * -1),
                        style: TextStyle {
                            font: asset_library.font_normal.clone(),
                            font_size: 40.,
                            color: Color::WHITE,
                        },
                    });
                }
            }
            summary_sections
        } else {
            vec![]
        }
    }
}

#[derive(Resource, Clone)]
pub struct PersistentGameState {
    pub show_rewind_screen_dialogue: bool,
    pub show_tutorial: [bool; 4],
}

impl Default for PersistentGameState {
    fn default() -> Self {
        Self {
            show_rewind_screen_dialogue: true,
            show_tutorial: [true, true, true, true],
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
