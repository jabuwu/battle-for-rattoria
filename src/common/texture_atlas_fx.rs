use bevy::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum TextureAtlasFxSystem {
    Update,
}

pub struct TextureAtlasFxPlugin;

impl Plugin for TextureAtlasFxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(texture_atlas_fx_update.in_set(TextureAtlasFxSystem::Update));
    }
}

#[derive(Component)]
pub struct TextureAtlasFx {
    frames: usize,
    frame: f32,
}

impl TextureAtlasFx {
    pub fn new(frames: usize) -> Self {
        TextureAtlasFx { frames, frame: 0. }
    }
}

fn texture_atlas_fx_update(
    mut texture_atlas_fx_query: Query<(Entity, &mut TextureAtlasFx, &mut TextureAtlasSprite)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (texture_atlas_fx_entity, mut texture_atlas_fx, mut texture_atlas_fx_sprite) in
        texture_atlas_fx_query.iter_mut()
    {
        texture_atlas_fx.frame += time.delta_seconds() * 10.;
        let frame_int = texture_atlas_fx.frame as usize;
        if frame_int < texture_atlas_fx.frames {
            texture_atlas_fx_sprite.index = frame_int;
        } else {
            if let Some(entity) = commands.get_entity(texture_atlas_fx_entity) {
                entity.despawn_recursive();
            }
        }
    }
}
