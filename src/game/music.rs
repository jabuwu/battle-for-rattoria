use std::time::Duration;

use bevy::prelude::*;
use bevy_audio_plus::source::AudioPlusSource;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl, AudioInstance, AudioTween};
use lerp::Lerp;

use crate::{AddFixedEvent, AppState, AssetLibrary, Persistent};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_fixed_event::<BattleJingleEvent>()
                .add_system(music_controller)
                .add_audio_channel::<Music>()
                .add_audio_channel::<BattleMusic>()
                .add_audio_channel::<IntroMusic>();
        }
    }
}

#[derive(Resource)]
pub struct Music;

#[derive(Resource)]
pub struct BattleMusic;

#[derive(Resource)]
pub struct IntroMusic;

#[derive(Default)]
pub struct MusicController {
    planning_instance: Option<Handle<AudioInstance>>,
    battle_instance: Option<Handle<AudioInstance>>,
    intro_instance: Option<Handle<AudioInstance>>,
    battle_crossfade: f64,
    intro_crossfade: f64,
    volume: f64,
    playback_rate: f64,
    jingle_time: f64,

    ambient_cauldron: Option<Entity>,
    ambient_battle: Option<Entity>,
}

pub enum BattleJingleEvent {
    Start,
    Victory,
    Defeat,
}

fn music_controller(
    mut local: Local<MusicController>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut battle_jingle_events: EventReader<BattleJingleEvent>,
    mut commands: Commands,
    channel: Res<AudioChannel<Music>>,
    battle_channel: Res<AudioChannel<BattleMusic>>,
    intro_channel: Res<AudioChannel<IntroMusic>>,
    app_state: Res<State<AppState>>,
    asset_library: Res<AssetLibrary>,
    time: Res<Time>,
) {
    let should_play = app_state.0.is_game_state();
    if should_play {
        if local.planning_instance.is_none() {
            local.planning_instance = Some(
                channel
                    .play(asset_library.sounds.music_planning.clone())
                    .looped()
                    .with_volume(0.)
                    .handle(),
            );
        }
        if local.battle_instance.is_none() {
            local.battle_instance = Some(
                battle_channel
                    .play(asset_library.sounds.music_battle.clone())
                    .looped()
                    .with_volume(0.)
                    .handle(),
            );
        }
        if local.intro_instance.is_none() {
            local.intro_instance = Some(
                intro_channel
                    .play(asset_library.sounds.music_intro.clone())
                    .looped()
                    .with_volume(0.)
                    .handle(),
            );
        }
    } else if !should_play {
        local.battle_crossfade = 0.;
        local.intro_crossfade = 1.;
        local.volume = 0.;
        local.playback_rate = 1.;
        local.jingle_time = 0.;
        if local.planning_instance.is_some() {
            channel.stop();
            local.planning_instance = None;
        }
        if local.battle_instance.is_some() {
            battle_channel.stop();
            local.battle_instance = None;
        }
        if local.intro_instance.is_some() {
            intro_channel.stop();
            local.intro_instance = None;
        }
    }

    if local.jingle_time > 0. {
        local.jingle_time -= time.delta_seconds_f64();
    }

    for jingle_event in battle_jingle_events.iter() {
        match jingle_event {
            BattleJingleEvent::Start => local.jingle_time = 2.,
            BattleJingleEvent::Victory => local.jingle_time = 5.,
            BattleJingleEvent::Defeat => local.jingle_time = 5.,
        }
    }

    let mut target_volume = if local.jingle_time > 0. { 0.05 } else { 0.7 };
    let mut target_playback_rate = 1.;
    if app_state.0 == AppState::GameRewind {
        target_volume *= 0.1;
        target_playback_rate = 0.95;
    }
    local.volume = local.volume.lerp(
        target_volume,
        time.delta_seconds_f64() * if local.jingle_time > 0. { 5. } else { 1. },
    );
    local.playback_rate = local
        .playback_rate
        .lerp(target_playback_rate, time.delta_seconds_f64() * 5.);
    let target_battle_crossfade =
        if app_state.0 == AppState::GameBattle || app_state.0 == AppState::GameOutro {
            1.
        } else {
            0.
        };
    let target_intro_crossfade = if app_state.0 == AppState::GameIntro {
        1.
    } else {
        0.
    };
    local.battle_crossfade = local
        .battle_crossfade
        .lerp(target_battle_crossfade, time.delta_seconds_f64() * 2.);
    local.intro_crossfade = local
        .intro_crossfade
        .lerp(target_intro_crossfade, time.delta_seconds_f64() * 1.);

    let planning_position = if let Some(planning_instance) = local
        .planning_instance
        .as_ref()
        .and_then(|i| audio_instances.get_mut(i))
    {
        planning_instance.set_volume(
            (1. - local.battle_crossfade.max(local.intro_crossfade)) * local.volume * 0.3,
            AudioTween::linear(Duration::from_millis(0)),
        );
        planning_instance.set_playback_rate(
            local.playback_rate,
            AudioTween::linear(Duration::from_millis(0)),
        );
        planning_instance.state().position().unwrap_or(0.)
    } else {
        0.
    };

    if let Some(battle_instance) = local
        .battle_instance
        .as_ref()
        .and_then(|i| audio_instances.get_mut(i))
    {
        battle_instance.set_volume(
            local.battle_crossfade * local.volume,
            AudioTween::linear(Duration::from_millis(0)),
        );
        battle_instance.set_playback_rate(
            local.playback_rate,
            AudioTween::linear(Duration::from_millis(0)),
        );
        let battle_position = battle_instance.state().position().unwrap_or(0.);
        if (battle_position - planning_position).abs() > 0.05 {
            battle_instance.seek_to(planning_position);
        }
    };

    if let Some(intro_instance) = local
        .intro_instance
        .as_ref()
        .and_then(|i| audio_instances.get_mut(i))
    {
        intro_instance.set_volume(
            local.intro_crossfade * local.volume * 0.3,
            AudioTween::linear(Duration::from_millis(0)),
        );
    };

    let wants_ambient_cauldron =
        app_state.0 == AppState::GameIntermission || app_state.0 == AppState::GamePlanning;
    if let Some(ambient_cauldron) = local.ambient_cauldron {
        if !wants_ambient_cauldron {
            if let Some(entity) = commands.get_entity(ambient_cauldron) {
                entity.despawn_recursive();
            }
            local.ambient_cauldron = None;
        }
    } else {
        if wants_ambient_cauldron {
            local.ambient_cauldron = Some(
                commands
                    .spawn((
                        AudioPlusSource::new(asset_library.sounds.ambient_cauldron.clone())
                            .as_looping(),
                        Persistent,
                    ))
                    .id(),
            );
        }
    }

    let wants_ambient_battle =
        app_state.0 == AppState::MainMenu || app_state.0 == AppState::GameBattle;
    if let Some(ambient_battle) = local.ambient_battle {
        if !wants_ambient_battle {
            if let Some(entity) = commands.get_entity(ambient_battle) {
                entity.despawn_recursive();
            }
            local.ambient_battle = None;
        }
    } else {
        if wants_ambient_battle {
            local.ambient_battle = Some(
                commands
                    .spawn((
                        AudioPlusSource::new(asset_library.sounds.ambient_battle.clone())
                            .as_looping(),
                        Persistent,
                    ))
                    .id(),
            );
        }
    }
}
