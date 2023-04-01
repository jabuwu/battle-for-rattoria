use bevy::prelude::*;

use crate::{Depth, DepthLayer, Transform2};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum YOrderSystem {
    Update,
}

pub struct YOrderPlugin;

impl Plugin for YOrderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(y_order_update.in_set(YOrderSystem::Update));
    }
}

#[derive(Component)]
pub struct YOrder;

fn y_order_update(mut y_order_query: Query<(&mut Depth, &Transform2), With<YOrder>>) {
    for (mut y_order_depth, y_order_transform) in y_order_query.iter_mut() {
        *y_order_depth = Depth::from(DepthLayer::YOrder(
            0.5 - y_order_transform.translation.y * 0.0001,
        ));
    }
}
