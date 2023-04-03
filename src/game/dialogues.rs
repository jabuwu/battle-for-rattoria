use crate::{DialogueEvent, DialogueLine, Script, UnitKind};

#[derive(Default)]
pub struct Quest {
    pub war_chef: usize,
    pub battle: usize,
}

impl Quest {
    pub fn preplanning_script(&self) -> Option<Script> {
        match self.war_chef {
            0 => match self.battle {
                0 => Some(wc1_preplanning1()),
                1 => Some(wc1_preplanning2()),
                2 => Some(wc1_preplanning3()),
                3 => Some(wc1_preplanning4()),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Hash)]
pub struct Noop;

pub fn wc1_preplanning1() -> Script {
    Script::new(vec![
        DialogueLine::message(
            "My Lord! Our scouts have returned. Mount Ratarat lies just north.",
        ),
        DialogueLine::message(
            "But take heed, my Lord! Scouts warn us that a host of moblings bearing the crest of spoon and knife marches on us! Glut Rattan's host, must be!",
        ),
        DialogueLine::branch(
            "",
            vec![
                (DialogueEvent::None, "We will crush them.", vec![
                    DialogueLine::message("There is no doubt, my Lord!"),
                    DialogueLine::message("For now, only moblings are in our throng, for now! But feed them well my Lord, and they will fight tooth and nail to rip and tear!"),
                    DialogueLine::branch("", vec![
                        (DialogueEvent::None, "Where's the rest of my army?!", vec![
                            DialogueLine::message("De-delayed, my Lord! Waylaid from your holdings!"),
                            DialogueLine::message("Surely, they will join us soon! Matter of time, just!"),
                            DialogueLine::message("But, your ratkins are strong! Will be even stronger with some food blazing in their bellies! Yes-yes!"),
                        ]),
                        (DialogueEvent::None, "Summon the troops!", vec![
                            DialogueLine::message("Yes, my Lord! To the feeding grounds! Battle awaits!"),
                        ]),
                    ]),
                ]),
                (DialogueEvent::None, "Rattan's horde must be exhausted from the long march. We attack, now!", vec![])
            ]
        ),
    ])
}

pub fn wc1_preplanning2() -> Script {
    Script::new(vec![
        DialogueLine::message("War Chef 1: Preplanning 2"),
        DialogueLine::message_and(
            "You gained 3 warriors",
            DialogueEvent::AddUnits(vec![(UnitKind::Warrior, 3)]),
        ),
    ])
}

pub fn wc1_preplanning3() -> Script {
    Script::new(vec![DialogueLine::message("War Chef 1: Preplanning 3")])
}

pub fn wc1_preplanning4() -> Script {
    Script::new(vec![DialogueLine::message("War Chef 1: Preplanning 4")])
}
