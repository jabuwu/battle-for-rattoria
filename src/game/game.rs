use bevy::prelude::*;
use bevy_audio_plus::{
    prelude::{AudioPlusListener, AudioPlusSoundEffect},
    source::AudioPlusSource,
};

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(game_enter.in_schedule(OnEnter(AppState::Game)))
                .add_system(game_update.run_if(in_state(AppState::Game)));
        }
    }
}

fn game_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), AudioPlusListener));

    commands.spawn(
        AudioPlusSource::new(AudioPlusSoundEffect::single(
            asset_server.load("audio/flying.ogg"),
        ))
        .as_looping(),
    );

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Game!\nPress escape to go back!",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 72.,
                color: Color::WHITE,
                ..Default::default()
            },
        )
        .with_alignment(TextAlignment::Center),
        ..Default::default()
    });
}

fn game_update(mut next_state: ResMut<NextState<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    }
}
