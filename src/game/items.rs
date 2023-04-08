use enum_map::Enum;

use crate::BattleModifier;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Enum)]
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

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }
}
