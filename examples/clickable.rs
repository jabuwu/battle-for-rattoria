#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_game::{
    AssetLibraryPlugin, Clickable, CollisionShape, CommonPlugins, GamePlugins, Persistent,
    Transform2,
};

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dialogue".to_owned(),
                resolution: (1280., 768.).into(),
                canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(AssetLibraryPlugin)
        .add_plugins(CommonPlugins)
        .add_plugins(GamePlugins)
        .add_startup_system(setup)
        .add_system(update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Persistent));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(200.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Transform2::default().with_scale(Vec2::new(1.5, 2.0)),
        Clickable {
            shape: CollisionShape::Rect {
                offset: Vec2::ZERO,
                size: Vec2::splat(200.),
            },
            ..Default::default()
        },
    ));
}

fn update(mut clickable_query: Query<(&mut Sprite, &Clickable)>) {
    for (mut clickable_sprite, clickable) in clickable_query.iter_mut() {
        if clickable.clicked {
            clickable_sprite.color = Color::RED;
        } else if clickable.hovered {
            clickable_sprite.color = Color::GREEN;
        } else {
            clickable_sprite.color = Color::WHITE;
        }
    }
}
