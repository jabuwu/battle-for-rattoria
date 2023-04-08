use crate::{Articy, Item, Quest, Script};

impl Quest {
    pub fn next(&mut self) {
        self.battle += 1;
        if self.war_chef == 0 && self.battle == 3 {
            self.war_chef = 1;
            self.battle = 0;
        } else if self.war_chef == 1 && self.battle == 3 {
            self.war_chef = 2;
            self.battle = 0;
        }
    }

    pub fn preplanning_script(&self, articy: &Articy) -> Option<Script> {
        match self.war_chef {
            0 => match self.battle {
                0 => Some("WC1B1"),
                1 => Some("WC1B2"),
                2 => Some("WC1B3"),
                _ => None,
            },
            1 => match self.battle {
                0 => Some("WC2B1"),
                1 => Some("WC2B2"),
                2 => Some("WC2B3"),
                _ => None,
            },
            2 => match self.battle {
                0 => Some("WC3B1"),
                1 => Some("WC3B2"),
                2 => Some("WC3B3"),
                3 => Some("WC3B4"),
                _ => None,
            },
            _ => None,
        }
        .map(|str| Script::new(articy.dialogues[str].clone()))
    }

    pub fn item_script(&mut self, used_item: Item, articy: &Articy) -> Option<Script> {
        if !self.seen_item_dialogue[used_item] {
            self.seen_item_dialogue[used_item] = true;
            match used_item {
                Item::BogHardWeeds => {
                    if self.war_chef == 1 && self.battle == 2 {
                        None
                    } else {
                        Some("BogHardWeeds")
                    }
                }
                Item::CeleryQuartz => Some("CeleryQuartz"),
                Item::CracklingMoss => Some("CracklingMoss"),
                Item::AxeShrooms => Some("AxeShrooms"),
                Item::SquirtBlopBerries => Some("SquirtBlopBerries"),
                Item::FrostyWebStrands => Some("FrostyWebStrands"),
                _ => None,
            }
            .map(|str| Script::new(articy.dialogues[str].clone()))
        } else {
            None
        }
    }
}
