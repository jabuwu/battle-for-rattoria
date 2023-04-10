use bevy::{
    asset::{HandleId, LoadState},
    prelude::*,
};
use bevy_audio_plus::prelude::AudioPlusSoundEffect;
use bevy_spine::{textures::SpineTextureCreateEvent, SkeletonData, SkeletonDataKind, SpineSystem};

use crate::{AppState, AssetLibrary, Depth, Transform2};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum LoadingSystem {
    Enter,
    Progress,
    CheckProgress,
}

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.init_resource::<Loading>()
                .add_system(
                    loading_enter
                        .in_schedule(OnEnter(AppState::Loading))
                        .in_set(LoadingSystem::Enter),
                )
                .add_system(
                    loading_progress
                        .in_set(LoadingSystem::Progress)
                        .run_if(in_state(AppState::Loading))
                        .after(LoadingSystem::CheckProgress),
                )
                .add_system(
                    loading_check_progress
                        .in_set(LoadingSystem::CheckProgress)
                        .after(SpineSystem::Load),
                );
        }
    }
}

#[derive(Resource, Default)]
struct Loading {
    progress: f32,
    spine_textures: Vec<HandleId>,
    proceed_timer: f32,
}

#[derive(Component)]
struct LoadingBar;

fn loading_enter(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GRAY,
                custom_size: Some(Vec2::new(800., 100.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Transform2::default(),
        Depth::Exact(0.0),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(800., 100.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Transform2::default().with_scale(Vec2::new(0., 1.)),
        Depth::Exact(0.1),
        LoadingBar,
    ));
}

fn loading_progress(
    mut loading_bar_query: Query<&mut Transform2, With<LoadingBar>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut loading: ResMut<Loading>,
    time: Res<Time>,
) {
    for mut loading_bar_transform in loading_bar_query.iter_mut() {
        loading_bar_transform.scale.x = loading.progress.clamp(0., 1.);
    }
    if loading.progress >= 1. {
        loading.proceed_timer += time.delta_seconds();
        if loading.proceed_timer > 0.2 {
            next_state.set(AppState::MainMenu);
        }
    } else {
        loading.proceed_timer = 0.;
    }
}

fn check_simple_loaded<T: Into<HandleId>>(asset: T, asset_server: &AssetServer) -> bool {
    asset_server.get_load_state(asset) == LoadState::Loaded
}

fn check_spine_loaded(spine_skeleton: &SkeletonData, asset_server: &AssetServer) -> bool {
    let atlas_loaded =
        asset_server.get_load_state(&spine_skeleton.atlas_handle) == LoadState::Loaded;
    let skeleton_loaded = match &spine_skeleton.kind {
        SkeletonDataKind::BinaryFile(handle) => asset_server.get_load_state(handle),
        SkeletonDataKind::JsonFile(handle) => asset_server.get_load_state(handle),
    } == LoadState::Loaded;
    atlas_loaded && skeleton_loaded
}

fn check_atlas_loaded(atlas: &TextureAtlas, asset_server: &AssetServer) -> bool {
    asset_server.get_load_state(&atlas.texture) == LoadState::Loaded
}

fn check_sfx_loaded(sfx: &AudioPlusSoundEffect, asset_server: &AssetServer) -> bool {
    for audio_source in sfx.audio_sources.iter() {
        if asset_server.get_load_state(audio_source) != LoadState::Loaded {
            return false;
        }
    }
    true
}

fn loading_check_progress(
    mut loading: ResMut<Loading>,
    mut spine_texture_create_events: EventReader<SpineTextureCreateEvent>,
    skeleton_assets: Res<Assets<SkeletonData>>,
    atlas_assets: Res<Assets<TextureAtlas>>,
    asset_library: Res<AssetLibrary>,
    asset_server: Res<AssetServer>,
) {
    for spine_texture_create_event in spine_texture_create_events.iter() {
        loading
            .spine_textures
            .push(spine_texture_create_event.handle.id());
    }

    let mut simple_assets = vec![
        asset_library.font_heading.id(),
        asset_library.font_normal.id(),
        asset_library.font_bold.id(),
        asset_library.image_menu_bg.id(),
        asset_library.image_background_bg.id(),
        asset_library.image_planning_bg.id(),
        asset_library.image_rewind_bg.id(),
        asset_library.image_vignette.id(),
        asset_library.image_bog_sick.id(),
        asset_library.sounds.music_battle.id(),
        asset_library.sounds.music_intro.id(),
        asset_library.sounds.music_planning.id(),
    ];
    simple_assets.extend(loading.spine_textures.clone());
    let atlases = [
        atlas_assets
            .get(&asset_library.image_atlas_planning_buttons)
            .unwrap(),
        atlas_assets.get(&asset_library.image_atlas_play).unwrap(),
        atlas_assets.get(&asset_library.image_atlas_units).unwrap(),
        atlas_assets
            .get(&asset_library.image_atlas_war_chef_rewind)
            .unwrap(),
        atlas_assets
            .get(&asset_library.image_atlas_start_battle)
            .unwrap(),
        atlas_assets
            .get(&asset_library.image_atlas_rewind_battle)
            .unwrap(),
        atlas_assets
            .get(&asset_library.image_atlas_blood_splat)
            .unwrap(),
    ];
    let skeletons = [
        skeleton_assets.get(&asset_library.spine_rat).unwrap(),
        skeleton_assets
            .get(&asset_library.spine_rat_warrior)
            .unwrap(),
        skeleton_assets
            .get(&asset_library.spine_rat_archer)
            .unwrap(),
        skeleton_assets.get(&asset_library.spine_rat_mage).unwrap(),
        skeleton_assets.get(&asset_library.spine_rat_brute).unwrap(),
        skeleton_assets
            .get(&asset_library.spine_attack_magic)
            .unwrap(),
        skeleton_assets.get(&asset_library.spine_dialogue).unwrap(),
        skeleton_assets
            .get(&asset_library.spine_battle_splash)
            .unwrap(),
        skeleton_assets.get(&asset_library.spine_planning).unwrap(),
        skeleton_assets.get(&asset_library.spine_intro).unwrap(),
        skeleton_assets.get(&asset_library.spine_outro).unwrap(),
    ];
    let sound_effects = [
        &asset_library.sounds.cutscene_text_appear,
        &asset_library.sounds.cutscene_proceed,
        &asset_library.sounds.ui_button_click,
        &asset_library.sounds.ui_button_hover,
        &asset_library.sounds.ui_button_release,
        &asset_library.sounds.ui_button_confirm,
        &asset_library.sounds.ui_feed_unit,
        &asset_library.sounds.dialogue_show,
        &asset_library.sounds.dialogue_hide,
        &asset_library.sounds.dialogue_skip_text,
        &asset_library.sounds.dialogue_proceed,
        &asset_library.sounds.dialogue_character,
        &asset_library.sounds.dialogue_choice_hover,
        &asset_library.sounds.dialogue_choice_select,
        &asset_library.sounds.cauldron_add_spice,
        &asset_library.sounds.loot_get,
        &asset_library.sounds.jingle_start,
        &asset_library.sounds.jingle_victory,
        &asset_library.sounds.jingle_defeat,
        &asset_library.sounds.unit_damage,
        &asset_library.sounds.unit_die,
        &asset_library.sounds.ambient_cauldron,
        &asset_library.sounds.ambient_battle,
    ];

    let count = simple_assets.len() + atlases.len() + skeletons.len() + sound_effects.len();
    let mut loaded = 0;
    for simple_asset in simple_assets {
        if check_simple_loaded(simple_asset, asset_server.as_ref()) {
            loaded += 1;
        }
    }
    for atlas in atlases {
        if check_atlas_loaded(atlas, asset_server.as_ref()) {
            loaded += 1;
        }
    }
    for skeleton in skeletons {
        if check_spine_loaded(skeleton, asset_server.as_ref()) {
            loaded += 1;
        }
    }
    for sound_effect in sound_effects {
        if check_sfx_loaded(sound_effect, asset_server.as_ref()) {
            loaded += 1;
        }
    }
    loading.progress = loaded as f32 / count as f32;
}
