use bevy::prelude::*;
use bevy_spine::SkeletonData;

use crate::Sounds;

#[derive(Resource, Default)]
pub struct AssetLibrary {
    //pub font_placeholder: Handle<Font>,
    pub font_heading: Handle<Font>,
    pub font_normal: Handle<Font>,
    pub font_bold: Handle<Font>,
    pub image_menu_bg: Handle<Image>,
    pub image_background_bg: Handle<Image>,
    pub image_planning_bg: Handle<Image>,
    pub image_rewind_bg: Handle<Image>,
    pub image_vignette: Handle<Image>,
    pub image_atlas_planning_buttons: Handle<TextureAtlas>,
    pub image_atlas_play: Handle<TextureAtlas>,
    pub image_atlas_units: Handle<TextureAtlas>,
    pub image_atlas_war_chef_rewind: Handle<TextureAtlas>,
    pub image_atlas_start_battle: Handle<TextureAtlas>,
    pub image_atlas_rewind_battle: Handle<TextureAtlas>,
    pub image_bog_sick: Handle<Image>,
    pub spine_rat: Handle<SkeletonData>,
    pub spine_rat_warrior: Handle<SkeletonData>,
    pub spine_rat_archer: Handle<SkeletonData>,
    pub spine_rat_mage: Handle<SkeletonData>,
    pub spine_rat_brute: Handle<SkeletonData>,
    pub spine_attack_magic: Handle<SkeletonData>,
    pub spine_fx_blood_splat: Handle<SkeletonData>,
    pub spine_dialogue: Handle<SkeletonData>,
    pub spine_battle_splash: Handle<SkeletonData>,
    pub spine_planning: Handle<SkeletonData>,
    pub spine_intro: Handle<SkeletonData>,
    pub spine_outro: Handle<SkeletonData>,

    pub sounds: Sounds,
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    //asset_library.font_placeholder = asset_server.load("fonts/FiraSans-Bold.ttf");
    asset_library.font_heading = asset_server.load("fonts/Enchanted Land.otf");
    asset_library.font_normal = asset_server.load("fonts/EBGaramond-SemiBold.ttf");
    asset_library.font_bold = asset_server.load("fonts/EBGaramond-ExtraBold.ttf");

    asset_library.image_menu_bg = asset_server.load("images/Logo_with_Background.png");
    asset_library.image_background_bg = asset_server.load("images/battlefield_bg.png");
    asset_library.image_planning_bg = asset_server.load("images/Background_Camp.png");
    asset_library.image_rewind_bg = asset_server.load("images/Background_Select_Battle.png");
    asset_library.image_vignette = asset_server.load("images/vignette.png");
    asset_library.image_bog_sick = asset_server.load("images/Sickness_icon.png");

    asset_library.image_atlas_planning_buttons = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("images/planning_buttons.png"),
        Vec2::new(230., 230.),
        7,
        6,
        None,
        None,
    ));
    asset_library.image_atlas_play = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("images/play.png"),
        Vec2::new(400., 200.),
        1,
        3,
        None,
        None,
    ));
    asset_library.image_atlas_units = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("images/units.png"),
        Vec2::new(400., 320.),
        3,
        2,
        None,
        None,
    ));
    asset_library.image_atlas_war_chef_rewind = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("images/war_chefs.png"),
        Vec2::new(210., 210.),
        5,
        1,
        None,
        None,
    ));
    asset_library.image_atlas_start_battle = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("images/Battle_Start_Button.png"),
        Vec2::new(440., 135.),
        1,
        3,
        None,
        None,
    ));
    asset_library.image_atlas_rewind_battle = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("images/Select_Battle_Button.png"),
        Vec2::new(700., 100.),
        1,
        3,
        None,
        None,
    ));

    asset_library.spine_rat = skeletons.add(SkeletonData::new_from_binary(
        asset_server.load("spines/rat_test/skeleton.skel"),
        asset_server.load("spines/rat_test/rat_test.atlas"),
    ));

    asset_library.spine_rat_warrior = skeletons.add(SkeletonData::new_from_binary(
        asset_server.load("spines/rat_warrior/skeleton.skel"),
        asset_server.load("spines/rat_warrior/rat_warrior.atlas"),
    ));

    asset_library.spine_rat_archer = skeletons.add(SkeletonData::new_from_binary(
        asset_server.load("spines/rat_archer/skeleton.skel"),
        asset_server.load("spines/rat_archer/rat_archer.atlas"),
    ));

    asset_library.spine_rat_mage = skeletons.add(SkeletonData::new_from_binary(
        asset_server.load("spines/rat_mage/skeleton.skel"),
        asset_server.load("spines/rat_mage/rat_mage.atlas"),
    ));

    asset_library.spine_rat_brute = skeletons.add(SkeletonData::new_from_binary(
        asset_server.load("spines/rat_brute/skeleton.skel"),
        asset_server.load("spines/rat_brute/rat_brute.atlas"),
    ));

    asset_library.spine_attack_magic = skeletons.add(SkeletonData::new_from_binary(
        asset_server.load("spines/magic_attack/skeleton.skel"),
        asset_server.load("spines/magic_attack/magic_attack.atlas"),
    ));

    asset_library.spine_fx_blood_splat = skeletons.add(SkeletonData::new_from_binary(
        asset_server.load("spines/blood_splat/skeleton.skel"),
        asset_server.load("spines/blood_splat/blood_splat.atlas"),
    ));

    asset_library.spine_dialogue = skeletons.add(SkeletonData::new_from_binary(
        asset_server.load("spines/dialogue/skeleton.skel"),
        asset_server.load("spines/dialogue/dialogue.atlas"),
    ));

    asset_library.spine_battle_splash = skeletons.add(SkeletonData::new_from_binary(
        asset_server.load("spines/battle_splash/skeleton.skel"),
        asset_server.load("spines/battle_splash/battle_splash.atlas"),
    ));

    asset_library.spine_planning = skeletons.add(SkeletonData::new_from_binary(
        asset_server.load("spines/planning/skeleton.skel"),
        asset_server.load("spines/planning/planning.atlas"),
    ));

    asset_library.spine_intro = skeletons.add(SkeletonData::new_from_binary(
        asset_server.load("spines/intro/skeleton.skel"),
        asset_server.load("spines/intro/intro.atlas"),
    ));

    asset_library.spine_outro = skeletons.add(SkeletonData::new_from_binary(
        asset_server.load("spines/outro/skeleton.skel"),
        asset_server.load("spines/outro/outro.atlas"),
    ));

    asset_library.sounds = Sounds::setup(asset_server.as_ref());
}
