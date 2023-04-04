use std::fs::read_to_string;

use articy::{Articy, DialogueKind};

fn main() {
    let articy: Articy =
        Articy::load(&read_to_string("./First_Battle_Intro.json").unwrap()).unwrap();
    let dialogue = &articy.dialogues["Dlg_CFBBE915"];
    let mut current_node = Some(dialogue.children[0]);
    while let Some(node) = current_node {
        let dialogue_node = dialogue.nodes.get(&node).unwrap();
        match &dialogue_node.kind {
            DialogueKind::Message { text } => {
                println!("{}", text);
            }
            DialogueKind::Instruction => {
                println!("INSTRUCTION");
            }
        }
        current_node = dialogue_node.children.get(0).cloned();
    }
}
