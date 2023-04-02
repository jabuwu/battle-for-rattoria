use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_spine::SkeletonData;

#[derive(Resource, Default)]
pub struct AssetLibrary {
    pub font_placeholder: Handle<Font>,
    pub sound_placeholder: Handle<AudioSource>,
    pub image_background_bg: Handle<Image>,
    pub spine_rat: Handle<SkeletonData>,
    pub spine_rat_warrior: Handle<SkeletonData>,
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

fn asset_library_load(
    mut asset_library: ResMut<AssetLibrary>,
    mut skeletons: ResMut<Assets<SkeletonData>>,
    asset_server: Res<AssetServer>,
) {
    asset_library.font_placeholder = asset_server.load("fonts/FiraSans-Bold.ttf");
    asset_library.sound_placeholder = asset_server.load("audio/flying.ogg");
    asset_library.image_background_bg = asset_server.load("images/battlefield_bg.png");

    asset_library.spine_rat = skeletons.add(SkeletonData::new_from_binary(
        asset_server.load("spines/rat_test/skeleton.skel"),
        asset_server.load("spines/rat_test/rat_test.atlas"),
    ));

    asset_library.spine_rat_warrior = skeletons.add(SkeletonData::new_from_binary(
        asset_server.load("spines/rat_warrior/skeleton.skel"),
        asset_server.load("spines/rat_warrior/rat_warrior.atlas"),
    ));
}
