use bevy::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum FramesToLiveSystem {
    Update,
}

pub struct FramesToLivePlugin;

impl Plugin for FramesToLivePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            frames_to_live_update
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(FramesToLiveSystem::Update),
        );
    }
}

#[derive(Component)]
pub struct FramesToLive {
    pub frames: usize,
}

impl FramesToLive {
    pub fn new(frames: usize) -> Self {
        Self { frames }
    }
}

fn frames_to_live_update(
    mut frames_to_live_query: Query<(Entity, &mut FramesToLive)>,
    mut commands: Commands,
) {
    for (frames_to_live_entity, mut frames_to_live) in frames_to_live_query.iter_mut() {
        if frames_to_live.frames == 0 {
            if let Some(entity) = commands.get_entity(frames_to_live_entity) {
                entity.despawn_recursive();
            }
        } else {
            frames_to_live.frames -= 1;
        }
    }
}
