use bevy::{
    asset::{HandleId, LoadState},
    prelude::*,
};
use bevy_kira_audio::AudioSource;
use bevy_spine::{Atlas, SkeletonBinary};

use crate::{AppState, Depth, Transform2};

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
                        .run_if(in_state(AppState::Loading)),
                );
        }
    }
}

#[derive(Resource, Default)]
struct Loading {
    progress: f32,
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
                custom_size: Some(Vec2::new(400., 100.)),
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
    loading: Res<Loading>,
) {
    for mut loading_bar_transform in loading_bar_query.iter_mut() {
        loading_bar_transform.scale.x = loading.progress;
    }
    if loading.progress == 1. {
        next_state.set(AppState::MainMenu);
    }
}

macro_rules! loading_check_progress {
    ($count:literal, $($ident:ident:$ty:ty),*) => {
        fn loading_check_progress(
            mut loading: ResMut<Loading>,
            asset_server: Res<AssetServer>,
            $(
                $ident: Res<Assets<$ty>>,
            )*
        ) {
            let mut asset_count = 0;
            $(
                for (handle, _) in $ident.iter() {
                    if matches!(handle, HandleId::AssetPathId(..)) {
                        let state = asset_server.get_load_state(handle);
                        if state == LoadState::Loaded {
                            asset_count += 1;
                        }
                    }
                }
            )*
            loading.progress = asset_count as f32 / $count as f32;
        }
    };
}

// game jam'd
loading_check_progress!(
    67,
    images: Image,
    audio_sources: AudioSource,
    fonts: Font,
    atlases: Atlas,
    skeletons: SkeletonBinary
);
