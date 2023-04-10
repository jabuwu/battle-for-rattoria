use std::mem::take;

use bevy::prelude::*;
use bevy_audio_plus::{prelude::AudioPlusSoundEffect, source::AudioPlusSource, AudioPlusSystem};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{AssetLibrary, BattleJingleEvent, Persistent, Sounds};

pub struct SfxPlugin;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct SfxSystem;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sfx>()
            .add_startup_system(sfx_create)
            .add_system(sfx_play.after(AudioPlusSystem::UpdateAudioSources));
    }
}

#[derive(Resource, Default)]
pub struct Sfx {
    queue: Vec<SfxKind>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter, Component)]
pub enum SfxKind {
    CutsceneTextAppear,
    CutsceneProceed,

    UiButtonClick,
    UiButtonHover,
    UiButtonRelease,
    UiButtonConfirm,
    UiFeedUnit,

    DialogueShow,
    DialogueHide,
    DialogueSkipText,
    DialogueProceed,
    DialogueCharacter,
    DialogueChoiceHover,
    DialogueChoiceSelect,

    CauldronAddSpice,

    LootGet,

    Mayhem,

    JingleStart,
    JingleVictory,
    JingleDefeat,
}

impl SfxKind {
    pub fn effect(&self, sounds: &Sounds) -> AudioPlusSoundEffect {
        match self {
            Self::CutsceneTextAppear => &sounds.cutscene_text_appear,
            Self::CutsceneProceed => &sounds.cutscene_proceed,
            Self::UiButtonClick => &sounds.ui_button_click,
            Self::UiButtonHover => &sounds.ui_button_hover,
            Self::UiButtonRelease => &sounds.ui_button_release,
            Self::UiButtonConfirm => &sounds.ui_button_confirm,
            Self::UiFeedUnit => &sounds.ui_feed_unit,
            Self::DialogueShow => &sounds.dialogue_show,
            Self::DialogueHide => &sounds.dialogue_hide,
            Self::DialogueSkipText => &sounds.dialogue_skip_text,
            Self::DialogueProceed => &sounds.dialogue_proceed,
            Self::DialogueCharacter => &sounds.dialogue_character,
            Self::DialogueChoiceHover => &sounds.dialogue_choice_hover,
            Self::DialogueChoiceSelect => &sounds.dialogue_choice_select,
            Self::CauldronAddSpice => &sounds.cauldron_add_spice,
            Self::LootGet => &sounds.loot_get,
            Self::Mayhem => &sounds.mayhem,
            Self::JingleStart => &sounds.jingle_start,
            Self::JingleVictory => &sounds.jingle_victory,
            Self::JingleDefeat => &sounds.jingle_defeat,
        }
        .clone()
    }
}

impl Sfx {
    pub fn play(&mut self, kind: SfxKind) {
        self.queue.push(kind);
    }
}

fn sfx_create(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    for sfx_kind in SfxKind::iter() {
        commands.spawn((
            AudioPlusSource::new(sfx_kind.effect(&asset_library.sounds)),
            Persistent,
            sfx_kind,
        ));
    }
}

fn sfx_play(
    mut sfx: ResMut<Sfx>,
    mut source_query: Query<(&mut AudioPlusSource, &SfxKind)>,
    mut battle_jingle_events: EventWriter<BattleJingleEvent>,
) {
    for kind in take(&mut sfx.queue) {
        for (mut source, source_kind) in source_query.iter_mut() {
            if kind == *source_kind {
                source.play();
            }
        }
        match kind {
            SfxKind::JingleStart => battle_jingle_events.send(BattleJingleEvent::Start),
            SfxKind::JingleVictory => battle_jingle_events.send(BattleJingleEvent::Victory),
            SfxKind::JingleDefeat => battle_jingle_events.send(BattleJingleEvent::Defeat),
            _ => {}
        }
    }
}
