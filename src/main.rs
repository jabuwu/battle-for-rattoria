#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use bevy_audio_plus::prelude::AudioPlusListener;
use bevy_game::{
    AppStatePlugin, AssetLibraryPlugin, CommonPlugins, GameDirector, GamePlugins, LoadingPlugin,
    MainMenuPlugins, Persistent,
};
use std::io::Cursor;
use winit::window::Icon;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy game".to_owned(), // ToDo
                resolution: (1280., 768.).into(),
                canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(AppStatePlugin)
        .add_plugin(AssetLibraryPlugin)
        .add_plugins(CommonPlugins)
        .add_plugin(LoadingPlugin)
        .add_plugins(MainMenuPlugins)
        .add_plugins(GamePlugins)
        .add_system(setup.in_schedule(CoreSchedule::Startup))
        .add_system(set_window_icon.in_schedule(CoreSchedule::Startup))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), AudioPlusListener, Persistent));
    commands.spawn((GameDirector, Persistent));
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let primary = windows.get_window(primary_entity).unwrap();
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
