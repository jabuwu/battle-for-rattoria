use bevy::{prelude::*, sprite::Anchor};
use bevy_egui::{egui, EguiContexts};
use bevy_spine::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    typewriter_text, AddFixedEvent, Articy, AssetLibrary, Clickable, ClickableSystem,
    CollisionShape, Depth, Dialogue, GameState, InteractionMode, InteractionSet, InteractionStack,
    Item, PersistentGameState, Script, Sfx, SfxKind, SpawnSet, Transform2, UnitKind, UpdateSet,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum PlanningSystem {
    Start,
    Update,
    SpineReady,
    UpdateButtonsAndInfo,
    UpdateUnitCountText,
    UpdateFoodCountText,
    UpdateItems,
    StartBattle,
    Ui,
}

pub struct PlanningPlugin;

impl Plugin for PlanningPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlanningState>()
            .add_fixed_event::<PlanningStartEvent>()
            .add_fixed_event::<PlanningEndedEvent>()
            .add_system(
                planning_start
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(PlanningSystem::Start)
                    .in_set(SpawnSet),
            )
            .add_system(
                planning_update
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(PlanningSystem::Update)
                    .in_set(UpdateSet),
            )
            .add_system(
                planning_spine_ready
                    .in_set(PlanningSystem::SpineReady)
                    .in_set(SpineSet::OnReady),
            )
            .add_system(
                planning_update_buttons_and_info
                    .in_set(PlanningSystem::UpdateButtonsAndInfo)
                    .after(ClickableSystem)
                    .after(InteractionSet),
            )
            .add_system(planning_update_unit_count_text.in_set(PlanningSystem::UpdateUnitCountText))
            .add_system(planning_update_food_count_text.in_set(PlanningSystem::UpdateFoodCountText))
            .add_system(planning_update_items.in_set(PlanningSystem::UpdateItems))
            .add_system(planning_start_battle.in_set(PlanningSystem::StartBattle))
            .add_system(planning_ui.in_set(PlanningSystem::Ui));
    }
}

#[derive(Resource)]
pub struct PlanningState {
    planning: bool,
    start: bool,
    skip: bool,
    rewind: bool,
}

impl PlanningState {
    pub fn stop(&mut self) {
        self.planning = false;
    }
}

impl Default for PlanningState {
    fn default() -> Self {
        Self {
            planning: false,
            start: false,
            skip: false,
            rewind: false,
        }
    }
}

#[derive(Default)]
pub struct PlanningStartEvent;

#[derive(Default)]
pub struct PlanningEndedEvent {
    pub skip: bool,
    pub rewind: bool,
    _private: (),
}

#[derive(Component)]
struct PlanningSpine;

#[derive(Component)]
struct PlanningButton {
    pub image_index: usize,
    pub kind: PlanningButtonKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PlanningButtonKind {
    Item(Item),
    Unit(UnitKind),
    Noop,
}

#[derive(Component)]
struct PlanningInfoText;

#[derive(Component)]
struct PlanningUnitCountText(UnitKind);

#[derive(Component)]
struct PlanningItemCountText;

#[derive(Component)]
struct PlanningFoodCountText;

#[derive(Component)]
struct PlanningStartBattle;

#[derive(Component)]
struct PlanningItem(usize);

#[derive(Component)]
struct PlanningHint;

fn planning_start(
    mut start_events: EventReader<PlanningStartEvent>,
    mut planning_state: ResMut<PlanningState>,
    mut dialogue: ResMut<Dialogue>,
    mut persistent_game_state: ResMut<PersistentGameState>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    articy: Res<Articy>,
    asset_library: Res<AssetLibrary>,
) {
    for _ in start_events.iter() {
        planning_state.planning = true;
        if let Some(tutorial_index) = match game_state.quest.war_chef {
            0 => match game_state.quest.battle {
                0 => Some(0),
                1 => Some(1),
                2 => Some(2),
                _ => None,
            },
            1 => match game_state.quest.battle {
                0 => Some(3),
                _ => None,
            },
            _ => None,
        } {
            let tutorials = &["Tutorial1", "Tutorial2", "Tutorial3", "Tutorial4"];
            if persistent_game_state.show_tutorial[tutorial_index] {
                persistent_game_state.show_tutorial[tutorial_index] = false;
                dialogue.queue(
                    Script::new(
                        articy
                            .dialogues
                            .get(tutorials[tutorial_index])
                            .unwrap()
                            .clone(),
                    ),
                    game_state.as_mut(),
                );
            }
        }
        commands.spawn((
            SpriteBundle {
                texture: asset_library.image_planning_bg.clone(),
                ..Default::default()
            },
            Transform2::default(),
            Depth::Exact(0.0),
        ));
        commands.spawn((
            SpineBundle {
                skeleton: asset_library.spine_planning.clone(),
                ..Default::default()
            },
            Transform2::default(),
            Depth::Exact(0.01),
            PlanningSpine,
            SpineSync,
        ));
    }
}

fn planning_spine_ready(
    mut spine_ready_events: EventReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine, With<PlanningSpine>>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for spine_ready_event in spine_ready_events.iter() {
        if let Ok(mut spine) = spine_query.get_mut(spine_ready_event.entity) {
            let _ = spine
                .animation_state
                .set_animation_by_name(0, "cauldron", true);
            for slot_index in 0..7 {
                if let Some(spice) = spine_ready_event
                    .bones
                    .get(&format!("spice{}", slot_index + 1))
                {
                    if let Some(mut spice_entity) = commands.get_entity(*spice) {
                        spice_entity.with_children(|parent| {
                            parent
                                .spawn((
                                    SpriteSheetBundle {
                                        sprite: TextureAtlasSprite {
                                            index: 0,
                                            ..Default::default()
                                        },
                                        texture_atlas: asset_library
                                            .image_atlas_planning_buttons
                                            .clone(),
                                        visibility: Visibility::Hidden,
                                        ..Default::default()
                                    },
                                    Transform2::default(),
                                    Depth::Exact(0.02),
                                    Clickable {
                                        shape: CollisionShape::Rect {
                                            size: Vec2::splat(200.),
                                            offset: Vec2::ZERO,
                                        },
                                        ..Default::default()
                                    },
                                    PlanningButton {
                                        image_index: 0,
                                        kind: PlanningButtonKind::Noop,
                                    },
                                    PlanningItem(slot_index),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text2dBundle {
                                            text: Text::from_section(
                                                "x0",
                                                TextStyle {
                                                    font: asset_library.font_bold.clone(),
                                                    font_size: 62.,
                                                    color: Color::WHITE,
                                                },
                                            )
                                            .with_alignment(TextAlignment::Right),
                                            text_anchor: Anchor::TopRight,
                                            ..Default::default()
                                        },
                                        Transform2::from_xy(85., -35.),
                                        Depth::Inherit(0.002),
                                        PlanningItemCountText,
                                    ));
                                    parent.spawn((
                                        Text2dBundle {
                                            text: Text::from_section(
                                                "",
                                                TextStyle {
                                                    font: asset_library.font_bold.clone(),
                                                    font_size: 62.,
                                                    color: Color::BLACK,
                                                },
                                            )
                                            .with_alignment(TextAlignment::Right),
                                            text_anchor: Anchor::TopRight,
                                            ..Default::default()
                                        },
                                        Transform2::from_xy(89., -39.),
                                        Depth::Inherit(0.001),
                                        PlanningItemCountText,
                                    ));
                                });
                        });
                    }
                }
            }
            for (unit_kind_index, unit_kind) in UnitKind::iter().enumerate() {
                if let Some(spice) = spine_ready_event
                    .bones
                    .get(&format!("unit{}", unit_kind_index + 1))
                {
                    let sprite_index = 21 + unit_kind_index;
                    if let Some(mut spice_entity) = commands.get_entity(*spice) {
                        spice_entity.with_children(|parent| {
                            parent
                                .spawn((
                                    SpriteSheetBundle {
                                        sprite: TextureAtlasSprite {
                                            index: sprite_index,
                                            ..Default::default()
                                        },
                                        texture_atlas: asset_library
                                            .image_atlas_planning_buttons
                                            .clone(),
                                        ..Default::default()
                                    },
                                    Transform2::default(),
                                    Depth::Exact(0.02),
                                    Clickable {
                                        shape: CollisionShape::Rect {
                                            size: Vec2::splat(200.),
                                            offset: Vec2::ZERO,
                                        },
                                        ..Default::default()
                                    },
                                    PlanningButton {
                                        image_index: sprite_index,
                                        kind: PlanningButtonKind::Unit(unit_kind),
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text2dBundle {
                                            text: Text::from_section(
                                                "x0",
                                                TextStyle {
                                                    font: asset_library.font_bold.clone(),
                                                    font_size: 62.,
                                                    color: Color::WHITE,
                                                },
                                            )
                                            .with_alignment(TextAlignment::Right),
                                            text_anchor: Anchor::TopRight,
                                            ..Default::default()
                                        },
                                        Transform2::from_xy(85., -35.),
                                        Depth::Inherit(0.002),
                                        PlanningUnitCountText(unit_kind),
                                    ));
                                    parent.spawn((
                                        Text2dBundle {
                                            text: Text::from_section(
                                                "",
                                                TextStyle {
                                                    font: asset_library.font_bold.clone(),
                                                    font_size: 62.,
                                                    color: Color::BLACK,
                                                },
                                            )
                                            .with_alignment(TextAlignment::Right),
                                            text_anchor: Anchor::TopRight,
                                            ..Default::default()
                                        },
                                        Transform2::from_xy(89., -39.),
                                        Depth::Inherit(0.001),
                                        PlanningUnitCountText(unit_kind),
                                    ));
                                });
                        });
                    }
                }
            }
            if let Some(food_text_entity) = spine_ready_event.bones.get("food_text") {
                if let Some(mut food_text_entity) = commands.get_entity(*food_text_entity) {
                    food_text_entity.with_children(|parent| {
                        parent.spawn((
                            Text2dBundle {
                                text: Text::from_section(
                                    "",
                                    TextStyle {
                                        font: asset_library.font_bold.clone(),
                                        font_size: 82.,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_alignment(TextAlignment::Center),
                                text_anchor: Anchor::Center,
                                ..Default::default()
                            },
                            Transform2::from_xy(0., 0.),
                            Depth::Inherit(0.001),
                            PlanningFoodCountText,
                        ));
                    });
                }
            }
            if let Some(info_box_text_entity) = spine_ready_event.bones.get("info_box_text") {
                if let Some(mut info_box_text_entity) = commands.get_entity(*info_box_text_entity) {
                    info_box_text_entity.with_children(|parent| {
                        parent.spawn((
                            Text2dBundle {
                                text: Text::from_sections(vec![])
                                    .with_alignment(TextAlignment::Left),
                                text_anchor: Anchor::TopLeft,
                                ..Default::default()
                            },
                            Transform2::default(),
                            Depth::Inherit(0.001),
                            PlanningInfoText,
                        ));
                    });
                }
            }
            if let Some(start_battle_entity) = spine_ready_event.bones.get("start_battle") {
                if let Some(mut start_battle_entity) = commands.get_entity(*start_battle_entity) {
                    start_battle_entity.with_children(|parent| {
                        parent.spawn((
                            Clickable {
                                shape: CollisionShape::Rect {
                                    offset: Vec2::ZERO,
                                    size: Vec2::new(375., 150.),
                                },
                                ..Default::default()
                            },
                            TransformBundle::default(),
                            Transform2::default(),
                            PlanningStartBattle,
                        ));
                    });
                }
            }
            if let Some(hint_entity) = spine_ready_event.bones.get("hint") {
                if let Some(mut hint_entity) = commands.get_entity(*hint_entity) {
                    hint_entity.with_children(|parent| {
                        parent.spawn((
                            Clickable {
                                shape: CollisionShape::Rect {
                                    offset: Vec2::ZERO,
                                    size: Vec2::new(120., 120.),
                                },
                                ..Default::default()
                            },
                            TransformBundle::default(),
                            Transform2::default(),
                            PlanningHint,
                        ));
                    });
                }
            }
        }
    }
}

fn planning_update(
    mut planning_state: ResMut<PlanningState>,
    mut planning_ended_events: EventWriter<PlanningEndedEvent>,
) {
    if planning_state.planning
        && (planning_state.start || planning_state.skip || planning_state.rewind)
    {
        planning_ended_events.send(PlanningEndedEvent {
            skip: planning_state.skip && !planning_state.rewind,
            rewind: planning_state.rewind,
            _private: (),
        });
        planning_state.planning = false;
        planning_state.start = false;
        planning_state.skip = false;
        planning_state.rewind = false;
    }
}

fn planning_update_buttons_and_info(
    mut button_query: Query<(Entity, &mut TextureAtlasSprite, &PlanningButton)>,
    mut planning_spine_query: Query<&mut Spine, With<PlanningSpine>>,
    mut game_state: ResMut<GameState>,
    mut info_text_query: Query<&mut Text, With<PlanningInfoText>>,
    mut sfx: ResMut<Sfx>,
    clickable_query: Query<&Clickable>,
    hint_query: Query<Entity, With<PlanningHint>>,
    asset_library: Res<AssetLibrary>,
) {
    let mut info_text = None;
    let header_style = TextStyle {
        font: asset_library.font_heading.clone(),
        font_size: 92.,
        color: Color::WHITE,
    };
    let description_style = TextStyle {
        font: asset_library.font_normal.clone(),
        font_size: 52.,
        color: Color::WHITE,
    };
    let bold_style = TextStyle {
        font: asset_library.font_bold.clone(),
        font_size: 52.,
        color: Color::WHITE,
    };
    for (button_entity, mut button_sprite, button) in button_query.iter_mut() {
        if let Ok(button_clickable) = clickable_query.get(button_entity) {
            let active = match button.kind {
                PlanningButtonKind::Unit(unit) => game_state.available_army.get_count(unit) > 0,
                PlanningButtonKind::Item(..) => game_state.consumed_items.is_empty(),
                PlanningButtonKind::Noop => false,
            };
            if active {
                if button_clickable.just_clicked() {
                    sfx.play(SfxKind::UiButtonClick);
                } else if button_clickable.just_hovered() {
                    sfx.play(SfxKind::UiButtonHover);
                } else if button_clickable.just_released() {
                    sfx.play(SfxKind::UiButtonRelease);
                }
            }
            if button_clickable.clicked && active {
                button_sprite.index = button.image_index + 14;
            } else if button_clickable.hovered && active {
                button_sprite.index = button.image_index + 7;
            } else {
                button_sprite.index = button.image_index;
            }
            button_sprite.color = if active {
                Color::WHITE
            } else {
                Color::rgba(0.5, 0.5, 0.5, 0.9)
            };
            if button_clickable.hovered {
                match button.kind {
                    PlanningButtonKind::Item(item) => {
                        info_text = Some(vec![
                            TextSection {
                                value: format!("{}\n", item.name()),
                                style: header_style.clone(),
                            },
                            TextSection {
                                value: format!("Effect: "),
                                style: bold_style.clone(),
                            },
                            TextSection {
                                value: format!("{}\n", item.positive_effect()),
                                style: description_style.clone(),
                            },
                            TextSection {
                                value: format!("Side Effect: "),
                                style: bold_style.clone(),
                            },
                            TextSection {
                                value: format!("{}", item.side_effect()),
                                style: description_style.clone(),
                            },
                        ])
                    }
                    PlanningButtonKind::Unit(unit) => {
                        info_text = Some(vec![
                            TextSection {
                                value: format!("{}\n", unit.name()),
                                style: header_style.clone(),
                            },
                            TextSection {
                                value: format!("{}\n\n", unit.description()),
                                style: description_style.clone(),
                            },
                            TextSection {
                                value: format!("Food Cost: "),
                                style: bold_style.clone(),
                            },
                            TextSection {
                                value: format!("{}", unit.stats().cost),
                                style: description_style.clone(),
                            },
                        ])
                    }
                    PlanningButtonKind::Noop => {}
                }
            }
            if button_clickable.confirmed && active {
                match button.kind {
                    PlanningButtonKind::Item(item) => {
                        for mut planning_spine in planning_spine_query.iter_mut() {
                            let _ = planning_spine.skeleton.set_skin_by_name(item.skin_name());
                            let _ = planning_spine.animation_state.set_animation_by_name(
                                1,
                                "insert_spices",
                                false,
                            );
                        }
                        sfx.play(SfxKind::CauldronAddSpice);
                        game_state.consumed_items.push(item);
                        game_state.inventory.remove_last(item);
                    }
                    PlanningButtonKind::Unit(unit_kind) => {
                        let unit_cost = unit_kind.stats().cost;
                        if game_state.available_army.get_count(unit_kind) > 0
                            && game_state.food >= unit_cost
                        {
                            game_state.fed_army.mutate_count(unit_kind, |i| i + 1);
                            game_state.available_army.mutate_count(unit_kind, |i| i - 1);
                            game_state.food -= unit_cost;
                            for mut planning_spine in planning_spine_query.iter_mut() {
                                let _ = planning_spine
                                    .animation_state
                                    .set_animation_by_name(1, "food_eat", false);
                            }
                            sfx.play(SfxKind::UiFeedUnit);
                        }
                    }
                    PlanningButtonKind::Noop => {}
                }
            }
        }
    }
    for hint_entity in hint_query.iter() {
        if let Ok(hint_clickable) = clickable_query.get(hint_entity) {
            if hint_clickable.clicked {
                info_text = Some(vec![
                    TextSection {
                        value: "Battle Hint\n".to_owned(),
                        style: header_style.clone(),
                    },
                    TextSection {
                        value: typewriter_text(game_state.quest.hint(), 999, false),
                        style: description_style.clone(),
                    },
                ])
            } else if hint_clickable.hovered {
                info_text = Some(vec![
                    TextSection {
                        value: "Battle Hint\n".to_owned(),
                        style: header_style.clone(),
                    },
                    TextSection {
                        value: "Hold left mouse button to see hint".to_owned(),
                        style: description_style.clone(),
                    },
                ])
            }
        }
    }
    for mut planning_spine in planning_spine_query.iter_mut() {
        let current_animation = planning_spine
            .animation_state
            .track_at_index(2)
            .map(|track| track.animation().name().to_owned())
            .unwrap_or(String::new());
        if info_text.is_some() {
            if current_animation != "info_box_show" {
                let _ =
                    planning_spine
                        .animation_state
                        .set_animation_by_name(2, "info_box_show", false);
            }
        } else {
            if current_animation != "info_box_hide" {
                let _ =
                    planning_spine
                        .animation_state
                        .set_animation_by_name(2, "info_box_hide", false);
            }
        }
    }
    for mut info_text_text in info_text_query.iter_mut() {
        *info_text_text = Text::from_sections(info_text.take().unwrap_or(vec![]))
            .with_alignment(TextAlignment::Left);
    }
}

fn planning_update_unit_count_text(
    mut unit_count_text_query: Query<(&mut Text, &PlanningUnitCountText)>,
    game_state: Res<GameState>,
) {
    for (mut unit_count_text_text, unit_count_text) in unit_count_text_query.iter_mut() {
        if let Some(section) = unit_count_text_text.sections.get_mut(0) {
            section.value = format!(
                "x{}",
                game_state.available_army.get_count(unit_count_text.0)
            );
        }
    }
}

fn planning_update_food_count_text(
    mut unit_count_text_query: Query<&mut Text, With<PlanningFoodCountText>>,
    game_state: Res<GameState>,
) {
    for mut unit_count_text_text in unit_count_text_query.iter_mut() {
        if let Some(section) = unit_count_text_text.sections.get_mut(0) {
            section.value = format!("x{}", game_state.food);
        }
    }
}

fn planning_update_items(
    mut item_query: Query<(
        &mut PlanningButton,
        &mut Visibility,
        &PlanningItem,
        &Children,
    )>,
    mut text_query: Query<&mut Text>,
    game_state: Res<GameState>,
) {
    let mut items = vec![];
    for item in game_state.inventory.items() {
        if !items.contains(item) {
            items.push(*item);
        }
    }
    for (mut item_button, mut item_visibility, _, item_children) in item_query.iter_mut() {
        *item_visibility = Visibility::Hidden;
        item_button.kind = PlanningButtonKind::Noop;
        for child in item_children.iter() {
            if let Ok(mut text) = text_query.get_mut(*child) {
                if let Some(section) = text.sections.get_mut(0) {
                    section.value = "".to_owned();
                }
            }
        }
    }
    for (mut item_button, mut item_visibility, planning_item, item_children) in
        item_query.iter_mut()
    {
        for (slot_index, item) in items.clone().into_iter().enumerate() {
            if slot_index == planning_item.0 {
                item_button.kind = PlanningButtonKind::Item(item);
                item_button.image_index = item.index();
                *item_visibility = Visibility::Inherited;
                for child in item_children.iter() {
                    if let Ok(mut text) = text_query.get_mut(*child) {
                        if let Some(section) = text.sections.get_mut(0) {
                            section.value = format!("x{}", game_state.inventory.count(item));
                        }
                    }
                }
            }
        }
    }
}

fn planning_start_battle(
    mut planning_state: ResMut<PlanningState>,
    mut dialogue: ResMut<Dialogue>,
    mut game_state: ResMut<GameState>,
    start_battle_query: Query<&Clickable, With<PlanningStartBattle>>,
    articy: Res<Articy>,
) {
    for start_battle_clickable in start_battle_query.iter() {
        if start_battle_clickable.confirmed {
            if let Some(tutorial_dialogue) = tutorial_dialogue(game_state.as_ref()) {
                dialogue.queue(
                    Script::new(articy.dialogues.get(tutorial_dialogue).unwrap().clone()),
                    game_state.as_mut(),
                );
            } else {
                planning_state.start = true;
            }
        }
    }
}

fn planning_ui(
    mut contexts: EguiContexts,
    mut game_state: ResMut<GameState>,
    mut planning_state: ResMut<PlanningState>,
    mut dialogue: ResMut<Dialogue>,
    interaction_stack: Res<InteractionStack>,
    articy: Res<Articy>,
) {
    if planning_state.planning && interaction_stack.can_interact(InteractionMode::Game) && false {
        egui::Window::new("Planning").show(contexts.ctx_mut(), |ui| {
            ui.label(format!("Food remaining: {}", game_state.food));

            for unit_kind in UnitKind::iter() {
                let unit_cost = unit_kind.stats().cost;
                if ui
                    .button(format!(
                        "Feed {} ({} available, {} ready, {} sick, cost: {})",
                        unit_kind.name(),
                        game_state.available_army.get_count(unit_kind),
                        game_state.fed_army.get_count(unit_kind),
                        game_state.sick_army.get_count(unit_kind),
                        unit_cost,
                    ))
                    .clicked()
                    && game_state.available_army.get_count(unit_kind) > 0
                    && game_state.food >= unit_cost
                {
                    game_state.fed_army.mutate_count(unit_kind, |i| i + 1);
                    game_state.available_army.mutate_count(unit_kind, |i| i - 1);
                    game_state.food -= unit_cost;
                }
            }

            ui.add_space(16.);

            ui.label("Inventory");
            if game_state.inventory.is_empty() {
                ui.label("NO ITEMS");
            } else {
                let mut remove_item = None;
                for (item_index, item) in
                    game_state.inventory.items().clone().into_iter().enumerate()
                {
                    if ui.button(format!("Use {}", item.name())).clicked() {
                        game_state.consumed_items.push(item);
                        remove_item = Some(item_index);
                    }
                }
                if let Some(remove_item) = remove_item {
                    game_state.inventory.remove(remove_item);
                }
            }

            ui.add_space(16.);

            if ui.button("Start Battle").clicked() {
                if let Some(tutorial_dialogue) = tutorial_dialogue(game_state.as_ref()) {
                    dialogue.queue(
                        Script::new(articy.dialogues.get(tutorial_dialogue).unwrap().clone()),
                        game_state.as_mut(),
                    );
                } else {
                    planning_state.start = true;
                }
            }

            ui.add_space(32.);

            ui.horizontal(|ui| {
                if ui.button("Skip Battle").clicked() {
                    planning_state.skip = true;
                }

                if game_state.can_rewind() {
                    if ui.button("Rewind to Previous Battle").clicked() {
                        planning_state.rewind = true;
                    }
                }
            });
        });
    }
}

fn tutorial_dialogue(game_state: &GameState) -> Option<&'static str> {
    if game_state.quest.war_chef == 0 {
        match game_state.quest.battle {
            0 => {
                if game_state.available_army.peasants != 0 {
                    return Some("Tutorial1");
                }
            }
            1 => {
                if game_state.available_army.peasants != 0
                    || game_state.available_army.warriors != 0
                {
                    return Some("Tutorial2");
                }
            }
            2 => {
                if game_state.available_army.peasants != 0
                    || game_state.available_army.warriors != 0
                    || !game_state.inventory.is_empty()
                {
                    return Some("Tutorial3");
                }
            }
            _ => {}
        }
    }
    if game_state.fed_army.total_units() == 0 {
        Some("MustFeedUnits")
    } else {
        None
    }
}
