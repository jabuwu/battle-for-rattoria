use bevy::prelude::*;
use bevy_spine::prelude::*;
use rand::prelude::*;

use crate::{
    AddFixedEvent, AssetLibrary, Banner, Depth, DepthLayer, EventSet, SpawnSet, SpineSpawnSet,
    Transform2, YOrder,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum BannerSystem {
    Spawn,
    OnReady,
    Wave,
}

pub struct BannerPlugin;

impl Plugin for BannerPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_event::<BannerSpawnEvent>()
            .add_system(
                banner_spawn
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(BannerSystem::Spawn)
                    .in_set(SpawnSet)
                    .in_set(SpineSpawnSet)
                    .after(EventSet::<BannerSpawnEvent>::Sender),
            )
            .add_system(
                banner_on_ready
                    .in_set(BannerSystem::OnReady)
                    .in_set(SpineSet::OnReady),
            )
            .add_system(banner_wave.in_set(BannerSystem::Wave));
    }
}

#[derive(Component)]
pub struct BannerComp {
    banner: Banner,
}

#[derive(Default)]
pub struct BannerSpawnEvent {
    pub banner: Banner,
    pub position: Vec2,
}

fn banner_spawn(
    mut commands: Commands,
    mut spawn_events: EventReader<BannerSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    for spawn_event in spawn_events.iter() {
        commands.spawn((
            SpineBundle {
                skeleton: asset_library.spine_banner.clone(),
                ..Default::default()
            },
            Transform2::from_translation(spawn_event.position).with_scale(Vec2::splat(0.75)),
            YOrder,
            Depth::from(DepthLayer::YOrder(0.)),
            BannerComp {
                banner: spawn_event.banner,
            },
        ));
    }
}

fn banner_on_ready(
    mut spine_ready_events: EventReader<SpineReadyEvent>,
    mut spine_query: Query<(&mut Spine, &BannerComp)>,
) {
    for spine_ready_event in spine_ready_events.iter() {
        if let Ok((mut spine, banner)) = spine_query.get_mut(spine_ready_event.entity) {
            let _ = spine.skeleton.set_skin_by_name(match banner.banner {
                Banner::Player => "player",
                Banner::WarChef1 => "wc1",
                Banner::WarChef2 => "wc2",
                Banner::WarChef3 => "wc3",
                Banner::WarChef4 => "wc4",
                Banner::WarChef5 => "wc5",
            });
            let _ = spine.animation_state.set_animation_by_name(0, "wave", true);
        }
    }
}

fn banner_wave(mut spine_query: Query<&mut Spine, With<BannerComp>>) {
    let mut rng = thread_rng();
    for mut spine in spine_query.iter_mut() {
        if let Some(mut track) = spine.animation_state.track_at_index_mut(0) {
            if rng.gen_bool(0.01) {
                track.set_timescale(rng.gen_range(0.4..1.4));
            }
        }
    }
}
