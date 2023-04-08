use bevy::prelude::*;

use crate::{
    CollisionShape, DamageFlags, DamageReceiveEvent, DebugDraw, DebugDrawSettings, DebugRectangle,
    EventSet, HitBox, UpdateSet,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum FeelerSystem {
    Update,
    DebugDraw,
}

pub struct FeelerPlugin;

impl Plugin for FeelerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            feeler_update
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(FeelerSystem::Update)
                .in_set(UpdateSet)
                .after(EventSet::<DamageReceiveEvent>::Sender),
        )
        .add_system(feeler_debug_draw.in_set(FeelerSystem::DebugDraw));
    }
}

#[derive(Default, Component)]
pub struct Feeler {
    pub shape: CollisionShape,
    pub flags: DamageFlags,
    pub feeling: bool,
}

pub fn feeler_update(
    mut feeler_query: Query<(Entity, &mut Feeler)>,
    hit_box_query: Query<(Entity, &HitBox)>,
    transform_query: Query<&GlobalTransform>,
) {
    for (feeler_entity, mut feeler) in feeler_query.iter_mut() {
        feeler.feeling = false;
        let Ok(feeler_transform) = transform_query.get(feeler_entity) else {
            continue;
        };
        for (hit_box_entity, hit_box) in hit_box_query.iter() {
            if feeler_entity == hit_box_entity {
                continue;
            }
            let Ok(hit_box_transform) = transform_query.get(hit_box_entity) else {
                continue;
            };
            if hit_box.flags & feeler.flags != DamageFlags::empty()
                && hit_box
                    .shape
                    .at(hit_box_transform.translation().truncate())
                    .overlaps(feeler.shape.at(feeler_transform.translation().truncate()))
            {
                feeler.feeling = true;
                break;
            }
        }
    }
}

pub fn feeler_debug_draw(
    mut debug_draw: ResMut<DebugDraw>,
    debug_draw_settings: Res<DebugDrawSettings>,
    feeler_query: Query<(&Feeler, &GlobalTransform)>,
) {
    if debug_draw_settings.draw_feelers {
        for (feeler, feeler_transform) in feeler_query.iter() {
            let (offset, size) = match feeler.shape {
                CollisionShape::None => (Vec2::ZERO, Vec2::ZERO),
                CollisionShape::Point { .. } => (Vec2::ZERO, Vec2::ZERO),
                CollisionShape::Rect { offset, size } => (offset, size),
            };
            debug_draw.draw(DebugRectangle {
                position: feeler_transform.translation().truncate() + offset,
                size,
                color: Color::rgba(0., 0., 1., 0.2),
                ..Default::default()
            });
        }
    }
}
