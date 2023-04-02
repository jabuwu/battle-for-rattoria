use bevy::prelude::*;

use crate::{DamageReceiveEvent, EventSet, UpdateSet};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum HealthSystem {
    ReceiveDamage,
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HealthDieEvent>().add_system(
            health_receive_damage
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(HealthSystem::ReceiveDamage)
                .in_set(UpdateSet)
                .after(EventSet::<DamageReceiveEvent>::Sender),
        );
    }
}

#[derive(Default, Component)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { max, current: max }
    }
}

pub struct HealthDieEvent {
    pub entity: Entity,
    _private: (),
}

pub fn health_receive_damage(
    mut damage_receive_events: EventReader<DamageReceiveEvent>,
    mut health_query: Query<&mut Health>,
    mut health_die_events: EventWriter<HealthDieEvent>,
) {
    for damage_receive_event in damage_receive_events.iter() {
        if let Ok(mut health) = health_query.get_mut(damage_receive_event.entity) {
            if health.current > 0. {
                health.current -= damage_receive_event.damage;
                if health.current <= 0. {
                    health_die_events.send(HealthDieEvent {
                        entity: damage_receive_event.entity,
                        _private: (),
                    });
                    health.current = 0.;
                }
            }
        }
    }
}
