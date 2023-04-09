use bevy::{prelude::*, sprite::Anchor};
use bevy_spine::prelude::*;

use crate::{AppState, AssetLibrary, Depth, Sfx, SfxKind, Transform2};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum IntroSystem {
    Enter,
    SpineReady,
    Next,
    Events,
}

pub struct IntroPlugin;

#[derive(Component)]
pub struct Intro {
    scene: usize,
}

#[derive(Component)]
pub struct IntroText;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(
                intro_enter
                    .in_schedule(OnEnter(AppState::GameIntro))
                    .in_set(IntroSystem::Enter),
            )
            .add_system(intro_spine_ready.in_set(IntroSystem::Enter))
            .add_system(intro_next.in_set(IntroSystem::Next))
            .add_system(intro_events.in_set(IntroSystem::Events));
        }
    }
}

fn intro_enter(mut commands: Commands, asset_library: Res<AssetLibrary>) {
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
            skeleton: asset_library.spine_intro.clone(),
            ..Default::default()
        },
        Transform2::default(),
        Depth::Exact(0.01),
        Intro { scene: 0 },
        SpineSync,
    ));
}

fn intro_spine_ready(
    mut spine_ready_events: EventReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine, With<Intro>>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for spine_ready_event in spine_ready_events.iter() {
        if let Ok(mut spine) = spine_query.get_mut(spine_ready_event.entity) {
            let _ = spine
                .animation_state
                .set_animation_by_name(0, "scene1", false);
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
                            IntroText,
                        ));
                    });
                }
            }
        }
    }
}

fn intro_next(
    mut spine_query: Query<(&mut Intro, &mut Spine)>,
    mut text_query: Query<&mut Text, With<IntroText>>,
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
        for (mut intro, mut intro_spine) in spine_query.iter_mut() {
            match intro.scene {
                0 => {
                    if !fast_skip {
                        sfx.play(SfxKind::CutsceneProceed);
                    }
                    let _ = intro_spine
                        .animation_state
                        .set_animation_by_name(1, "scene2", false);
                }
                1 => {
                    if !fast_skip {
                        sfx.play(SfxKind::CutsceneProceed);
                    }
                    let _ = intro_spine
                        .animation_state
                        .set_animation_by_name(2, "scene3", false);
                }
                2 => {
                    if !fast_skip {
                        sfx.play(SfxKind::CutsceneProceed);
                    }
                    let _ = intro_spine
                        .animation_state
                        .set_animation_by_name(3, "scene4", false);
                }
                3 => {
                    if !fast_skip {
                        sfx.play(SfxKind::CutsceneProceed);
                    }
                    let _ = intro_spine
                        .animation_state
                        .set_animation_by_name(4, "scene5", false);
                }
                4 => {
                    if !fast_skip {
                        sfx.play(SfxKind::CutsceneProceed);
                    }
                    let _ = intro_spine
                        .animation_state
                        .set_animation_by_name(4, "scene6", false);
                }
                5 => {
                    if !fast_skip {
                        sfx.play(SfxKind::CutsceneProceed);
                    }
                    let _ = intro_spine
                        .animation_state
                        .set_animation_by_name(5, "out", false);
                }
                _ => {}
            }
            intro.scene += 1;
        }
    }
}

fn intro_events(
    mut spine_events: EventReader<SpineEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    mut text_query: Query<&mut Text, With<IntroText>>,
    mut sfx: ResMut<Sfx>,
    intro_query: Query<&Intro>,
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
                if let Ok(intro) = intro_query.get(*entity) {
                    match name.as_str() {
                        "subtitles" => {
                            if *float as usize == intro.scene {
                                if !fast_skip {
                                    sfx.play(SfxKind::CutsceneTextAppear);
                                }
                                for mut text in text_query.iter_mut() {
                                    if let Some(section) = text.sections.get_mut(0) {
                                        section.value = string.replace("\\n", "\n");
                                    }
                                }
                            }
                        }
                        "out" => {
                            next_state.set(AppState::GameStart);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
