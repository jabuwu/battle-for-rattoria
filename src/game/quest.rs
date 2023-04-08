use enum_map::EnumMap;

use crate::{BattleModifiers, Item, UnitComposition};

#[derive(Default, Clone)]
pub struct Quest {
    pub war_chef: usize,
    pub battle: usize,
    pub seen_item_dialogue: EnumMap<Item, bool>,
}

impl Quest {
    pub fn enemy_unit_composition(&self) -> UnitComposition {
        match self.war_chef {
            0 => match self.battle {
                0 => UnitComposition {
                    peasants: 3,
                    warriors: 0,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                },
                1 => UnitComposition {
                    peasants: 4,
                    warriors: 0,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                },
                2 => UnitComposition {
                    peasants: 10,
                    warriors: 0,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                },
                _ => UnitComposition::empty(),
            },
            1 => match self.battle {
                0 => UnitComposition {
                    peasants: 2,
                    warriors: 2,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                },
                1 => UnitComposition {
                    peasants: 20,
                    warriors: 3,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                },
                2 => UnitComposition {
                    peasants: 10,
                    warriors: 2,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                },
                _ => UnitComposition::empty(),
            },
            2 => match self.battle {
                0 => UnitComposition {
                    peasants: 3,
                    warriors: 0,
                    archers: 10,
                    mages: 0,
                    brutes: 0,
                },
                1 => UnitComposition {
                    peasants: 5,
                    warriors: 1,
                    archers: 20,
                    mages: 0,
                    brutes: 0,
                },
                _ => UnitComposition::empty(),
            },
            _ => UnitComposition::empty(),
        }
    }

    pub fn enemy_modifiers(&self) -> BattleModifiers {
        BattleModifiers::default()
    }
}
