use bevy::prelude::*;
use bevy_audio_plus::{prelude::AudioPlusSoundEffect, source::AudioPlusSource};

use crate::{Persistent, Transform2};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum TempSfxSystem {
    Update,
}

pub struct TempSfxPlugin;

impl Plugin for TempSfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(temp_sfx_update.in_set(TempSfxSystem::Update));
    }
}

#[derive(Component)]
pub struct TempSfx {
    time_to_live: f32,
}

impl TempSfx {
    pub fn new() -> Self {
        Self { time_to_live: 5. }
    }
}

fn temp_sfx_update(
    mut temp_sfx_query: Query<(Entity, &mut TempSfx)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (temp_sfx_entity, mut temp_sfx) in temp_sfx_query.iter_mut() {
        temp_sfx.time_to_live -= time.delta_seconds();
        if temp_sfx.time_to_live < 0. {
            if let Some(entity) = commands.get_entity(temp_sfx_entity) {
                entity.despawn_recursive();
            }
        }
    }
}

#[derive(Bundle)]
pub struct TempSfxBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub audio_source: AudioPlusSource,
    pub transform2: Transform2,
    pub persistent: Persistent,
    pub temp_sfx: TempSfx,
}

impl Default for TempSfxBundle {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            audio_source: AudioPlusSource::new(AudioPlusSoundEffect::none()),
            transform2: Transform2::default(),
            persistent: Persistent,
            temp_sfx: TempSfx::new(),
        }
    }
}
