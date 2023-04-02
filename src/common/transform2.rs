use bevy::{prelude::*, transform::TransformSystem};
use lerp::Lerp;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum Transform2System {
    Transform2Propagate,
}

pub struct Transform2Plugin;

impl Plugin for Transform2Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (transform2_propagate,)
                .in_base_set(CoreSet::PostUpdate)
                .in_set(Transform2System::Transform2Propagate)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

#[derive(Clone, Component, Copy, Debug)]
pub struct Transform2 {
    pub translation: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Transform2 {
    pub const IDENTITY: Self = Transform2 {
        translation: Vec2::ZERO,
        rotation: 0.,
        scale: Vec2::ONE,
    };

    pub const fn from_xy(x: f32, y: f32) -> Self {
        Self::from_translation(Vec2::new(x, y))
    }

    pub const fn from_translation(translation: Vec2) -> Self {
        Transform2 {
            translation,
            ..Self::IDENTITY
        }
    }

    pub const fn from_rotation(rotation: f32) -> Self {
        Transform2 {
            rotation,
            ..Self::IDENTITY
        }
    }

    pub const fn from_scale(scale: Vec2) -> Self {
        Transform2 {
            scale,
            ..Self::IDENTITY
        }
    }

    pub const fn with_translation(mut self, translation: Vec2) -> Self {
        self.translation = translation;
        self
    }

    pub const fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub const fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }
}

impl Default for Transform2 {
    fn default() -> Self {
        Transform2::IDENTITY
    }
}

#[derive(Clone, Component, Copy, Debug, PartialEq)]
pub enum Depth {
    Exact(f32),
    Inherit(f32),
}

pub fn transform2_propagate(
    mut transform_query: Query<(&mut Transform, Option<&Transform2>, Option<&Depth>)>,
    root_query: Query<Entity, Without<Parent>>,
    children_query: Query<&Children>,
) {
    for root_entity in root_query.iter() {
        update_transform2_recursive(root_entity, &children_query, &mut transform_query, 0.);
    }
}

fn update_transform2_recursive(
    entity: Entity,
    children_query: &Query<&Children>,
    transform_query: &mut Query<(&mut Transform, Option<&Transform2>, Option<&Depth>)>,
    mut cumulative_depth: f32,
) {
    if let Ok((mut transform, transform2, depth)) = transform_query.get_mut(entity) {
        if let Some(transform2) = transform2 {
            transform.translation.x = transform2.translation.x;
            transform.translation.y = transform2.translation.y;
            transform.scale = transform2.scale.extend(1.);
            transform.rotation = Quat::from_rotation_z(transform2.rotation);
        }
        if let Some(depth) = depth {
            transform.translation.z = match depth {
                Depth::Exact(depth_value) => *depth_value - cumulative_depth,
                Depth::Inherit(depth_value) => *depth_value,
            };
        }
        cumulative_depth += transform.translation.z;
    }
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            update_transform2_recursive(*child, children_query, transform_query, cumulative_depth);
        }
    }
}

pub enum DepthLayer {
    Back(f32),
    YOrder(f32),
    Foreground(f32),
    Front(f32),
}

impl From<DepthLayer> for Depth {
    fn from(depth_layer: DepthLayer) -> Self {
        match depth_layer {
            DepthLayer::Back(x) => Depth::Exact(0.01_f32.lerp(0.29, x)),
            DepthLayer::YOrder(x) => Depth::Exact(0.3_f32.lerp(0.69, x)),
            DepthLayer::Foreground(x) => Depth::Exact(0.7_f32.lerp(0.89, x)),
            DepthLayer::Front(x) => Depth::Exact(0.9_f32.lerp(0.99, x)),
        }
    }
}
