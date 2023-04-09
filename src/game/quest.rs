use enum_map::{enum_map, EnumMap};

use crate::{BattleModifier, BattleModifiers, Item, UnitComposition};

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
                    peasants: 10,
                    warriors: 2,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                },
                2 => UnitComposition {
                    peasants: 8,
                    warriors: 1,
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
                    archers: 15,
                    mages: 0,
                    brutes: 0,
                },
                2 => UnitComposition {
                    peasants: 3,
                    warriors: 1,
                    archers: 5,
                    mages: 0,
                    brutes: 0,
                },
                3 => UnitComposition {
                    peasants: 0,
                    warriors: 0,
                    archers: 0,
                    mages: 0,
                    brutes: 1,
                },
                _ => UnitComposition::empty(),
            },
            3 => match self.battle {
                0 => UnitComposition {
                    peasants: 5,
                    warriors: 1,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                },
                1 => UnitComposition {
                    peasants: 8,
                    warriors: 2,
                    archers: 0,
                    mages: 1,
                    brutes: 0,
                },
                2 => UnitComposition {
                    peasants: 10,
                    warriors: 1,
                    archers: 0,
                    mages: 1,
                    brutes: 0,
                },
                3 => UnitComposition {
                    peasants: 10,
                    warriors: 0,
                    archers: 0,
                    mages: 2,
                    brutes: 0,
                },
                _ => UnitComposition::empty(),
            },
            4 => match self.battle {
                0 => UnitComposition {
                    peasants: 40,
                    warriors: 0,
                    archers: 3,
                    mages: 0,
                    brutes: 0,
                },
                1 => UnitComposition {
                    peasants: 0,
                    warriors: 6,
                    archers: 0,
                    mages: 0,
                    brutes: 1,
                },
                2 => UnitComposition {
                    peasants: 10,
                    warriors: 2,
                    archers: 2,
                    mages: 0,
                    brutes: 2,
                },
                _ => UnitComposition::empty(),
            },

            _ => UnitComposition::empty(),
        }
    }

    pub fn enemy_modifiers(&self) -> BattleModifiers {
        match self.war_chef {
            3 => match self.battle {
                2 => enum_map! {
                    BattleModifier::Fire => true,
                    BattleModifier::Combustion => true,
                    _ => false,
                },
                _ => BattleModifiers::default(),
            },
            4 => match self.battle {
                0 => enum_map! {
                    BattleModifier::Wet => true,
                    BattleModifier::Slowness => true,
                    _ => false,
                },
                _ => BattleModifiers::default(),
            },
            _ => BattleModifiers::default(),
        }
    }

    pub fn hint(&self) -> &'static str {
        match self.war_chef {
            0 => match self.battle {
                0 => "Glut Rattan's small mobling force approaches!",
                1 => "Glut Rattan's small mobling force approaches!",
                2 => "Glut Rattan's small mobling force approaches!",
                _ => "??",
            },
            1 => match self.battle {
                0 => "Toothsy's small parley force approaches!",
                1 => "Toothsy hopes to win this battle with moblings and warriors!",
                2 => "Toothsy is weak but still has plenty of fight!",
                _ => "??",
            },
            2 => match self.battle {
                0 => "Rattin Hood hopes to rain arrows from above!",
                1 => "Rattin Hood is making a stronger front line for his archers!",
                2 => "Rattin Hood is almost out of steam!",
                3 => "The ground shakes from Rattin Hood's final combatant!",
                _ => "??",
            },
            3 => match self.battle {
                0 => "Archmage Ratus sends a small force to test your army!",
                1 => "Archmage Ratus aids his warriors with the power of magic!",
                2 => "Archmage Ratus aids his warriors with the power of magic, again!",
                3 => "Archmage Ratus sends his mobling hoard guarded by two mages!",
                _ => "??",
            },
            4 => match self.battle {
                0 => "Chompers the Barbarian sends his unstoppable (and soaked) mobling hoard!",
                1 => "Chompers the Barbarian sends his finest warriors and one Bigg-Rat!",
                2 => "The final battle!",
                _ => "??",
            },
            _ => "??",
        }
    }
}
