use bevy::{prelude::*, sprite::Anchor};
use bevy_spine::prelude::*;

use crate::{AppState, AssetLibrary, Depth, Sfx, SfxKind, Transform2};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum OutroSystem {
    Enter,
    SpineReady,
    Next,
    Events,
}

pub struct OutroPlugin;

#[derive(Component)]
pub struct Outro {
    scene: usize,
}

#[derive(Component)]
pub struct OutroText;

#[derive(Component)]
pub struct OutroMidText;

impl Plugin for OutroPlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(
                outro_enter
                    .in_schedule(OnEnter(AppState::GameOutro))
                    .in_set(OutroSystem::Enter),
            )
            .add_system(outro_spine_ready.in_set(OutroSystem::Enter))
            .add_system(outro_next.in_set(OutroSystem::Next))
            .add_system(outro_events.in_set(OutroSystem::Events));
        }
    }
}

fn outro_enter(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::splat(99999.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Transform2::default(),
        Depth::Exact(0.0),
    ));
    commands.spawn((
        SpineBundle {
            skeleton: asset_library.spine_outro.clone(),
            ..Default::default()
        },
        Transform2::default(),
        Depth::Exact(0.01),
        Outro { scene: 0 },
        SpineSync,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "".to_owned(),
                TextStyle {
                    font: asset_library.font_normal.clone(),
                    font_size: 72.,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            text_anchor: Anchor::Center,
            ..Default::default()
        },
        Transform2::default(),
        Depth::Exact(0.02),
        OutroMidText,
    ));
}

fn outro_spine_ready(
    mut spine_ready_events: EventReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine, With<Outro>>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for spine_ready_event in spine_ready_events.iter() {
        if let Ok(mut spine) = spine_query.get_mut(spine_ready_event.entity) {
            let _ = spine
                .animation_state
                .set_animation_by_name(0, "bevy_dance", true);
            let _ = spine
                .animation_state
                .set_animation_by_name(1, "scene1", false);
            if let Some(subtitles_bone) = spine_ready_event.bones.get("subtitles") {
                if let Some(mut subtitles_entity) = commands.get_entity(*subtitles_bone) {
                    subtitles_entity.with_children(|parent| {
                        parent.spawn((
                            Text2dBundle {
                                text: Text::from_section(
                                    "".to_owned(),
                                    TextStyle {
                                        font: asset_library.font_normal.clone(),
                                        font_size: 72.,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_alignment(TextAlignment::Center),
                                text_anchor: Anchor::Center,
                                ..Default::default()
                            },
                            Transform2::default(),
                            Depth::Inherit(0.01),
                            OutroText,
                        ));
                    });
                }
            }
        }
    }
}

fn outro_next(
    mut spine_query: Query<(&mut Outro, &mut Spine)>,
    mut text_query: Query<&mut Text, With<OutroText>>,
    mut sfx: ResMut<Sfx>,
    mouse_buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
) {
    let fast_skip = keys.pressed(KeyCode::LShift) || keys.pressed(KeyCode::RShift);
    if mouse_buttons.just_pressed(MouseButton::Left) || fast_skip {
        for mut text in text_query.iter_mut() {
            if let Some(section) = text.sections.get_mut(0) {
                section.value = "".to_owned();
            }
        }
        for (mut outro, mut outro_spine) in spine_query.iter_mut() {
            match outro.scene {
                0 => {
                    if !fast_skip {
                        sfx.play(SfxKind::CutsceneProceed);
                    }
                    let _ = outro_spine
                        .animation_state
                        .set_animation_by_name(2, "scene2", false);
                }
                1 => {
                    if !fast_skip {
                        sfx.play(SfxKind::CutsceneProceed);
                    }
                    let _ = outro_spine
                        .animation_state
                        .set_animation_by_name(3, "scene3", false);
                }
                2 => {
                    if !fast_skip {
                        sfx.play(SfxKind::CutsceneProceed);
                    }
                    let _ = outro_spine
                        .animation_state
                        .set_animation_by_name(4, "out", false);
                }
                _ => {}
            }
            outro.scene += 1;
        }
    }
}

fn outro_events(
    mut spine_events: EventReader<SpineEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    mut text_query: Query<&mut Text>,
    mut sfx: ResMut<Sfx>,
    subtext_query: Query<Entity, With<OutroText>>,
    midtext_query: Query<Entity, With<OutroMidText>>,
    outro_query: Query<&Outro>,
    keys: Res<Input<KeyCode>>,
) {
    let fast_skip = keys.pressed(KeyCode::LShift) || keys.pressed(KeyCode::RShift);
    for spine_event in spine_events.iter() {
        match spine_event {
            SpineEvent::Event {
                entity,
                name,
                string,
                float,
                ..
            } => {
                if let Ok(outro) = outro_query.get(*entity) {
                    match name.as_str() {
                        "subtitles" => {
                            if *float as usize == outro.scene {
                                if !fast_skip {
                                    sfx.play(SfxKind::CutsceneTextAppear);
                                }
                                for subtext_entity in subtext_query.iter() {
                                    if let Ok(mut text) = text_query.get_mut(subtext_entity) {
                                        if let Some(section) = text.sections.get_mut(0) {
                                            section.value = string.replace("\\n", "\n");
                                        }
                                    }
                                }
                            }
                        }
                        "midtext" => {
                            for subtext_entity in midtext_query.iter() {
                                if let Ok(mut text) = text_query.get_mut(subtext_entity) {
                                    if let Some(section) = text.sections.get_mut(0) {
                                        section.value = string.replace("\\n", "\n");
                                    }
                                }
                            }
                        }
                        "out" => {
                            next_state.set(AppState::MainMenu);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
