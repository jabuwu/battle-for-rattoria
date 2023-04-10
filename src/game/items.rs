use enum_map::Enum;
use strum_macros::EnumIter;

use crate::BattleModifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enum, EnumIter)]
pub enum Item {
    CracklingMoss,
    SquirtBlopBerries,
    FiremanderSalts,
    AxeShrooms,
    BogHardWeeds,
    CeleryQuartz,
    FrostyWebStrands,
}

impl Item {
    pub fn modifiers(&self) -> Vec<BattleModifier> {
        match self {
            Self::CracklingMoss => vec![BattleModifier::QuickAttack, BattleModifier::Cowardly],
            Self::SquirtBlopBerries => vec![BattleModifier::Wet, BattleModifier::Slowness],
            Self::FiremanderSalts => vec![BattleModifier::Fire, BattleModifier::Combustion],
            Self::AxeShrooms => vec![BattleModifier::ExtraAttack, BattleModifier::FriendlyFire],
            Self::BogHardWeeds => vec![BattleModifier::ExtraDefense, BattleModifier::Sickness],
            Self::CeleryQuartz => vec![BattleModifier::ExtraSpeed, BattleModifier::Explosive],
            Self::FrostyWebStrands => vec![BattleModifier::Ice, BattleModifier::Blindness],
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::CracklingMoss => "Crackling Moss",
            Self::SquirtBlopBerries => "Squirt Blop-Berries",
            Self::FiremanderSalts => "Firemander Salts",
            Self::AxeShrooms => "Axe Shrooms",
            Self::BogHardWeeds => "Bog Hard-Weeds",
            Self::CeleryQuartz => "Celery Quartz",
            Self::FrostyWebStrands => "Frosty Web-Strands",
        }
    }

    pub fn skin_name(&self) -> &'static str {
        match self {
            Self::CracklingMoss => "CracklingMoss",
            Self::SquirtBlopBerries => "SquirtBlopBerries",
            Self::FiremanderSalts => "FiremanderSalts",
            Self::AxeShrooms => "AxeShrooms",
            Self::BogHardWeeds => "BogHardWeeds",
            Self::CeleryQuartz => "CeleryQuartz",
            Self::FrostyWebStrands => "FrostyWebStrands",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Self::CracklingMoss => 0,
            Self::SquirtBlopBerries => 1,
            Self::FiremanderSalts => 2,
            Self::AxeShrooms => 3,
            Self::BogHardWeeds => 4,
            Self::CeleryQuartz => 5,
            Self::FrostyWebStrands => 6,
        }
    }

    pub fn positive_effect(&self) -> &'static str {
        match self {
            Self::CracklingMoss => "Attack more quickly.",
            Self::SquirtBlopBerries => "Become wet and resist fire.",
            Self::FiremanderSalts => "Fire damage burns non-armored enemies.",
            Self::AxeShrooms => "All attacks deal more damage.",
            Self::BogHardWeeds => "Increase defense.",
            Self::CeleryQuartz => "Increase movement speed.",
            Self::FrostyWebStrands => {
                "Ice damage slows enemies and is effective\nagainst wet enemies."
            }
        }
    }

    pub fn side_effect(&self) -> &'static str {
        match self {
            Self::CracklingMoss => "Units may run from battle.",
            Self::SquirtBlopBerries => "Units become slow.",
            Self::FiremanderSalts => "Units may spontaneously combust.",
            Self::AxeShrooms => "Units may attack friendlies.",
            Self::BogHardWeeds => "Some units become sick and unusable \nnext battle.",
            Self::CeleryQuartz => "Units may explode.",
            Self::FrostyWebStrands => "Units become blind and attack\nrandomly.",
        }
    }
}

#[derive(Default, Clone)]
pub struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn remove(&mut self, index: usize) {
        self.items.remove(index);
    }

    pub fn remove_last(&mut self, item: Item) {
        let mut i = self.items.len() - 1;
        loop {
            if self.items[i] == item {
                self.remove(i);
                break;
            } else if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn count(&self, item: Item) -> usize {
        self.items.iter().filter(|i| **i == item).count()
    }
}
