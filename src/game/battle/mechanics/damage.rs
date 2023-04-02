use bevy::prelude::*;
use bitflags::bitflags;

use crate::{
    CollisionShape, DebugDraw, DebugDrawSettings, DebugRectangle, EventSet, FramesToLiveSystem,
    UpdateSet,
};

bitflags! {
    pub struct DamageFlags: u32 {
        const FRIENDLY = 0b00000001;
        const ENEMY = 0b00000010;
    }
}

impl Default for DamageFlags {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum DamageKind {
    #[default]
    Flesh,
    Sword,
    Magic,
}

impl DamageKind {
    pub fn damage_multiplier(&self, defense_kind: DefenseKind) -> f32 {
        match self {
            Self::Flesh => match defense_kind {
                DefenseKind::Flesh => 1.,
                DefenseKind::Armor => 0.5,
            },
            Self::Sword => match defense_kind {
                DefenseKind::Flesh => 2.,
                DefenseKind::Armor => 1.,
            },
            Self::Magic => match defense_kind {
                DefenseKind::Flesh => 1.,
                DefenseKind::Armor => 2.,
            },
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum DefenseKind {
    #[default]
    Flesh,
    Armor,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum DamageSystem {
    Update,
    Events,
    DebugDraw,
}

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageInflictEvent>()
            .add_event::<DamageReceiveEvent>()
            .add_system(
                damage_update
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(DamageSystem::Update)
                    .in_set(UpdateSet)
                    .in_set(EventSet::<DamageInflictEvent>::Sender)
                    .after(FramesToLiveSystem::Update),
            )
            .add_system(
                damage_events
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(DamageSystem::Events)
                    .in_set(UpdateSet)
                    .in_set(EventSet::<DamageReceiveEvent>::Sender)
                    .after(EventSet::<DamageInflictEvent>::Sender),
            )
            .add_system(damage_debug_draw.in_set(DamageSystem::DebugDraw));
    }
}

#[derive(Default, Component)]
pub struct HitBox {
    pub flags: DamageFlags,
    pub shape: CollisionShape,
    pub defense_kind: DefenseKind,
}

#[derive(Clone, Copy, Default, Component)]
pub struct HurtBox {
    pub flags: DamageFlags,
    pub shape: CollisionShape,
    pub damage: f32,
    pub damage_kind: DamageKind,
    pub max_hits: usize,
}

pub struct DamageInflictEvent {
    pub entity: Entity,
    pub damage: f32,
}

pub struct DamageReceiveEvent {
    pub entity: Entity,
    pub damage: f32,
    _private: (),
}

pub fn damage_update(
    mut damage_inflict_events: EventWriter<DamageInflictEvent>,
    mut hurt_box_query: Query<(Entity, &mut HurtBox)>,
    hit_box_query: Query<(Entity, &HitBox)>,
    transform_query: Query<&GlobalTransform>,
) {
    for (hurt_box_entity, mut hurt_box) in hurt_box_query.iter_mut() {
        let Ok(hurt_box_transform) = transform_query.get(hurt_box_entity) else {
            continue;
        };
        if hurt_box.max_hits == 0 {
            continue;
        }
        for (hit_box_entity, hit_box) in hit_box_query.iter() {
            if hurt_box_entity == hit_box_entity {
                continue;
            }
            let Ok(hit_box_transform) = transform_query.get(hit_box_entity) else {
                continue;
            };
            if hurt_box
                .shape
                .at(hurt_box_transform.translation().truncate())
                .overlaps(hit_box.shape.at(hit_box_transform.translation().truncate()))
                && hurt_box.flags & hit_box.flags != DamageFlags::empty()
            {
                let damage =
                    hurt_box.damage * hurt_box.damage_kind.damage_multiplier(hit_box.defense_kind);
                if damage > 0. && hurt_box.max_hits > 0 {
                    damage_inflict_events.send(DamageInflictEvent {
                        entity: hit_box_entity,
                        damage,
                    });
                    hurt_box.max_hits -= 1;
                }
            }
        }
    }
}

pub fn damage_events(
    mut damage_inflict_events: EventReader<DamageInflictEvent>,
    mut damage_receive_events: EventWriter<DamageReceiveEvent>,
) {
    for damage_inflict_event in damage_inflict_events.iter() {
        damage_receive_events.send(DamageReceiveEvent {
            entity: damage_inflict_event.entity,
            damage: damage_inflict_event.damage,
            _private: (),
        });
    }
}

pub fn damage_debug_draw(
    mut debug_draw: ResMut<DebugDraw>,
    debug_draw_settings: Res<DebugDrawSettings>,
    hit_box_query: Query<(&HitBox, &GlobalTransform)>,
    hurt_box_query: Query<(&HurtBox, &GlobalTransform)>,
) {
    if debug_draw_settings.draw_hit_boxes {
        for (hit_box, hit_box_transform) in hit_box_query.iter() {
            let size = match hit_box.shape {
                CollisionShape::None => Vec2::ZERO,
                CollisionShape::Rect(size) => size,
            };
            debug_draw.draw(DebugRectangle {
                position: hit_box_transform.translation().truncate(),
                size,
                color: Color::rgba(0., 1., 0., 0.1),
                ..Default::default()
            });
        }
    }

    if debug_draw_settings.draw_hurt_boxes {
        for (hurt_box, hurt_box_transform) in hurt_box_query.iter() {
            let size = match hurt_box.shape {
                CollisionShape::None => Vec2::ZERO,
                CollisionShape::Rect(size) => size,
            };
            debug_draw.draw(DebugRectangle {
                position: hurt_box_transform.translation().truncate(),
                size,
                color: Color::rgba(1., 0., 0., 0.2),
                ..Default::default()
            });
        }
    }
}
