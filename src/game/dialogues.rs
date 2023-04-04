use crate::{Articy, Quest, Script};

impl Quest {
    pub fn preplanning_script(&self, articy: &Articy) -> Option<Script> {
        match self.war_chef {
            0 => match self.battle {
                0 => Some(wc1_preplanning1(articy)),
                1 => Some(wc1_preplanning2(articy)),
                2 => Some(wc1_preplanning3(articy)),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Hash)]
pub struct Noop;

pub fn wc1_preplanning1(articy: &Articy) -> Script {
    Script::new(articy.dialogues["WC1B1"].clone())
}

pub fn wc1_preplanning2(articy: &Articy) -> Script {
    Script::new(articy.dialogues["WC1B2"].clone())
}

pub fn wc1_preplanning3(articy: &Articy) -> Script {
    Script::new(articy.dialogues["WC1B3"].clone())
}
