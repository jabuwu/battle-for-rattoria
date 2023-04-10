use bevy::prelude::*;
use bevy_spine::{SkeletonData, SpinePlugin};
use bullet::BulletPlugin;
use player::{PlayerPlugin, PlayerSpawnEvent};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpinePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(BulletPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut skeletons: ResMut<Assets<SkeletonData>>,
    mut player_spawn_events: EventWriter<PlayerSpawnEvent>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    let skeleton = SkeletonData::new_from_binary(
        asset_server.load("spineboy/export/spineboy-pro.skel"),
        asset_server.load("spineboy/export/spineboy-pma.atlas"),
    );
    let skeleton_handle = skeletons.add(skeleton);
    player_spawn_events.send(PlayerSpawnEvent {
        skeleton: skeleton_handle,
    });
}

mod bullet;
mod player;
