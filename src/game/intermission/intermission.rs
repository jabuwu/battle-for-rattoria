use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{AppState, Articy, AssetLibrary, Depth, Dialogue, GameState, Transform2};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum IntermissionSystem {
    Enter,
    SpineReady,
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
                intermission_spine_ready
                    .in_set(IntermissionSystem::SpineReady)
                    .in_set(SpineSet::OnReady),
            )
            .add_system(
                intermission_update
                    .run_if(in_state(AppState::GameIntermission))
                    .in_set(IntermissionSystem::Update),
            );
        }
    }
}

#[derive(Component)]
struct CauldronSpine;

fn intermission_enter(
    mut dialogue: ResMut<Dialogue>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    articy: Res<Articy>,
    asset_library: Res<AssetLibrary>,
) {
    game_state.food += 15;
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
            texture: asset_library.image_planning_bg.clone(),
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
    commands.spawn((
        SpineBundle {
            skeleton: asset_library.spine_planning.clone(),
            ..Default::default()
        },
        Transform2::default(),
        Depth::Exact(0.01),
        CauldronSpine,
    ));
}

fn intermission_spine_ready(
    mut spine_ready_events: EventReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine, With<CauldronSpine>>,
) {
    for spine_ready_event in spine_ready_events.iter() {
        if let Ok(mut spine) = spine_query.get_mut(spine_ready_event.entity) {
            let _ = spine
                .animation_state
                .set_animation_by_name(0, "cauldron", true);
            let _ = spine
                .animation_state
                .set_animation_by_name(1, "intermission", true);
        }
    }
}

fn intermission_update(mut next_state: ResMut<NextState<AppState>>, dialogue: Res<Dialogue>) {
    if !dialogue.active() {
        next_state.set(AppState::GamePlanning);
    }
}
