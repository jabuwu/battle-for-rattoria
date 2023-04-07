use std::{collections::VecDeque, hash::Hash, mem::take};

use bevy::{prelude::*, sprite::Anchor};
use bevy_spine::{prelude::*, rusty_spine::Skin};

use crate::{
    AddFixedEvent, ArticyDialogue, ArticyDialogueInstruction, ArticyDialogueKind, ArticyId,
    AssetLibrary, Clickable, CollisionShape, Depth, GameState, InteractionMode, InteractionSet,
    InteractionStack, Persistent, Transform2, DEPTH_DIALOGUE,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum DialogueSystem {
    Setup,
    Update,
    SpineReady,
    SpineEvents,
    UpdateInteraction,
}

const FONT_SIZE: f32 = 42.;
const FONT_SIZE_NAME: f32 = 42.;
const FONT_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const FONT_COLOR_CHOICE: Color = Color::rgba(0.2, 0.2, 0.2, 0.97);
const CHARACTERS_PER_LINE: usize = 40;
const CHOICES_DISTANCE: f32 = 44.;
const CHOICES_PADDING: Vec2 = Vec2::new(20., 4.);
const CHOICES_GAP: f32 = 4.;

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Dialogue>()
            .add_fixed_event::<DialogueEvent>()
            .add_startup_system(dialogue_setup.in_set(DialogueSystem::Setup))
            .add_system(dialogue_update.in_set(DialogueSystem::Update))
            .add_system(
                dialogue_spine_ready
                    .in_set(DialogueSystem::SpineReady)
                    .in_set(SpineSet::OnReady),
            )
            .add_system(
                dialogue_spine_events
                    .in_set(DialogueSystem::SpineEvents)
                    .after(SpineSystem::Update),
            )
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
    chars: f32,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum Speaker {
    NoOne,
    Player,
    General,
    WarChef1,
    WarChef2,

    Mobling,
    StabbyRat,
}

impl Speaker {
    pub fn speaker_kind(&self) -> SpeakerKind {
        match self {
            Self::NoOne => SpeakerKind::Unit,
            Self::Player => SpeakerKind::Friendly,
            Self::General => SpeakerKind::Friendly,
            Self::WarChef1 => SpeakerKind::Enemy,
            Self::WarChef2 => SpeakerKind::Enemy,
            Self::Mobling => SpeakerKind::Unit,
            Self::StabbyRat => SpeakerKind::Unit,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::NoOne => "",
            Self::Player => "War Chef",
            Self::General => "General Ratso",
            Self::WarChef1 => "Glut Rattan",
            Self::WarChef2 => "Field Marshal Toothsy",
            Self::Mobling => "Mobling",
            Self::StabbyRat => "Stabby-Rat",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SpeakerKind {
    Friendly,
    Enemy,
    Unit,
}

impl Dialogue {
    pub fn active(&self) -> bool {
        self.action.is_some()
    }

    pub fn clear(&mut self) {
        self.action = None;
        self.chars = 0.;
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
                        let ArticyDialogueKind::Message { text, .. } = &node.kind else {
                            unreachable!()
                        };
                        choices.push((text.clone(), node.children.clone()));
                    }
                    self.action = Some(DialogueAction::Choice { choices });
                    self.chars = 0.;
                } else {
                    self.action = Some(DialogueAction::Text {
                        speaker: Speaker::NoOne,
                        text: "INVALID DIALOGUE".to_owned(),
                        children: vec![],
                    });
                    self.chars = 0.;
                }
            } else if let Some(id) = ids.get(0) {
                let node = &current_script.dialogue.nodes[id];
                match &node.kind {
                    ArticyDialogueKind::Message { speaker, text } => {
                        self.action = Some(DialogueAction::Text {
                            speaker: *speaker,
                            text: text.clone(),
                            children: node.children.clone(),
                        });
                        self.chars = 0.;
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
                    ArticyDialogueKind::Noop => {
                        self.show(node.children.clone(), game_state);
                    }
                }
            } else {
                self.scripts.remove(0);
                if let Some(next_script) = self.scripts.get(0) {
                    self.show(next_script.dialogue.children.clone(), game_state);
                } else {
                    self.action = None;
                    self.chars = 0.;
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum DialogueAction {
    Text {
        speaker: Speaker,
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
struct DialogueRoot;

#[derive(Component)]
struct DialogueSpine {
    pub visible_in: bool,
    pub transitioning: bool,
    pub portrait_current: Speaker,
}

#[derive(Component)]
struct DialogueText {
    pub speaker_kind: SpeakerKind,
}

#[derive(Component)]
struct DialogueNameText;

#[derive(Component)]
struct DialogueOptionText(usize);

#[derive(Component)]
struct DialogueOptionTextBg;

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
            VisibilityBundle::default(),
            TransformBundle::default(),
            Transform2::from_xy(0., -485.),
            Depth::from(DEPTH_DIALOGUE),
            Persistent,
            DialogueRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                SpineBundle {
                    skeleton: asset_library.spine_dialogue.clone(),
                    ..Default::default()
                },
                Transform2::default().with_scale(Vec2::splat(1.4)),
                Depth::Inherit(0.01),
                Persistent,
                DialogueSpine {
                    visible_in: false,
                    transitioning: false,
                    portrait_current: Speaker::NoOne,
                },
                SpineSync,
            ));
        });
}

fn dialogue_spine_ready(
    mut spine_query: Query<&mut Spine, With<DialogueSpine>>,
    mut spine_ready_events: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for spine_ready_event in spine_ready_events.iter() {
        if let Ok(mut spine) = spine_query.get_mut(spine_ready_event.entity) {
            let _ = spine
                .animation_state
                .set_animation_by_name(0, "init", false);
            if let Some(mut friendly_text_entity) = spine_ready_event
                .bones
                .get("friendly_text")
                .and_then(|entity| commands.get_entity(*entity))
            {
                friendly_text_entity.with_children(|parent| {
                    parent.spawn((
                        Text2dBundle {
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    font: asset_library.font_placeholder.clone(),
                                    font_size: FONT_SIZE,
                                    color: FONT_COLOR,
                                },
                            )
                            .with_alignment(TextAlignment::Left),
                            text_anchor: Anchor::TopLeft,
                            ..Default::default()
                        },
                        Transform2::default(),
                        Depth::Inherit(0.02),
                        DialogueText {
                            speaker_kind: SpeakerKind::Friendly,
                        },
                    ));
                    for y in 0..4 {
                        parent
                            .spawn((
                                Text2dBundle {
                                    text: Text::from_section(
                                        "",
                                        TextStyle {
                                            font: asset_library.font_placeholder.clone(),
                                            font_size: FONT_SIZE,
                                            color: FONT_COLOR_CHOICE,
                                        },
                                    )
                                    .with_alignment(TextAlignment::Left),
                                    text_anchor: Anchor::TopLeft,
                                    ..Default::default()
                                },
                                Transform2::from_xy(0., 0. + y as f32 * -50.),
                                Depth::Inherit(0.03),
                                DialogueOptionText(y),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    SpriteBundle {
                                        sprite: Sprite {
                                            color: Color::WHITE,
                                            custom_size: Some(Vec2::splat(1.)),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    Transform2::default(),
                                    Depth::Inherit(-0.01),
                                    DialogueOptionTextBg,
                                    Clickable {
                                        shape: CollisionShape::Rect {
                                            size: Vec2::splat(1.),
                                            offset: Vec2::ZERO,
                                        },
                                        ..Default::default()
                                    },
                                ));
                            });
                    }
                });
            }
            if let Some(mut enemy_text_entity) = spine_ready_event
                .bones
                .get("enemy_text")
                .and_then(|entity| commands.get_entity(*entity))
            {
                enemy_text_entity.with_children(|parent| {
                    parent.spawn((
                        Text2dBundle {
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    font: asset_library.font_placeholder.clone(),
                                    font_size: FONT_SIZE,
                                    color: FONT_COLOR,
                                },
                            )
                            .with_alignment(TextAlignment::Left),
                            text_anchor: Anchor::TopLeft,
                            ..Default::default()
                        },
                        Transform2::default(),
                        Depth::Inherit(0.02),
                        DialogueText {
                            speaker_kind: SpeakerKind::Enemy,
                        },
                    ));
                    parent.spawn((
                        Text2dBundle {
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    font: asset_library.font_placeholder.clone(),
                                    font_size: FONT_SIZE,
                                    color: FONT_COLOR,
                                },
                            )
                            .with_alignment(TextAlignment::Left),
                            text_anchor: Anchor::TopLeft,
                            ..Default::default()
                        },
                        Transform2::default(),
                        Depth::Inherit(0.02),
                        DialogueText {
                            speaker_kind: SpeakerKind::Unit,
                        },
                    ));
                });
            }
            if let Some(mut name_entity) = spine_ready_event
                .bones
                .get("name")
                .and_then(|entity| commands.get_entity(*entity))
            {
                name_entity.with_children(|parent| {
                    parent.spawn((
                        Text2dBundle {
                            text: Text::from_section(
                                "War Chef",
                                TextStyle {
                                    font: asset_library.font_placeholder.clone(),
                                    font_size: FONT_SIZE_NAME,
                                    color: FONT_COLOR,
                                },
                            )
                            .with_alignment(TextAlignment::Center),
                            text_anchor: Anchor::Center,
                            ..Default::default()
                        },
                        Depth::Inherit(0.02),
                        DialogueNameText,
                    ));
                });
            }
        }
    }
}

fn dialogue_spine_events(
    mut spine_events: EventReader<SpineEvent>,
    mut spine_query: Query<&mut DialogueSpine>,
) {
    for spine_event in spine_events.iter() {
        match spine_event {
            SpineEvent::Event { entity, name, .. } => {
                if let Ok(mut dialogue_spine) = spine_query.get_mut(*entity) {
                    match name.as_str() {
                        "appeared" | "disappeared" => dialogue_spine.transitioning = false,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

fn dialogue_update(
    mut dialogue: ResMut<Dialogue>,
    mut dialogue_root_query: Query<Entity, With<DialogueRoot>>,
    mut dialogue_spine_query: Query<(&mut Spine, &mut DialogueSpine)>,
    mut visibility_query: Query<&mut Visibility>,
    mut text_query: Query<&mut Text>,
    mut transform_query: Query<&mut Transform2>,
    mut sprite_query: Query<&mut Sprite>,
    mut dialogue_events: EventWriter<DialogueEvent>,
    mut game_state: ResMut<GameState>,
    dialogue_text_query: Query<(Entity, &DialogueText)>,
    dialogue_name_text_query: Query<Entity, With<DialogueNameText>>,
    dialogue_option_text_query: Query<(Entity, &DialogueOptionText, &Children)>,
    dialogue_option_text_bg_query: Query<(Entity, &Clickable), With<DialogueOptionTextBg>>,
    keys: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    for event in take(&mut dialogue.events) {
        dialogue_events.send(event);
    }

    let mut transitioning = false;

    for (mut dialogue_spine, mut dialogue_spine_state) in dialogue_spine_query.iter_mut() {
        if dialogue.active() {
            if !dialogue_spine_state.visible_in {
                dialogue_spine_state.visible_in = true;
                dialogue_spine_state.transitioning = true;
                let _ =
                    dialogue_spine
                        .animation_state
                        .set_animation_by_name(0, "dialogue_in", false);
            }
        } else {
            if dialogue_spine_state.visible_in {
                dialogue_spine_state.visible_in = false;
                dialogue_spine_state.transitioning = true;
                let _ =
                    dialogue_spine
                        .animation_state
                        .set_animation_by_name(0, "dialogue_out", false);
            }
        }
        transitioning = dialogue_spine_state.transitioning;
        let active_speaker = if let Some(dialogue_action) = &dialogue.action.clone() {
            match dialogue_action {
                DialogueAction::Text { speaker, .. } => *speaker,
                DialogueAction::Choice { .. } => Speaker::Player,
            }
        } else {
            Speaker::NoOne
        };
        if active_speaker != dialogue_spine_state.portrait_current {
            let mut skin = Skin::new("dialogue_skin");
            if let Some(speaker_skin) = match active_speaker {
                Speaker::Player => Some("player"),
                Speaker::General => Some("general"),
                Speaker::WarChef1 => Some("wc1"),
                Speaker::WarChef2 => Some("wc2"),
                _ => None,
            } {
                if let Some(skin_general) = dialogue_spine.skeleton.data().find_skin(speaker_skin) {
                    skin.add_skin(skin_general.as_ref());
                }
            }
            let _ = dialogue_spine.skeleton.set_skin(&skin);
            dialogue_spine_state.portrait_current = active_speaker;
            let _ = dialogue_spine
                .animation_state
                .set_animation_by_name(1, "friendly_in", false);
            let _ = dialogue_spine
                .animation_state
                .set_animation_by_name(2, "enemy_in", false);
        }
    }

    for dialogue_root_entity in dialogue_root_query.iter_mut() {
        if let Ok(mut dialogue_root_visibility) = visibility_query.get_mut(dialogue_root_entity) {
            *dialogue_root_visibility = if dialogue.action.is_some() || transitioning {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
    if let Some(dialogue_action) = &dialogue.action.clone() {
        dialogue.chars += time.delta_seconds() * 50.;
        if matches!(dialogue_action, DialogueAction::Choice { .. }) {
            dialogue.chars += time.delta_seconds() * 90.;
        }
        for (dialogue_entity, dialogue_text) in dialogue_text_query.iter() {
            if let Ok(mut dialogue_text_text) = text_query.get_mut(dialogue_entity) {
                if let Some(section) = dialogue_text_text.sections.get_mut(0) {
                    if transitioning {
                        section.value = "".to_owned();
                    } else {
                        section.value = match dialogue_action {
                            DialogueAction::Text { speaker, text, .. } => {
                                if dialogue_text.speaker_kind == speaker.speaker_kind() {
                                    typewriter_text(text, dialogue.chars as usize)
                                } else {
                                    "".to_owned()
                                }
                            }
                            _ => "".to_owned(),
                        };
                    }
                }
            }
        }
        let mut dialogue_option_text_query_sorted =
            dialogue_option_text_query.iter().collect::<Vec<_>>();
        dialogue_option_text_query_sorted.sort_by(|a, b| a.1 .0.cmp(&b.1 .0));

        let mut choice_y = 0;
        let mut clicked_choice = None;
        for (dialogue_option_text_entity, dialogue_option_text, dialogue_option_text_children) in
            dialogue_option_text_query_sorted.into_iter()
        {
            let choice = match dialogue_action {
                DialogueAction::Choice { choices } => choices.get(dialogue_option_text.0),
                _ => None,
            };
            if let Ok(mut dialogue_option_text_visibility) =
                visibility_query.get_mut(dialogue_option_text_entity)
            {
                *dialogue_option_text_visibility = if choice.is_some() {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
            let mut hovered = false;
            let mut clicked = false;
            for dialogue_option_text_child in dialogue_option_text_children.iter() {
                if let Ok((_, dialogue_option_text_bg_clickable)) =
                    dialogue_option_text_bg_query.get(*dialogue_option_text_child)
                {
                    if dialogue_option_text_bg_clickable.hovered {
                        hovered = true;
                    }
                    if dialogue_option_text_bg_clickable.clicked {
                        clicked = true;
                    }
                    if dialogue_option_text_bg_clickable.confirmed {
                        clicked_choice = Some(dialogue_option_text.0);
                    }
                }
            }
            if let Ok(mut transform_query) = transform_query.get_mut(dialogue_option_text_entity) {
                transform_query.translation.y = choice_y as f32 * -CHOICES_DISTANCE
                    + (dialogue_option_text.0 as f32 * -(CHOICES_PADDING.y * 2. + CHOICES_GAP));
            }
            let mut height = 0;
            if let Some(choice) = choice {
                if let Ok(mut dialogue_option_text_text) =
                    text_query.get_mut(dialogue_option_text_entity)
                {
                    if let Some(section) = dialogue_option_text_text.sections.get_mut(0) {
                        section.value = typewriter_text(&choice.0, dialogue.chars as usize);
                        section.style.color = if hovered {
                            FONT_COLOR
                        } else {
                            FONT_COLOR_CHOICE
                        };
                        height = typewriter_text(&choice.0, 99999).split('\n').count();
                    }
                }
            }
            for dialogue_option_text_child in dialogue_option_text_children.iter() {
                if let Ok((dialogue_option_text_bg_entity, _)) =
                    dialogue_option_text_bg_query.get(*dialogue_option_text_child)
                {
                    if let Ok(mut dialogue_option_text_bg_sprite) =
                        sprite_query.get_mut(dialogue_option_text_bg_entity)
                    {
                        if clicked {
                            dialogue_option_text_bg_sprite.color = Color::rgba(1., 1., 1., 1.);
                        } else if hovered {
                            dialogue_option_text_bg_sprite.color = Color::rgba(1., 1., 1., 0.6);
                        } else {
                            dialogue_option_text_bg_sprite.color = Color::rgba(1., 1., 1., 0.1);
                        }
                    }
                    if let Ok(mut dialogue_option_text_bg_transform) =
                        transform_query.get_mut(dialogue_option_text_bg_entity)
                    {
                        dialogue_option_text_bg_transform.scale = Vec2::new(
                            850. + CHOICES_PADDING.x * 2.,
                            height as f32 * CHOICES_DISTANCE + CHOICES_PADDING.y * 2.,
                        );
                        dialogue_option_text_bg_transform.translation =
                            dialogue_option_text_bg_transform.scale * Vec2::new(0.5, -0.5)
                                + Vec2::new(-CHOICES_PADDING.x, CHOICES_PADDING.y);
                    }
                }
            }
            choice_y += height;
        }
        match dialogue_action {
            DialogueAction::Text { children, text, .. } => {
                if keys.just_pressed(KeyCode::Space)
                    || mouse_buttons.just_pressed(MouseButton::Left)
                {
                    let children = children.clone();
                    if typewriter_text(&text, dialogue.chars as usize).len()
                        == typewriter_text(&text, 99999).len()
                    {
                        dialogue.show(children, game_state.as_mut());
                    } else {
                        dialogue.chars = 99999.;
                    }
                }
            }
            DialogueAction::Choice { choices } => {
                let dialogue_keys = &[KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4];
                for (index, dialogue_key) in dialogue_keys.iter().enumerate() {
                    if let Some(dialogue_line_choice) = choices.get(index) {
                        let clicked = if let Some(clicked_choice) = clicked_choice {
                            clicked_choice == index
                        } else {
                            false
                        };
                        if clicked || keys.just_pressed(*dialogue_key) {
                            dialogue.show(dialogue_line_choice.1.clone(), game_state.as_mut());
                            break;
                        }
                    }
                }
            }
        }
        for dialogue_name_text_entity in dialogue_name_text_query.iter() {
            if let Ok(mut dialogue_name_text_entity_text) =
                text_query.get_mut(dialogue_name_text_entity)
            {
                if let Some(mut section) = dialogue_name_text_entity_text.sections.get_mut(0) {
                    let speaker = match dialogue_action {
                        DialogueAction::Text { speaker, .. } => *speaker,
                        DialogueAction::Choice { .. } => Speaker::Player,
                    };
                    section.value = speaker.name().to_owned();
                }
            }
        }
    } else {
        for (dialogue_entity, _) in dialogue_text_query.iter() {
            if let Ok(mut dialogue_text_text) = text_query.get_mut(dialogue_entity) {
                if let Some(section) = dialogue_text_text.sections.get_mut(0) {
                    section.value = "".to_owned();
                }
            }
        }
        for dialogue_name_text_entity in dialogue_name_text_query.iter() {
            if let Ok(mut dialogue_name_text_entity_text) =
                text_query.get_mut(dialogue_name_text_entity)
            {
                if let Some(mut section) = dialogue_name_text_entity_text.sections.get_mut(0) {
                    section.value = "".to_owned();
                }
            }
        }
    }
}

fn dialogue_update_interaction(
    mut interaction_stack: ResMut<InteractionStack>,
    dialogue: Res<Dialogue>,
) {
    interaction_stack.set_wants_interaction(InteractionMode::Dialogue, dialogue.action.is_some());
}

fn typewriter_text(string: &str, cap_chars: usize) -> String {
    let mut wrapped_string = String::new();
    for line in string.split('\n') {
        let mut chars = 0;
        for split in line.to_owned().split_ascii_whitespace() {
            chars += split.len();
            if chars > CHARACTERS_PER_LINE {
                wrapped_string.push('\n');
                wrapped_string += split;
                chars = split.len();
            } else {
                wrapped_string += split;
            }
            wrapped_string.push(' ');
        }
        wrapped_string.pop();
        wrapped_string.push('\n');
    }
    wrapped_string.pop();
    wrapped_string[0..cap_chars.min(wrapped_string.len())].to_owned()
}
