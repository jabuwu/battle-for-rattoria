use std::mem::take;

use bevy::{prelude::*, sprite::Anchor};
use strum::IntoEnumIterator;

use crate::{
    AppState, Articy, AssetLibrary, Clickable, ClickableSystem, Depth, Dialogue, GameState,
    PersistentGameState, Script, Sfx, SfxKind, Transform2, UnitKind,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum RewindSystem {
    Enter,
    StageClick,
    UpdateButton,
    UpdateButtonText,
    UpdateBattleInfo,
}

pub struct RewindPlugin;

impl Plugin for RewindPlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.init_resource::<RewindState>()
                .add_system(
                    rewind_enter
                        .in_schedule(OnEnter(AppState::GameRewind))
                        .in_set(RewindSystem::Enter),
                )
                .add_system(
                    rewind_stage_click
                        .in_set(RewindSystem::StageClick)
                        .after(ClickableSystem),
                )
                .add_system(
                    rewind_update_button
                        .in_set(RewindSystem::UpdateButton)
                        .after(ClickableSystem),
                )
                .add_system(rewind_update_button_text.in_set(RewindSystem::UpdateButtonText))
                .add_system(rewind_update_battle_info.in_set(RewindSystem::UpdateBattleInfo));
        }
    }
}

#[derive(Resource, Default)]
pub struct RewindState {
    stages: Vec<GameState>,
    selected: usize,
}

#[derive(Component)]
struct RewindStage {
    index: usize,
    enabled: bool,
}

#[derive(Component)]
struct RewindButton;

#[derive(Component)]
struct RewindText;

#[derive(Component)]
struct BattleInfo;

fn rewind_enter(
    mut commands: Commands,
    mut rewind_state: ResMut<RewindState>,
    mut game_state: ResMut<GameState>,
    mut persistent_game_state: ResMut<PersistentGameState>,
    mut dialogue: ResMut<Dialogue>,
    asset_library: Res<AssetLibrary>,
    articy: Res<Articy>,
) {
    *rewind_state = RewindState::default();
    rewind_state.stages = vec![];
    if game_state.checkpoint.is_none() {
        game_state.checkpoint();
    }
    let mut checkpoint = game_state.checkpoint.clone();
    while let Some(cp) = checkpoint {
        rewind_state.stages.push((*cp).clone());
        checkpoint = cp.checkpoint.clone();
    }
    rewind_state.stages = take(&mut rewind_state.stages).into_iter().rev().collect();
    rewind_state.selected = rewind_state.stages.len() - 1;
    if persistent_game_state.show_rewind_screen_dialogue {
        dialogue.queue(
            Script::new(articy.dialogues.get("RewindScreen").unwrap().clone()),
            game_state.as_mut(),
        );
        persistent_game_state.show_rewind_screen_dialogue = false;
    }
    commands.spawn((
        SpriteBundle {
            texture: asset_library.image_rewind_bg.clone(),
            ..Default::default()
        },
        Transform2::default(),
        Depth::Exact(0.),
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Select Battle",
                TextStyle {
                    font: asset_library.font_heading.clone(),
                    font_size: 128.,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            ..Default::default()
        },
        Transform2::from_xy(-615., 635.),
        Depth::Exact(0.1),
    ));
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: asset_library.image_atlas_rewind_battle.clone(),
                ..Default::default()
            },
            Clickable {
                shape: crate::CollisionShape::Rect {
                    offset: Vec2::ZERO,
                    size: Vec2::new(700., 100.),
                },
                ..Default::default()
            },
            Transform2::from_xy(590., -450.).with_scale(Vec2::splat(1.5)),
            Depth::Exact(0.1),
            RewindButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: asset_library.font_normal.clone(),
                            font_size: 42.,
                            color: Color::WHITE,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..Default::default()
                },
                Transform2::from_xy(0., -75.),
                Depth::Inherit(0.01),
                RewindText,
            ));
        });
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection {
                    value: "Checkpoint Info\n".to_owned(),
                    style: TextStyle {
                        font: asset_library.font_heading.clone(),
                        font_size: 128.,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "".to_owned(),
                    style: TextStyle {
                        font: asset_library.font_normal.clone(),
                        font_size: 42.,
                        color: Color::WHITE,
                    },
                },
            ])
            .with_alignment(TextAlignment::Center),
            text_anchor: Anchor::TopCenter,
            ..Default::default()
        },
        Transform2::from_xy(550., 705.),
        Depth::Exact(0.1),
        BattleInfo,
    ));
    let mut stage_index = 0;
    for y in 0..5 {
        let count = match y {
            0 => 3,
            1 => 3,
            2 => 4,
            3 => 4,
            _ => 3,
        };
        for x in 0..count {
            let enabled = stage_index < rewind_state.stages.len();
            let selected = stage_index == rewind_state.selected;
            let mut entity = commands.spawn((
                SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        color: match (selected, enabled) {
                            (true, _) => Color::WHITE,
                            (_, true) => Color::GRAY,
                            _ => Color::rgb(0.1, 0.1, 0.1),
                        },
                        index: y as usize,
                        ..Default::default()
                    },
                    texture_atlas: asset_library.image_atlas_war_chef_rewind.clone(),
                    ..Default::default()
                },
                Transform2::from_xy(x as f32 * 250. - 960., y as f32 * -250. + 430.),
                Depth::Exact(0.1),
                RewindStage {
                    index: stage_index,
                    enabled,
                },
            ));
            if enabled {
                entity.insert(Clickable {
                    shape: crate::CollisionShape::Rect {
                        offset: Vec2::ZERO,
                        size: Vec2::splat(200.),
                    },
                    ..Default::default()
                });
            }
            stage_index += 1;
        }
    }
}

fn rewind_stage_click(
    mut rewind_state: ResMut<RewindState>,
    mut rewind_stage_query: Query<(&mut TextureAtlasSprite, &RewindStage, &Clickable)>,
    mut sfx: ResMut<Sfx>,
    dialogue: Res<Dialogue>,
) {
    if dialogue.active() {
        return;
    }
    for (mut rewind_stage_sprite, rewind_stage, rewind_stage_clickable) in
        rewind_stage_query.iter_mut()
    {
        if rewind_stage_clickable.confirmed {
            sfx.play(SfxKind::UiButtonClick);
            rewind_state.selected = rewind_stage.index;
        }

        let selected = rewind_state.selected == rewind_stage.index;
        let enabled = rewind_stage.enabled;
        rewind_stage_sprite.color = match (selected, enabled) {
            (true, _) => Color::WHITE,
            (_, true) => Color::GRAY,
            _ => Color::rgb(0.1, 0.1, 0.1),
        };
    }
}

fn rewind_update_button(
    mut rewind_button_text_query: Query<(&mut TextureAtlasSprite, &Clickable), With<RewindButton>>,
    mut game_state: ResMut<GameState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut sfx: ResMut<Sfx>,
    rewind_state: Res<RewindState>,
    dialogue: Res<Dialogue>,
) {
    if dialogue.active() {
        return;
    }
    for (mut rewind_button_sprite, rewind_button_clickable) in rewind_button_text_query.iter_mut() {
        if rewind_button_clickable.just_clicked() {
            sfx.play(SfxKind::UiButtonClick);
        } else if rewind_button_clickable.just_hovered() {
            sfx.play(SfxKind::UiButtonHover);
        } else if rewind_button_clickable.just_released() {
            sfx.play(SfxKind::UiButtonRelease);
        }
        if rewind_button_clickable.clicked {
            rewind_button_sprite.index = 2;
        } else if rewind_button_clickable.hovered {
            rewind_button_sprite.index = 1;
        } else {
            rewind_button_sprite.index = 0;
        }
        if rewind_button_clickable.confirmed {
            sfx.play(SfxKind::UiButtonConfirm);
            *game_state = rewind_state.stages[rewind_state.selected].clone();
            game_state.checkpoint();
            next_state.set(AppState::GameIntermission);
        }
    }
}

fn rewind_update_button_text(
    mut rewind_text_query: Query<&mut Text, With<RewindText>>,
    rewind_state: Res<RewindState>,
) {
    for mut rewind_text_text in rewind_text_query.iter_mut() {
        if let Some(section) = rewind_text_text.sections.get_mut(0) {
            if rewind_state.selected == rewind_state.stages.len() - 1 {
                section.value = "".to_owned();
            } else {
                section.value = "Note: progress will be lost!".to_owned();
            }
        }
    }
}

fn rewind_update_battle_info(
    mut battle_info_query: Query<&mut Text, With<BattleInfo>>,
    rewind_state: Res<RewindState>,
) {
    if rewind_state.stages.len() == 0 {
        return;
    }
    let game_state = &rewind_state.stages[rewind_state.selected];
    for mut battle_info_text in battle_info_query.iter_mut() {
        if let Some(mut section) = battle_info_text.sections.get_mut(1) {
            let mut info = String::new();
            info += &format!("\nFood: {}\n\n", game_state.food);
            for unit_kind in UnitKind::iter() {
                info += &format!(
                    "{}: {}\n",
                    unit_kind.name_plural(),
                    game_state.available_army.get_count(unit_kind)
                );
            }
            if !game_state.inventory.is_empty() {
                info += &format!("\nItems:\n");
                for item in game_state.inventory.items() {
                    info += &format!("{}\n", item.name());
                }
            }
            section.value = info;
        }
    }
}
