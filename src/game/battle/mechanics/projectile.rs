use bevy::prelude::*;

use crate::{DamageReceiveEvent, EventSet, Transform2, UpdateSet};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum ProjectileSystem {
    Update,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            projectile_update
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(ProjectileSystem::Update)
                .in_set(UpdateSet)
                .after(EventSet::<DamageReceiveEvent>::Sender),
        );
    }
}

#[derive(Default, Component)]
pub struct Projectile {
    pub velocity: Vec2,
}

pub fn projectile_update(
    mut projectile_query: Query<(&mut Projectile, &mut Transform2)>,
    time: Res<FixedTime>,
) {
    for (mut projectile, mut projectile_transform) in projectile_query.iter_mut() {
        projectile_transform.translation += projectile.velocity * time.period.as_secs_f32();
        projectile_transform.rotation = Vec2::angle_between(Vec2::X, projectile.velocity);
        projectile.velocity.y -= time.period.as_secs_f32() * 800.;
    }
}
