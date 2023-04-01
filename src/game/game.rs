use bevy::prelude::*;
use bevy_audio_plus::prelude::AudioPlusListener;

use crate::{AppState, BattleStartEvent};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(game_enter.in_schedule(OnEnter(AppState::Game)))
                .add_system(game_update.run_if(in_state(AppState::Game)));
        }
    }
}

fn game_enter(mut commands: Commands, mut battle_start_events: EventWriter<BattleStartEvent>) {
    commands.spawn((Camera2dBundle::default(), AudioPlusListener));

    battle_start_events.send_default();
}

fn game_update(mut next_state: ResMut<NextState<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    }
}
