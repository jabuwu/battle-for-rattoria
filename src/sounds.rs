use bevy::prelude::*;
use bevy_audio_plus::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Default)]
pub struct Sounds {
    pub cutscene_text_appear: AudioPlusSoundEffect,
    pub cutscene_proceed: AudioPlusSoundEffect,

    pub ui_button_click: AudioPlusSoundEffect,
    pub ui_button_hover: AudioPlusSoundEffect,
    pub ui_button_release: AudioPlusSoundEffect,
    pub ui_button_confirm: AudioPlusSoundEffect,
    pub ui_feed_unit: AudioPlusSoundEffect,

    // when dialogue box appears
    pub dialogue_show: AudioPlusSoundEffect,
    // when dialogue box disappears
    pub dialogue_hide: AudioPlusSoundEffect,
    // when player skips typewriter effect
    pub dialogue_skip_text: AudioPlusSoundEffect,
    // when player proceeds normal text
    pub dialogue_proceed: AudioPlusSoundEffect,
    // for each individual character in dialogue (typewriter)
    pub dialogue_character: AudioPlusSoundEffect,
    // when player hovers over dialogue option
    pub dialogue_choice_hover: AudioPlusSoundEffect,
    // when player confirms dialogue option
    pub dialogue_choice_select: AudioPlusSoundEffect,

    pub cauldron_add_spice: AudioPlusSoundEffect,

    pub loot_get: AudioPlusSoundEffect,

    pub mayhem: AudioPlusSoundEffect,

    pub jingle_start: AudioPlusSoundEffect,
    pub jingle_victory: AudioPlusSoundEffect,
    pub jingle_defeat: AudioPlusSoundEffect,

    pub unit_explode: AudioPlusSoundEffect,
    pub unit_fire: AudioPlusSoundEffect,

    pub unit_peasant_attack: AudioPlusSoundEffect,
    pub unit_peasant_damage: AudioPlusSoundEffect,
    pub unit_peasant_die: AudioPlusSoundEffect,

    pub unit_archer_attack: AudioPlusSoundEffect,
    pub unit_archer_damage: AudioPlusSoundEffect,
    pub unit_archer_die: AudioPlusSoundEffect,

    pub unit_warrior_attack: AudioPlusSoundEffect,
    pub unit_warrior_damage: AudioPlusSoundEffect,
    pub unit_warrior_die: AudioPlusSoundEffect,

    pub unit_mage_attack: AudioPlusSoundEffect,
    pub unit_mage_damage: AudioPlusSoundEffect,
    pub unit_mage_die: AudioPlusSoundEffect,

    pub unit_brute_attack: AudioPlusSoundEffect,
    pub unit_brute_damage: AudioPlusSoundEffect,
    pub unit_brute_die: AudioPlusSoundEffect,

    pub ambient_cauldron: AudioPlusSoundEffect,
    pub ambient_battle: AudioPlusSoundEffect,

    pub music_planning: Handle<AudioSource>,
    pub music_battle: Handle<AudioSource>,
    pub music_intro: Handle<AudioSource>,
}

impl Sounds {
    pub fn setup(asset_server: &AssetServer) -> Sounds {
        let distance = 2000.;
        Sounds {
            cutscene_text_appear: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/placeholder.ogg")],
                volume: 0.,
                ..Default::default()
            },
            cutscene_proceed: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/sfx/UI/Button - Click 01.ogg")],
                volume: 0.7,
                ..Default::default()
            },
            ui_button_click: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/sfx/UI/Button - Click 01.ogg")],
                volume: 0.7,
                ..Default::default()
            },
            ui_button_hover: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/sfx/UI/Button - Hover 01.ogg")],
                volume: 0.7,
                ..Default::default()
            },
            ui_button_release: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/sfx/UI/Button - Release 01.ogg")],
                volume: 0.7,
                ..Default::default()
            },
            ui_button_confirm: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/sfx/UI/Confirm 01.ogg")],
                volume: 0.7,
                ..Default::default()
            },
            ui_feed_unit: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/UI/Feed Unit 01.ogg"),
                    asset_server.load("audio/sfx/UI/Feed Unit 02.ogg"),
                    asset_server.load("audio/sfx/UI/Feed Unit 03.ogg"),
                ],
                voices: 3,
                volume: 0.5,
                pitch: 0.9,
                pitch_variation: 0.2,
                ..Default::default()
            },
            dialogue_show: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/sfx/UI/Button - Release 01.ogg")],
                volume: 0.7,
                ..Default::default()
            },
            dialogue_hide: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/placeholder.ogg")],
                volume: 0.,
                ..Default::default()
            },
            dialogue_skip_text: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/sfx/UI/Button - Click 01.ogg")],
                volume: 0.3,
                ..Default::default()
            },
            dialogue_proceed: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/sfx/UI/Button - Click 01.ogg")],
                volume: 0.3,
                ..Default::default()
            },
            dialogue_character: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/sfx/UI/Dialogue Letter Tick 01.ogg")],
                volume: 0.,
                pitch_variation: 0.3,
                ..Default::default()
            },
            dialogue_choice_hover: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/sfx/UI/Button - Hover 01.ogg")],
                volume: 0.7,
                ..Default::default()
            },
            dialogue_choice_select: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/sfx/UI/Confirm 01.ogg")],
                volume: 0.7,
                ..Default::default()
            },
            cauldron_add_spice: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Spice - Add to Cauldron 01.ogg"),
                    asset_server.load("audio/sfx/Spice - Add to Cauldron 02.ogg"),
                    asset_server.load("audio/sfx/Spice - Add to Cauldron 03.ogg"),
                ],
                volume: 1.,
                ..Default::default()
            },
            loot_get: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/UI/Loot Received 01.ogg"),
                    asset_server.load("audio/sfx/UI/Loot Received 02.ogg"),
                    asset_server.load("audio/sfx/UI/Loot Received 03.ogg"),
                ],
                volume: 1.,
                ..Default::default()
            },
            mayhem: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Battle Start Mayhem 01.ogg"),
                    asset_server.load("audio/sfx/Battle Start Mayhem 02.ogg"),
                    asset_server.load("audio/sfx/Battle Start Mayhem 03.ogg"),
                ],
                volume: 0.7,
                ..Default::default()
            },
            jingle_start: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/music/Jingle - Battle Start.ogg")],
                volume: 0.7,
                ..Default::default()
            },
            jingle_victory: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/music/Jingle - Victory.ogg")],
                volume: 0.7,
                ..Default::default()
            },
            jingle_defeat: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/music/Jingle - Defeat.ogg")],
                volume: 0.7,
                ..Default::default()
            },
            unit_peasant_attack: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Peasant - Attack 01.ogg"),
                    asset_server.load("audio/sfx/Units/Peasant - Attack 02.ogg"),
                    asset_server.load("audio/sfx/Units/Peasant - Attack 03.ogg"),
                ],
                volume: 0.6,
                pitch: 0.8,
                pitch_variation: 0.4,
                positional: true,
                distance,
                chance: 0.05,
                ..Default::default()
            },
            unit_peasant_damage: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Common - Take Damage 01.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 02.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 03.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 04.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 05.ogg"),
                    asset_server.load("audio/sfx/Units/Peasant - Take Damage 01.ogg"),
                    asset_server.load("audio/sfx/Units/Peasant - Take Damage 02.ogg"),
                    asset_server.load("audio/sfx/Units/Peasant - Take Damage 03.ogg"),
                ],
                volume: 0.6,
                pitch: 0.8,
                pitch_variation: 0.4,
                positional: true,
                distance,
                chance: 0.1,
                ..Default::default()
            },
            unit_peasant_die: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Common - Die 01.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Die 02.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Die 03.ogg"),
                    asset_server.load("audio/sfx/Units/Peasant - Die 01.ogg"),
                    asset_server.load("audio/sfx/Units/Peasant - Die 02.ogg"),
                    asset_server.load("audio/sfx/Units/Peasant - Die 03.ogg"),
                ],
                volume: 0.8,
                pitch: 0.9,
                pitch_variation: 0.2,
                positional: true,
                distance,
                ..Default::default()
            },
            unit_warrior_attack: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Warrior - Attack 01.ogg"),
                    asset_server.load("audio/sfx/Units/Warrior - Attack 02.ogg"),
                    asset_server.load("audio/sfx/Units/Warrior - Attack 03.ogg"),
                ],
                volume: 0.6,
                pitch: 0.8,
                pitch_variation: 0.4,
                positional: true,
                distance,
                chance: 0.3,
                ..Default::default()
            },
            unit_warrior_damage: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Common - Take Damage 01.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 02.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 03.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 04.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 05.ogg"),
                    asset_server.load("audio/sfx/Units/Warrior - Take Damage 01.ogg"),
                    asset_server.load("audio/sfx/Units/Warrior - Take Damage 02.ogg"),
                    asset_server.load("audio/sfx/Units/Warrior - Take Damage 03.ogg"),
                ],
                volume: 0.6,
                pitch: 0.8,
                pitch_variation: 0.4,
                positional: true,
                distance,
                chance: 0.3,
                ..Default::default()
            },
            unit_warrior_die: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Common - Die 01.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Die 02.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Die 03.ogg"),
                    asset_server.load("audio/sfx/Units/Warrior - Die 01.ogg"),
                    asset_server.load("audio/sfx/Units/Warrior - Die 02.ogg"),
                    asset_server.load("audio/sfx/Units/Warrior - Die 03.ogg"),
                ],
                volume: 0.8,
                pitch: 0.9,
                pitch_variation: 0.2,
                positional: true,
                distance,
                ..Default::default()
            },
            unit_archer_attack: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Archer - Attack 01.ogg"),
                    asset_server.load("audio/sfx/Units/Archer - Attack 02.ogg"),
                    asset_server.load("audio/sfx/Units/Archer - Attack 03.ogg"),
                ],
                volume: 0.6,
                pitch: 0.8,
                pitch_variation: 0.4,
                positional: true,
                distance,
                chance: 0.3,
                ..Default::default()
            },
            unit_archer_damage: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Common - Take Damage 01.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 02.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 03.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 04.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 05.ogg"),
                    asset_server.load("audio/sfx/Units/Archer - Take Damage 01.ogg"),
                    asset_server.load("audio/sfx/Units/Archer - Take Damage 02.ogg"),
                    asset_server.load("audio/sfx/Units/Archer - Take Damage 03.ogg"),
                ],
                volume: 0.6,
                pitch: 0.8,
                pitch_variation: 0.4,
                positional: true,
                distance,
                chance: 0.3,
                ..Default::default()
            },
            unit_archer_die: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Common - Die 01.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Die 02.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Die 03.ogg"),
                    asset_server.load("audio/sfx/Units/Archer - Die 01.ogg"),
                    asset_server.load("audio/sfx/Units/Archer - Die 02.ogg"),
                    asset_server.load("audio/sfx/Units/Archer - Die 03.ogg"),
                ],
                volume: 0.8,
                pitch: 0.9,
                pitch_variation: 0.2,
                positional: true,
                distance,
                ..Default::default()
            },
            unit_mage_attack: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Mage - Attack 01.ogg"),
                    asset_server.load("audio/sfx/Units/Mage - Attack 02.ogg"),
                    asset_server.load("audio/sfx/Units/Mage - Attack 03.ogg"),
                ],
                volume: 0.6,
                pitch: 0.8,
                pitch_variation: 0.4,
                positional: true,
                distance,
                ..Default::default()
            },
            unit_mage_damage: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Common - Take Damage 01.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 02.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 03.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 04.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 05.ogg"),
                    asset_server.load("audio/sfx/Units/Mage - Take Damage 01.ogg"),
                    asset_server.load("audio/sfx/Units/Mage - Take Damage 02.ogg"),
                    asset_server.load("audio/sfx/Units/Mage - Take Damage 03.ogg"),
                ],
                volume: 0.6,
                pitch: 0.8,
                pitch_variation: 0.4,
                positional: true,
                distance,
                ..Default::default()
            },
            unit_mage_die: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Common - Die 01.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Die 02.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Die 03.ogg"),
                    asset_server.load("audio/sfx/Units/Mage - Die 01.ogg"),
                    asset_server.load("audio/sfx/Units/Mage - Die 02.ogg"),
                    asset_server.load("audio/sfx/Units/Mage - Die 03.ogg"),
                ],
                volume: 0.8,
                pitch: 0.9,
                pitch_variation: 0.2,
                positional: true,
                distance,
                ..Default::default()
            },
            unit_brute_attack: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Brute - Attack 01.ogg"),
                    asset_server.load("audio/sfx/Units/Brute - Attack 02.ogg"),
                    asset_server.load("audio/sfx/Units/Brute - Attack 03.ogg"),
                ],
                volume: 0.6,
                pitch: 0.8,
                pitch_variation: 0.4,
                positional: true,
                distance,
                ..Default::default()
            },
            unit_brute_damage: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Common - Take Damage 01.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 02.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 03.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 04.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Take Damage 05.ogg"),
                    asset_server.load("audio/sfx/Units/Brute - Take Damage 01.ogg"),
                    asset_server.load("audio/sfx/Units/Brute - Take Damage 02.ogg"),
                    asset_server.load("audio/sfx/Units/Brute - Take Damage 03.ogg"),
                ],
                volume: 0.6,
                pitch: 0.8,
                pitch_variation: 0.4,
                positional: true,
                distance,
                ..Default::default()
            },
            unit_brute_die: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Common - Die 01.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Die 02.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Die 03.ogg"),
                    asset_server.load("audio/sfx/Units/Brute - Die 01.ogg"),
                    asset_server.load("audio/sfx/Units/Brute - Die 02.ogg"),
                    asset_server.load("audio/sfx/Units/Brute - Die 03.ogg"),
                ],
                volume: 0.8,
                pitch: 0.9,
                pitch_variation: 0.2,
                positional: true,
                distance,
                ..Default::default()
            },
            unit_explode: AudioPlusSoundEffect {
                audio_sources: vec![
                    asset_server.load("audio/sfx/Units/Common - Explode 01.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Explode 02.ogg"),
                    asset_server.load("audio/sfx/Units/Common - Explode 03.ogg"),
                ],
                volume: 0.8,
                pitch: 0.95,
                pitch_variation: 0.1,
                positional: true,
                distance,
                ..Default::default()
            },
            unit_fire: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/sfx/Units/Common - Catch Fire 01.ogg")],
                volume: 0.5,
                positional: true,
                distance,
                ..Default::default()
            },
            ambient_cauldron: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/sfx/Ambience - Preparation 01.ogg")],
                volume: 0.6,
                ..Default::default()
            },
            ambient_battle: AudioPlusSoundEffect {
                audio_sources: vec![asset_server.load("audio/sfx/Ambience - Battle 01.ogg")],
                volume: 0.7,
                ..Default::default()
            },
            music_planning: asset_server.load("audio/music/Preparation.ogg"),
            music_battle: asset_server.load("audio/music/Battle.ogg"),
            music_intro: asset_server.load("audio/music/Intro.ogg"),
        }
    }
}
