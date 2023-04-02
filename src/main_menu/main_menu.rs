use bevy::prelude::*;

use crate::{AppState, AssetLibrary};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(main_menu_enter.in_schedule(OnEnter(AppState::MainMenu)))
                .add_system(main_menu_update.run_if(in_state(AppState::MainMenu)));
        }
    }
}

fn main_menu_enter(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Main Menu\nPress space to play",
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
    }
}
