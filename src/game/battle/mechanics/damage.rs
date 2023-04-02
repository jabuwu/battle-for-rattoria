use bevy::prelude::*;
use bitflags::bitflags;

use crate::{CollisionShape, EventSet};

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

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum DamageSystem {
    Update,
    Events,
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
                    .in_set(EventSet::<DamageInflictEvent>::Sender),
            )
            .add_system(
                damage_events
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(DamageSystem::Events)
                    .in_set(EventSet::<DamageReceiveEvent>::Sender)
                    .after(EventSet::<DamageInflictEvent>::Sender),
            );
    }
}

#[derive(Default, Component)]
pub struct HitBox {
    pub flags: DamageFlags,
    pub shape: CollisionShape,
}

#[derive(Default, Component)]
pub struct HurtBox {
    pub flags: DamageFlags,
    pub shape: CollisionShape,
    pub damage: f32,
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
    hurtbox_query: Query<(Entity, &HurtBox)>,
    hitbox_query: Query<(Entity, &HitBox)>,
    transform_query: Query<&GlobalTransform>,
) {
    for (hurtbox_entity, hurtbox) in hurtbox_query.iter() {
        let Ok(hurtbox_transform) = transform_query.get(hurtbox_entity) else {
            continue;
        };
        for (hitbox_entity, hitbox) in hitbox_query.iter() {
            if hurtbox_entity == hitbox_entity {
                continue;
            }
            let Ok(hitbox_transform) = transform_query.get(hitbox_entity) else {
                continue;
            };
            if hurtbox
                .shape
                .at(hurtbox_transform.translation().truncate())
                .overlaps(hitbox.shape.at(hitbox_transform.translation().truncate()))
                && hurtbox.flags & hitbox.flags != DamageFlags::empty()
            {
                damage_inflict_events.send(DamageInflictEvent {
                    entity: hitbox_entity,
                    damage: 1.,
                });
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
