use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use anyhow::{bail, Context, Result};
use serde_json::Value;

#[derive(Debug)]
pub struct Articy {
    pub dialogues: HashMap<String, Dialogue>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ArticyId(u64);

impl From<String> for ArticyId {
    fn from(value: String) -> Self {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        Self(hasher.finish())
    }
}

#[derive(Debug)]
pub struct Dialogue {
    pub nodes: HashMap<ArticyId, DialogueNode>,
    pub children: Vec<ArticyId>,
}

#[derive(Debug)]
pub struct DialogueNode {
    pub kind: DialogueKind,
    pub children: Vec<ArticyId>,
}

#[derive(Debug)]
pub enum DialogueKind {
    Message { text: String },
    Instruction,
}

impl Articy {
    pub fn load(contents: &str) -> Result<Articy> {
        let root: json::Root = serde_json::from_str(contents)?;
        let mut articy = Articy {
            dialogues: HashMap::new(),
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
                        serde_json::from_value(Value::Object(model.properties.clone()))?;
                    dialogues.push(dialogue);
                } else {
                    let properties_id = serde_json::from_value::<json::PropertiesId>(
                        Value::Object(model.properties.clone()),
                    )?;
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
        Ok(articy)
    }
}

fn parse_dialogue(
    dialogue: json::Dialogue,
    models: &HashMap<ArticyId, json::Model>,
) -> Result<Dialogue> {
    let first_input_pin = dialogue
        .input_pins
        .get(0)
        .context("expected an input pin")?;
    let mut dialogue = Dialogue {
        nodes: HashMap::new(),
        children: vec![],
    };
    for connection in &first_input_pin.connections {
        dialogue
            .children
            .push(ArticyId::from(connection.target.clone()));
        parse_dialogue_nodes(
            &mut dialogue,
            ArticyId::from(connection.target.clone()),
            models,
        )?;
    }
    Ok(dialogue)
}

fn parse_dialogue_nodes(
    dialogue: &mut Dialogue,
    id: ArticyId,
    models: &HashMap<ArticyId, json::Model>,
) -> Result<()> {
    if !dialogue.nodes.contains_key(&id) {
        let model = models.get(&id).context("expected model to exist")?;
        let dialogue_kind = match model.model_type.as_str() {
            "DialogueFragment" => {
                let dialogue_fragment = serde_json::from_value::<json::DialogueFragment>(
                    Value::Object(model.properties.clone()),
                )?;
                DialogueKind::Message {
                    text: dialogue_fragment.text.clone(),
                }
            }
            "Instruction" => {
                let _instruction = serde_json::from_value::<json::Instruction>(Value::Object(
                    model.properties.clone(),
                ))?;
                DialogueKind::Instruction
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
        let first_output_pin = properties_output_pins
            .output_pins
            .get(0)
            .context("expected an output pin on node")?;
        let mut children = vec![];
        if let Some(connections) = first_output_pin.connections.as_ref() {
            for connection in connections {
                children.push(ArticyId::from(connection.target.clone()));
                parse_dialogue_nodes(dialogue, ArticyId::from(connection.target.clone()), models)?;
            }
        }
        dialogue.nodes.insert(
            ArticyId::from(properties_id.id.clone()),
            DialogueNode {
                kind: dialogue_kind,
                children,
            },
        );
    }
    Ok(())
}

mod json {
    use serde_json::Map;

    use serde::Deserialize;
    use serde_json::Value;

    #[derive(Deserialize)]
    pub struct Root {
        #[serde(rename = "Packages")]
        pub packages: Vec<Package>,
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
        pub output_pins: Vec<OutputPin>,
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
        #[serde(rename = "Text")]
        pub text: String,
    }

    #[derive(Deserialize)]
    pub struct Instruction {
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Text")]
        pub text: String,
    }

    #[derive(Deserialize)]
    pub struct InputPin {
        #[serde(rename = "Connections")]
        pub connections: Vec<Connection>,
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
}
