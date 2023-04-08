use std::{collections::HashMap, hash::Hash};

use anyhow::{bail, Context, Result};
use bevy::prelude::*;
use serde_json::Value;

use crate::{Item, Speaker, UnitKind};

pub struct ArticyPlugin;

impl Plugin for ArticyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Articy>();
    }
}

#[derive(Resource)]
pub struct Articy {
    pub dialogues: HashMap<String, ArticyDialogue>,
    pub global_variables: HashMap<String, bool>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ArticyId(String);

impl From<String> for ArticyId {
    fn from(value: String) -> Self {
        ArticyId(value)
    }
}

#[derive(Clone)]
pub struct ArticyDialogue {
    pub nodes: HashMap<ArticyId, ArticyDialogueNode>,
    pub children: Vec<ArticyId>,
}

#[derive(Clone)]
pub struct ArticyDialogueNode {
    pub kind: ArticyDialogueKind,
    pub children: Vec<ArticyId>,
}

#[derive(Clone)]
pub enum ArticyDialogueKind {
    Message {
        speaker: Speaker,
        text: String,
    },
    Instruction {
        instructions: Vec<ArticyDialogueInstruction>,
    },
    Condition {
        variable: String,
        equals: bool,
    },
    Noop,
}

#[derive(Clone)]
pub enum ArticyDialogueInstruction {
    AddUnits(UnitKind, usize),
    SubtractUnits(UnitKind, usize),
    AddFood(usize),
    SubtractFood(usize),
    AddItem(Item),
    SetGlobalVariable(String, bool),
}

impl Default for Articy {
    fn default() -> Articy {
        let root: json::Root = serde_json::from_str(include_str!("./articy.json")).unwrap();
        let mut articy = Articy {
            dialogues: HashMap::new(),
            global_variables: HashMap::new(),
        };
        for package in root.packages {
            if !package.is_default_package {
                continue;
            }
            let mut dialogues = vec![];
            let mut models = HashMap::new();
            for model in package.models {
                if model.model_type == "Dialogue" {
                    let dialogue: json::Dialogue =
                        serde_json::from_value(Value::Object(model.properties.clone())).unwrap();
                    dialogues.push(dialogue);
                } else {
                    let properties_id = serde_json::from_value::<json::PropertiesId>(
                        Value::Object(model.properties.clone()),
                    )
                    .unwrap();
                    models.insert(ArticyId::from(properties_id.id), model);
                }
            }
            for dialogue in dialogues {
                articy.dialogues.insert(
                    dialogue.technical_name.clone(),
                    parse_dialogue(dialogue, &models).unwrap(),
                );
            }
        }
        for global_variables in root.global_variables.iter() {
            if global_variables.namespace == "Game" {
                for variable in &global_variables.variables {
                    match variable.variable_type.as_str() {
                        "Boolean" => match variable.value.to_ascii_lowercase().as_str() {
                            "true" => {
                                articy
                                    .global_variables
                                    .insert(variable.variable.clone(), true);
                            }
                            "false" => {
                                articy
                                    .global_variables
                                    .insert(variable.variable.clone(), false);
                            }
                            _ => {
                                panic!("unsupported boolean variable value: {}", variable.value);
                            }
                        },
                        _ => {
                            panic!("unsupported variable type: {}", variable.variable_type);
                        }
                    }
                }
            }
        }
        articy
    }
}

fn parse_dialogue(
    dialogue: json::Dialogue,
    models: &HashMap<ArticyId, json::Model>,
) -> Result<ArticyDialogue> {
    let first_input_pin = dialogue
        .input_pins
        .get(0)
        .context("expected an input pin")?;
    let mut dialogue = ArticyDialogue {
        nodes: HashMap::new(),
        children: vec![],
    };
    if let Some(connections) = first_input_pin.connections.as_ref() {
        for connection in connections {
            dialogue
                .children
                .push(ArticyId::from(connection.target.clone()));
            parse_dialogue_nodes(
                &mut dialogue,
                ArticyId::from(connection.target.clone()),
                models,
            )?;
        }
    }
    Ok(dialogue)
}

fn parse_dialogue_nodes(
    dialogue: &mut ArticyDialogue,
    id: ArticyId,
    models: &HashMap<ArticyId, json::Model>,
) -> Result<()> {
    if !dialogue.nodes.contains_key(&id) {
        let model = models
            .get(&id)
            .context(format!("expected model to exist: {:?}", &id))?;
        let mut children = vec![];
        let dialogue_kind = match model.model_type.as_str() {
            "DialogueFragment" => {
                let dialogue_fragment = serde_json::from_value::<json::DialogueFragment>(
                    Value::Object(model.properties.clone()),
                )?;
                let speaker = if &dialogue_fragment.speaker == "0x0000000000000000" {
                    Speaker::NoOne
                } else {
                    let speaker_model = models
                        .get(&ArticyId::from(dialogue_fragment.speaker.clone()))
                        .expect(&format!(
                            "expected speaker to exist: {}",
                            dialogue_fragment.speaker
                        ));
                    let character =
                        serde_json::from_value::<json::DefaultSupportingCharacterTemplate>(
                            Value::Object(speaker_model.properties.clone()),
                        )?;
                    match character.display_name.as_str() {
                        "War Chef" => Speaker::Player,
                        "General Ratso" => Speaker::General,
                        "Glut Rattan" => Speaker::WarChef1,
                        "Field Marshal Toothsy" => Speaker::WarChef2,
                        "Rattin Hood" => Speaker::WarChef3,
                        "Archmage Ratus" => Speaker::WarChef4,
                        "Chompers the Barbarian" => Speaker::WarChef5,
                        "Mobling" => Speaker::Mobling,
                        "Stabby-Rat" => Speaker::StabbyRat,
                        "Shooty-Rat" => Speaker::ShootyRat,
                        "Scoutling" => Speaker::Scoutling,
                        "Deserter" => Speaker::Deserter,
                        "Blasty-Rat" => Speaker::BlastyRat,
                        "Narrator" => Speaker::Narrator,
                        _ => panic!("Unknown speaker: {}", character.display_name),
                    }
                };
                ArticyDialogueKind::Message {
                    speaker,
                    text: dialogue_fragment.text.clone(),
                }
            }
            "Instruction" => {
                let instruction = serde_json::from_value::<json::Instruction>(Value::Object(
                    model.properties.clone(),
                ))?;
                ArticyDialogueKind::Instruction {
                    instructions: parse_instructions(&instruction.expression),
                }
            }
            "Condition" => {
                let condition = serde_json::from_value::<json::Condition>(Value::Object(
                    model.properties.clone(),
                ))?;
                let (variable, equals) = parse_condition(&condition.expression);
                ArticyDialogueKind::Condition { variable, equals }
            }
            "Hub" => ArticyDialogueKind::Noop,
            "Jump" => {
                let jump =
                    serde_json::from_value::<json::Jump>(Value::Object(model.properties.clone()))?;
                children.push(ArticyId::from(jump.target));
                ArticyDialogueKind::Noop
            }
            model_type => {
                bail!("unknown articy type: {}", model_type);
            }
        };
        let properties_id =
            serde_json::from_value::<json::PropertiesId>(Value::Object(model.properties.clone()))?;
        let properties_output_pins = serde_json::from_value::<json::PropertiesOutputPins>(
            Value::Object(model.properties.clone()),
        )?;
        if let Some(output_pins) = properties_output_pins.output_pins.as_ref() {
            for output_pin in output_pins.iter() {
                if let Some(connections) = output_pin.connections.as_ref() {
                    for connection in connections {
                        children.push(ArticyId::from(connection.target.clone()));
                        parse_dialogue_nodes(
                            dialogue,
                            ArticyId::from(connection.target.clone()),
                            models,
                        )?;
                    }
                }
            }
        }
        dialogue.nodes.insert(
            ArticyId::from(properties_id.id.clone()),
            ArticyDialogueNode {
                kind: dialogue_kind,
                children,
            },
        );
    }
    Ok(())
}

fn parse_instructions(str: &str) -> Vec<ArticyDialogueInstruction> {
    #[derive(Debug)]
    enum Param {
        String(String),
        Integer(i32),
    }
    let mut instructions = vec![];
    for untrimmed_split in str.split(';') {
        let split = untrimmed_split.trim();
        if !split.is_empty() {
            if let Some(open_paren) = split.find("(") {
                let close_paren = split.find(")").expect("instruction expected a )");
                let function = &split[0..open_paren];
                let mut params = vec![];
                for param in split[(open_paren + 1)..close_paren].split(',') {
                    let trim_param = param.trim();
                    if trim_param.starts_with('"') && trim_param.ends_with('"') {
                        params.push(Param::String(
                            trim_param[1..trim_param.len() - 1].to_owned(),
                        ));
                    } else {
                        params.push(Param::Integer(trim_param.parse::<i32>().unwrap()));
                    }
                }
                match function {
                    "AddUnits" => match (params.get(0), params.get(1)) {
                        (Some(Param::String(unit_kind)), Some(Param::Integer(amount))) => {
                            let unit_kind = match unit_kind.as_str() {
                                "Peasant" => UnitKind::Peasant,
                                "Warrior" => UnitKind::Warrior,
                                "Archer" => UnitKind::Archer,
                                "Mage" => UnitKind::Mage,
                                "Brute" => UnitKind::Brute,
                                _ => panic!("unknown AddUnit() kind: {}", unit_kind),
                            };
                            instructions.push(ArticyDialogueInstruction::AddUnits(
                                unit_kind,
                                *amount as usize,
                            ));
                        }
                        _ => panic!(
                            "wrong parameters to articy function: {} {:?}",
                            function, params
                        ),
                    },
                    "SubtractUnits" => match (params.get(0), params.get(1)) {
                        (Some(Param::String(unit_kind)), Some(Param::Integer(amount))) => {
                            let unit_kind = match unit_kind.as_str() {
                                "Peasant" => UnitKind::Peasant,
                                "Warrior" => UnitKind::Warrior,
                                "Archer" => UnitKind::Archer,
                                "Mage" => UnitKind::Mage,
                                "Brute" => UnitKind::Brute,
                                _ => panic!("unknown AddUnit() kind: {}", unit_kind),
                            };
                            instructions.push(ArticyDialogueInstruction::SubtractUnits(
                                unit_kind,
                                *amount as usize,
                            ));
                        }
                        _ => panic!(
                            "wrong parameters to articy function: {} {:?}",
                            function, params
                        ),
                    },
                    "AddFood" => match params.get(0) {
                        Some(Param::Integer(amount)) => {
                            instructions.push(ArticyDialogueInstruction::AddFood(*amount as usize));
                        }
                        _ => panic!(
                            "wrong parameters to articy function: {} {:?}",
                            function, params
                        ),
                    },
                    "SubtractFood" => match params.get(0) {
                        Some(Param::Integer(amount)) => {
                            instructions
                                .push(ArticyDialogueInstruction::SubtractFood(*amount as usize));
                        }
                        _ => panic!(
                            "wrong parameters to articy function: {} {:?}",
                            function, params
                        ),
                    },
                    "AddItem" => match params.get(0) {
                        Some(Param::String(name)) => {
                            let item = match name.as_str() {
                                "Crackling Moss" => Item::CracklingMoss,
                                "Squirt Blop-Berries" => Item::SquirtBlopBerries,
                                "Firemander Salts" => Item::FiremanderSalts,
                                "Axe Shrooms" => Item::AxeShrooms,
                                "Bog Hard-Weeds" => Item::BogHardWeeds,
                                "Celery Quartz" => Item::CeleryQuartz,
                                "Frosty Web-Strands" => Item::FrostyWebStrands,
                                _ => panic!("unknown AddItem() name: {}", name),
                            };
                            instructions.push(ArticyDialogueInstruction::AddItem(item));
                        }
                        _ => panic!(
                            "wrong parameters to articy function: {} {:?}",
                            function, params
                        ),
                    },
                    _ => panic!("unknown articy function: {}", function),
                }
            } else if split.starts_with("Game.") {
                let period = split.find(".").expect("instruction expected an .");
                let assignment = &split[(period + 1)..];
                let mut eq_split = assignment.split("=");
                match (
                    eq_split.next().map(|x| x.trim()),
                    eq_split.next().map(|x| x.trim()),
                ) {
                    (Some(variable), Some(value)) => {
                        instructions.push(ArticyDialogueInstruction::SetGlobalVariable(
                            variable.to_owned(),
                            match value {
                                "true" => true,
                                "false" => false,
                                _ => panic!("invalid variable assignment value: {}", value),
                            },
                        ));
                    }
                    _ => panic!("invalid assignment: {}", assignment),
                }
            } else {
                panic!("invalid statement: {}", split);
            }
        }
    }
    instructions
}

fn parse_condition(str: &str) -> (String, bool) {
    if str.starts_with("Game.") {
        let period = str.find(".").expect("instruction expected an .");
        let condition = &str[(period + 1)..];
        let mut eq_split = condition.split("==");
        match (
            eq_split.next().map(|x| x.trim()),
            eq_split.next().map(|x| x.trim()),
        ) {
            (Some(variable), Some(value)) => (
                variable.to_owned(),
                match value {
                    "true" | "true;" => true,
                    "false" | "false;" => false,
                    _ => panic!("invalid condition: {}", str),
                },
            ),
            _ => panic!("invalid condition: {}", str),
        }
    } else {
        panic!("invalid condition: {}", str);
    }
}

mod json {
    use serde_json::Map;

    use serde::Deserialize;
    use serde_json::Value;

    #[derive(Deserialize)]
    pub struct Root {
        #[serde(rename = "Packages")]
        pub packages: Vec<Package>,
        #[serde(rename = "GlobalVariables")]
        pub global_variables: Vec<GlobalVariables>,
    }

    #[derive(Deserialize)]
    pub struct Package {
        #[serde(rename = "IsDefaultPackage")]
        pub is_default_package: bool,
        #[serde(rename = "Models")]
        pub models: Vec<Model>,
    }

    #[derive(Deserialize)]
    pub struct Model {
        #[serde(rename = "Type")]
        pub model_type: String,
        #[serde(rename = "Properties")]
        pub properties: Map<String, Value>,
    }

    #[derive(Deserialize)]
    pub struct PropertiesId {
        #[serde(rename = "Id")]
        pub id: String,
    }

    #[derive(Deserialize)]
    pub struct PropertiesOutputPins {
        #[serde(rename = "OutputPins")]
        pub output_pins: Option<Vec<OutputPin>>,
    }

    #[derive(Deserialize)]
    pub struct Dialogue {
        #[serde(rename = "TechnicalName")]
        pub technical_name: String,
        #[serde(rename = "InputPins")]
        pub input_pins: Vec<InputPin>,
    }

    #[derive(Deserialize)]
    pub struct DialogueFragment {
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Speaker")]
        pub speaker: String,
        #[serde(rename = "Text")]
        pub text: String,
    }

    #[derive(Deserialize)]
    pub struct Instruction {
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Text")]
        pub text: String,
        #[serde(rename = "Expression")]
        pub expression: String,
    }

    #[derive(Deserialize)]
    pub struct Condition {
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Expression")]
        pub expression: String,
    }

    #[derive(Deserialize)]
    pub struct Jump {
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Target")]
        pub target: String,
    }

    #[derive(Deserialize)]
    pub struct DefaultSupportingCharacterTemplate {
        #[serde(rename = "DisplayName")]
        pub display_name: String,
    }

    #[derive(Deserialize)]
    pub struct InputPin {
        #[serde(rename = "Connections")]
        pub connections: Option<Vec<Connection>>,
    }

    #[derive(Deserialize)]
    pub struct OutputPin {
        #[serde(rename = "Connections")]
        pub connections: Option<Vec<Connection>>,
    }

    #[derive(Deserialize)]
    pub struct Connection {
        #[serde(rename = "Target")]
        pub target: String,
    }

    #[derive(Deserialize)]
    pub struct GlobalVariables {
        #[serde(rename = "Namespace")]
        pub namespace: String,
        #[serde(rename = "Variables")]
        pub variables: Vec<Variable>,
    }

    #[derive(Deserialize)]
    pub struct Variable {
        #[serde(rename = "Variable")]
        pub variable: String,
        #[serde(rename = "Type")]
        pub variable_type: String,
        #[serde(rename = "Value")]
        pub value: String,
    }
}
