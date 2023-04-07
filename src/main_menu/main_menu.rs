use bevy::prelude::*;

use crate::{AppState, Articy, AssetLibrary, Dialogue, GameState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(main_menu_enter.in_schedule(OnEnter(AppState::MainMenu)))
                .add_system(main_menu_update.run_if(in_state(AppState::MainMenu)));
        }
    }
}

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
                font: asset_library.font_placeholder.clone(),
                font_size: 72.,
                color: Color::WHITE,
                ..Default::default()
            },
        )
        .with_alignment(TextAlignment::Center),
        ..Default::default()
    });
}

fn main_menu_update(mut next_state: ResMut<NextState<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(AppState::GameStart);
    } else if keys.just_pressed(KeyCode::S) {
        next_state.set(AppState::Sandbox);
    }
}
