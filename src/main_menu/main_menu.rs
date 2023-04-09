use bevy::{prelude::*, sprite::Anchor};

use crate::{
    AppState, Articy, AssetLibrary, Clickable, CollisionShape, Depth, Dialogue, GameState, Sfx,
    SfxKind, Transform2,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(main_menu_enter.in_schedule(OnEnter(AppState::MainMenu)))
                .add_system(main_menu_update.run_if(in_state(AppState::MainMenu)));
        }
    }
}

#[derive(Component)]
struct PlayButton;

fn main_menu_enter(
    mut commands: Commands,
    mut dialogue: ResMut<Dialogue>,
    mut game_state: ResMut<GameState>,
    asset_library: Res<AssetLibrary>,
    articy: Res<Articy>,
) {
    dialogue.clear();
    for (name, value) in articy.global_variables.iter() {
        game_state.global_variables.insert(name.clone(), *value);
    }
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "War Chef: Battle for Rattoria\n\nPress space to play\n\nPress S to enter Sandbox",
            TextStyle {
                font: asset_library.font_normal.clone(),
                font_size: 72.,
                color: Color::WHITE,
                ..Default::default()
            },
        )
        .with_alignment(TextAlignment::Center),
        ..Default::default()
    });
    commands.spawn((
        SpriteBundle {
            texture: asset_library.image_menu_bg.clone(),
            ..Default::default()
        },
        Transform2::default(),
        Depth::Exact(0.),
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_library.image_menu_bg.clone(),
            ..Default::default()
        },
        Transform2::from_xy(0., 210.),
        Depth::Exact(0.1),
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: asset_library.image_atlas_play.clone(),
            ..Default::default()
        },
        Transform2::from_xy(0., -480.).with_scale(Vec2::splat(1.4)),
        Depth::Exact(0.2),
        Clickable {
            shape: CollisionShape::Rect {
                offset: Vec2::ZERO,
                size: Vec2::new(350., 180.),
            },
            ..Default::default()
        },
        PlayButton,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "A game for Bevy Jam 3",
                TextStyle {
                    font: asset_library.font_normal.clone(),
                    color: Color::GRAY,
                    font_size: 42.,
                },
            ),
            text_anchor: Anchor::BottomLeft,
            ..Default::default()
        },
        Transform2::from_xy(-1220., -720.).with_scale(Vec2::splat(1.5)),
        Depth::Exact(0.2),
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection {
                    value: "Made by\n".to_owned(),
                    style: TextStyle {
                        font: asset_library.font_heading.clone(),
                        color: Color::GRAY,
                        font_size: 52.,
                    },
                },
                TextSection {
                    value: "jabu\nSilkForCalde\nFantazmo\nMordi".to_owned(),
                    style: TextStyle {
                        font: asset_library.font_normal.clone(),
                        color: Color::GRAY,
                        font_size: 32.,
                    },
                },
            ])
            .with_alignment(TextAlignment::Center),
            text_anchor: Anchor::BottomRight,
            ..Default::default()
        },
        Transform2::from_xy(1220., -720.).with_scale(Vec2::splat(1.5)),
        Depth::Exact(0.2),
    ));
}

fn main_menu_update(
    mut next_state: ResMut<NextState<AppState>>,
    mut button_query: Query<(&mut TextureAtlasSprite, &Clickable), With<PlayButton>>,
    mut sfx: ResMut<Sfx>,
    keys: Res<Input<KeyCode>>,
) {
    for (mut button_sprite, button_clickable) in button_query.iter_mut() {
        if button_clickable.just_clicked() {
            sfx.play(SfxKind::UiButtonClick);
        } else if button_clickable.just_hovered() {
            sfx.play(SfxKind::UiButtonHover);
        } else if button_clickable.just_released() {
            sfx.play(SfxKind::UiButtonRelease);
        }
        if button_clickable.clicked {
            button_sprite.index = 2;
        } else if button_clickable.hovered {
            button_sprite.index = 1;
        } else {
            button_sprite.index = 0;
        }
        if button_clickable.confirmed {
            sfx.play(SfxKind::UiButtonConfirm);
            next_state.set(AppState::GameIntro);
        }
    }
    if keys.just_pressed(KeyCode::S) {
        next_state.set(AppState::Sandbox);
    }
}
