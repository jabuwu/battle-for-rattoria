use bevy::prelude::*;

use crate::{AppState, Articy, AssetLibrary, Depth, Dialogue, GameState, Transform2};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum IntermissionSystem {
    Enter,
    Update,
}

pub struct IntermissionPlugin;

impl Plugin for IntermissionPlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(
                intermission_enter
                    .in_schedule(OnEnter(AppState::GameIntermission))
                    .in_set(IntermissionSystem::Enter),
            )
            .add_system(
                intermission_update
                    .run_if(in_state(AppState::GameIntermission))
                    .in_set(IntermissionSystem::Update),
            );
        }
    }
}

fn intermission_enter(
    mut dialogue: ResMut<Dialogue>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    articy: Res<Articy>,
    asset_library: Res<AssetLibrary>,
) {
    game_state.food += 20;
    for used_item in game_state.used_items.clone().iter() {
        if let Some(script) = game_state.quest.item_script(*used_item, articy.as_ref()) {
            dialogue.queue(script, game_state.as_mut());
        }
    }
    if let Some(script) = game_state.quest.preplanning_script(articy.as_ref()) {
        dialogue.queue(script, game_state.as_mut());
    }
    commands.spawn((
        SpriteBundle {
            texture: asset_library.image_background_bg.clone(),
            ..Default::default()
        },
        Transform2::default(),
        Depth::Exact(0.),
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_library.image_vignette.clone(),

            ..Default::default()
        },
        Transform2::default(),
        Depth::Exact(0.1),
    ));
}

fn intermission_update(mut next_state: ResMut<NextState<AppState>>, dialogue: Res<Dialogue>) {
    if !dialogue.active() {
        next_state.set(AppState::GamePlanning);
    }
}
