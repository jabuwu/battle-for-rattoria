use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Resource, Default)]
pub struct AssetLibrary {
    pub font_placeholder: Handle<Font>,
    pub sound_placeholder: Handle<AudioSource>,
    pub image_rat: Handle<Image>,
}

pub struct AssetLibraryPlugin;

impl Plugin for AssetLibraryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetLibrary::default()).add_system(
            asset_library_load
                .in_schedule(CoreSchedule::Startup)
                .in_base_set(StartupSet::PreStartup),
        );
    }
}

fn asset_library_load(mut asset_library: ResMut<AssetLibrary>, asset_server: Res<AssetServer>) {
    asset_library.font_placeholder = asset_server.load("fonts/FiraSans-Bold.ttf");
    asset_library.sound_placeholder = asset_server.load("audio/flying.ogg");
    asset_library.image_rat = asset_server.load("images/rat.png");
}
