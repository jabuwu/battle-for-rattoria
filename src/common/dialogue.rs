use std::{collections::VecDeque, hash::Hash, mem::take};

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    AddFixedEvent, ArticyDialogue, ArticyDialogueInstruction, ArticyDialogueKind, ArticyId,
    AssetLibrary, Depth, GameState, InteractionMode, InteractionSet, InteractionStack, Persistent,
    Transform2, DEPTH_DIALOGUE,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum DialogueSystem {
    Setup,
    Update,
    UpdateInteraction,
}

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Dialogue>()
            .add_fixed_event::<DialogueEvent>()
            .add_startup_system(dialogue_setup.in_set(DialogueSystem::Setup))
            .add_system(dialogue_update.in_set(DialogueSystem::Update))
            .add_system(
                dialogue_update_interaction
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(DialogueSystem::UpdateInteraction)
                    .before(InteractionSet),
            );
    }
}

#[derive(Resource, Default)]
pub struct Dialogue {
    action: Option<DialogueAction>,
    scripts: VecDeque<Script>,
    events: Vec<DialogueEvent>,
}

impl Dialogue {
    pub fn active(&self) -> bool {
        self.action.is_some()
    }

    pub fn clear(&mut self) {
        self.action = None;
        self.scripts.clear();
    }

    pub fn queue(&mut self, script: Script, game_state: &mut GameState) {
        if self.action.is_none() {
            let children = script.dialogue.children.clone();
            self.scripts.push_back(script);
            if self.action.is_none() {
                self.show(children, game_state);
            }
        } else {
            self.scripts.push_back(script);
        }
    }

    fn show(&mut self, ids: Vec<ArticyId>, game_state: &mut GameState) {
        if let Some(current_script) = self.scripts.get(0) {
            if ids.len() > 1 {
                let nodes = ids
                    .iter()
                    .map(|id| current_script.dialogue.nodes[id].clone())
                    .collect::<Vec<_>>();
                let mut valid = true;
                for node in &nodes {
                    if !matches!(node.kind, ArticyDialogueKind::Message { .. }) {
                        valid = false;
                    }
                }
                if valid {
                    let mut choices = vec![];
                    for node in &nodes {
                        let ArticyDialogueKind::Message { text } = &node.kind else {
                            unreachable!()
                        };
                        choices.push((text.clone(), node.children.clone()));
                    }
                    self.action = Some(DialogueAction::Choice { choices });
                } else {
                    self.action = Some(DialogueAction::Text {
                        text: "INVALID DIALOGUE".to_owned(),
                        children: vec![],
                    });
                }
            } else if let Some(id) = ids.get(0) {
                let node = &current_script.dialogue.nodes[id];
                match &node.kind {
                    ArticyDialogueKind::Message { text } => {
                        self.action = Some(DialogueAction::Text {
                            text: text.clone(),
                            children: node.children.clone(),
                        });
                    }
                    ArticyDialogueKind::Instruction { instructions } => {
                        for instruction in instructions.iter() {
                            self.events.push(DialogueEvent {
                                instruction: instruction.clone(),
                            });
                        }
                        self.show(node.children.clone(), game_state);
                    }
                    ArticyDialogueKind::Condition { variable, equals } => {
                        let value = game_state
                            .global_variables
                            .get(variable)
                            .expect(&format!("expected global variable to exist: {}", variable));
                        let child_index = if *value == *equals { 0 } else { 1 };
                        if let Some(child) = node.children.get(child_index) {
                            self.show(vec![child.clone()], game_state);
                        } else {
                            self.show(vec![], game_state);
                        }
                    }
                }
            } else {
                self.scripts.remove(0);
                if let Some(next_script) = self.scripts.get(0) {
                    self.show(next_script.dialogue.children.clone(), game_state);
                } else {
                    self.action = None;
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum DialogueAction {
    Text {
        text: String,
        children: Vec<ArticyId>,
    },
    Choice {
        choices: Vec<(String, Vec<ArticyId>)>,
    },
}

#[derive(Clone)]
pub struct DialogueEvent {
    pub instruction: ArticyDialogueInstruction,
}

#[derive(Component)]
struct DialogueBox;

#[derive(Component)]
struct DialogueText;

#[derive(Component)]
struct DialogueOption(usize);

#[derive(Clone)]
pub struct Script {
    dialogue: ArticyDialogue,
}

#[derive(Clone)]
pub enum ScriptAction {
    Text {
        text: String,
        next: Option<ArticyId>,
    },
    Choice {
        choices: Vec<(Option<ArticyId>, String)>,
    },
}

impl Script {
    pub fn new(dialogue: ArticyDialogue) -> Self {
        Self { dialogue }
    }
}

fn dialogue_setup(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(2560., 500.)),
                    color: Color::rgba(0., 0., 0., 0.9),
                    ..Default::default()
                },
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            Transform2::from_xy(0., -520.),
            Depth::from(DEPTH_DIALOGUE),
            Persistent,
            DialogueBox,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: asset_library.font_placeholder.clone(),
                            font_size: 44.,
                            color: Color::WHITE,
                        },
                    )
                    .with_alignment(TextAlignment::Left),
                    text_anchor: Anchor::TopLeft,
                    ..Default::default()
                },
                Transform2::from_xy(-1240., 220.),
                Depth::Inherit(0.01),
                DialogueText,
            ));
            for y in 0..4 {
                parent.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            "",
                            TextStyle {
                                font: asset_library.font_placeholder.clone(),
                                font_size: 44.,
                                color: Color::GRAY,
                            },
                        )
                        .with_alignment(TextAlignment::Left),
                        text_anchor: Anchor::TopLeft,
                        ..Default::default()
                    },
                    Transform2::from_xy(-1200., 120. + y as f32 * -80.),
                    Depth::Inherit(0.01),
                    DialogueOption(y),
                ));
            }
        });
}

fn dialogue_update(
    mut dialogue: ResMut<Dialogue>,
    mut dialogue_box_query: Query<Entity, With<DialogueBox>>,
    mut visibility_query: Query<&mut Visibility>,
    mut text_query: Query<&mut Text>,
    mut dialogue_events: EventWriter<DialogueEvent>,
    mut interaction_stack: ResMut<InteractionStack>,
    mut game_state: ResMut<GameState>,
    dialogue_text_query: Query<Entity, With<DialogueText>>,
    dialogue_option_query: Query<(Entity, &DialogueOption)>,
    keys: Res<Input<KeyCode>>,
) {
    for event in take(&mut dialogue.events) {
        dialogue_events.send(event);
    }
    for dialogue_box_entity in dialogue_box_query.iter_mut() {
        if let Ok(mut dialogue_box_visibility) = visibility_query.get_mut(dialogue_box_entity) {
            *dialogue_box_visibility = if dialogue.action.is_some() {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
    if let Some(dialogue_action) = &dialogue.action.clone() {
        interaction_stack.set_wants_interaction(InteractionMode::Dialogue, true);
        for dialogue_entity in dialogue_text_query.iter() {
            if let Ok(mut dialogue_text) = text_query.get_mut(dialogue_entity) {
                if let Some(section) = dialogue_text.sections.get_mut(0) {
                    section.value = match dialogue_action {
                        DialogueAction::Text { text, .. } => text.clone(),
                        _ => "".to_owned(),
                    };
                }
            }
        }
        for (dialogue_option_entity, dialogue_option) in dialogue_option_query.iter() {
            let choice = match dialogue_action {
                DialogueAction::Choice { choices } => choices.get(dialogue_option.0),
                _ => None,
            };
            if let Ok(mut dialogue_option_visibility) =
                visibility_query.get_mut(dialogue_option_entity)
            {
                *dialogue_option_visibility = if choice.is_some() {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
            if let Some(choice) = choice {
                if let Ok(mut dialogue_option_text) = text_query.get_mut(dialogue_option_entity) {
                    if let Some(section) = dialogue_option_text.sections.get_mut(0) {
                        section.value = format!("{}) {}", dialogue_option.0 + 1, choice.0);
                    }
                }
            }
        }
        match dialogue_action {
            DialogueAction::Text { children, .. } => {
                if keys.just_pressed(KeyCode::Space) {
                    let children = children.clone();
                    dialogue.show(children, game_state.as_mut());
                }
            }
            DialogueAction::Choice { choices } => {
                let dialogue_keys = &[KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4];
                for (index, dialogue_key) in dialogue_keys.iter().enumerate() {
                    if let Some(dialogue_line_choice) = choices.get(index) {
                        if keys.just_pressed(*dialogue_key) {
                            dialogue.show(dialogue_line_choice.1.clone(), game_state.as_mut());
                            break;
                        }
                    }
                }
            }
        }
    } else {
        interaction_stack.set_wants_interaction(InteractionMode::Dialogue, false);
    }
}

fn dialogue_update_interaction(
    mut interaction_stack: ResMut<InteractionStack>,
    dialogue: Res<Dialogue>,
) {
    interaction_stack.set_wants_interaction(InteractionMode::Dialogue, dialogue.action.is_some());
}
