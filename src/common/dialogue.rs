use std::{collections::VecDeque, hash::Hash};

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    AddFixedEvent, AssetLibrary, Depth, HashContext, InteractionMode, InteractionSet,
    InteractionStack, Persistent, Transform2, UnitKind, DEPTH_DIALOGUE,
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
    line: Option<DialogueLine>,
    scripts: VecDeque<Script>,
}

#[derive(Clone)]
pub enum DialogueEvent {
    None,
    Context(HashContext),
    AddUnits(UnitKind, usize),
    GainIntel(UnitKind),
    Multiple(Vec<DialogueEvent>),
}

impl DialogueEvent {
    pub fn is(&self, context: impl Hash) -> bool {
        if let DialogueEvent::Context(event_context) = &self {
            *event_context == context.into()
        } else {
            false
        }
    }

    pub fn send(&self, dialogue_events: &mut EventWriter<DialogueEvent>) {
        match self.clone() {
            Self::None => {}
            Self::Multiple(vec) => {
                for event in vec {
                    event.send(dialogue_events);
                }
            }
            x => dialogue_events.send(x),
        }
    }

    pub fn and(self, other: DialogueEvent) -> DialogueEvent {
        match self {
            DialogueEvent::None => other,
            DialogueEvent::Multiple(mut vec) => {
                vec.push(other);
                DialogueEvent::Multiple(vec)
            }
            _ => DialogueEvent::Multiple(vec![self, other]),
        }
    }
}

#[derive(Component)]
struct DialogueBox;

#[derive(Component)]
struct DialogueText;

#[derive(Component)]
struct DialogueOption(usize);

impl Dialogue {
    pub fn active(&self) -> bool {
        self.line.is_some()
    }

    pub fn clear(&mut self) {
        self.line = None;
        self.scripts.clear();
    }

    pub fn queue(&mut self, script: Script) {
        self.scripts.push_back(script);
        if self.line.is_none() {
            self.next_line();
        }
    }

    fn next_line(&mut self) {
        self.line = None;
        while !self.scripts.is_empty() {
            if let Some(line) = self.scripts[0].next() {
                self.line = Some(line.clone());
                break;
            } else {
                self.scripts.pop_front();
            }
        }
    }

    fn next_line_with_choice(&mut self, choice: usize) {
        if let Some(current_script) = self.scripts.front_mut() {
            current_script.choose(choice);
        }
        self.next_line();
    }
}

#[derive(Clone)]
pub struct DialogueLine {
    message: String,
    event: Option<DialogueEvent>,
    choices: Vec<(DialogueEvent, String, Vec<DialogueLine>)>,
}

impl DialogueLine {
    pub fn message(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            event: None,
            choices: vec![],
        }
    }

    pub fn message_and(message: &str, event: DialogueEvent) -> Self {
        Self {
            message: message.to_owned(),
            event: Some(event),
            choices: vec![],
        }
    }

    pub fn branch(message: &str, choices: Vec<(DialogueEvent, &str, Vec<DialogueLine>)>) -> Self {
        Self {
            message: message.to_owned(),
            event: None,
            choices: choices
                .into_iter()
                .map(|(context, str, lines)| (context, str.to_owned(), lines))
                .collect(),
        }
    }

    pub fn branch_and(
        message: &str,
        event: DialogueEvent,
        choices: Vec<(DialogueEvent, &str, Vec<DialogueLine>)>,
    ) -> Self {
        Self {
            message: message.to_owned(),
            event: Some(event),
            choices: choices
                .into_iter()
                .map(|(context, str, lines)| (context, str.to_owned(), lines))
                .collect(),
        }
    }
}

#[derive(Clone)]
pub struct Script {
    lines: Vec<DialogueLine>,
    current_line: usize,
}

impl Script {
    pub fn new(lines: Vec<DialogueLine>) -> Self {
        Self {
            lines,
            current_line: 0,
        }
    }
}

impl Script {
    fn next(&mut self) -> Option<DialogueLine> {
        if let Some(line) = self.lines.get(self.current_line) {
            self.current_line += 1;
            Some(line.clone())
        } else {
            None
        }
    }

    fn choose(&mut self, index: usize) {
        if let Some(current_line) = self.lines.get(self.current_line - 1) {
            if let Some(choice) = current_line.choices.get(index) {
                self.lines = choice.2.clone();
                self.current_line = 0;
            }
        }
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
                        "Hello",
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
                            "diggity dawg",
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
    dialogue_text_query: Query<Entity, With<DialogueText>>,
    dialogue_option_query: Query<(Entity, &DialogueOption)>,
    keys: Res<Input<KeyCode>>,
) {
    for dialogue_box_entity in dialogue_box_query.iter_mut() {
        if let Ok(mut dialogue_box_visibility) = visibility_query.get_mut(dialogue_box_entity) {
            *dialogue_box_visibility = if dialogue.line.is_some() {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
    if let Some(dialogue_line) = &mut dialogue.line {
        if let Some(event) = dialogue_line.event.take() {
            event.send(&mut dialogue_events);
        }
        interaction_stack.set_wants_interaction(InteractionMode::Dialogue, true);
        for dialogue_entity in dialogue_text_query.iter() {
            if let Ok(mut dialogue_text) = text_query.get_mut(dialogue_entity) {
                if let Some(section) = dialogue_text.sections.get_mut(0) {
                    section.value = dialogue_line.message.to_owned();
                }
            }
        }
        for (dialogue_option_entity, dialogue_option) in dialogue_option_query.iter() {
            let choice = dialogue_line.choices.get(dialogue_option.0);
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
                        section.value = format!("{}) {}", dialogue_option.0 + 1, choice.1);
                    }
                }
            }
        }
        if dialogue_line.choices.len() == 0 {
            if keys.just_pressed(KeyCode::Space) {
                dialogue.next_line();
            }
        } else {
            let dialogue_keys = &[KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4];
            for (index, dialogue_key) in dialogue_keys.iter().enumerate() {
                if let Some(dialogue_line_choice) = dialogue_line.choices.get(index) {
                    if keys.just_pressed(*dialogue_key) {
                        dialogue_line_choice.0.send(&mut dialogue_events);
                        dialogue.next_line_with_choice(index);
                        break;
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
    interaction_stack.set_wants_interaction(InteractionMode::Dialogue, dialogue.line.is_some());
}
