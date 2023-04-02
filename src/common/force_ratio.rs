use bevy::{prelude::*, transform::TransformSystem};

use crate::Persistent;

const DESIRED_WIDTH: f32 = 1280. * 2.;
const DESIRED_HEIGHT: f32 = 768. * 2.;
const RATIO_BAR_SIZE: f32 = 100_000.;

pub struct ForceRatioPlugin;

impl Plugin for ForceRatioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(force_ratio_setup).add_system(
            force_ratio_update
                .in_base_set(CoreSet::PostUpdate)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

#[derive(Component, Clone, Copy)]
pub enum ForceRatioBar {
    Top,
    Bottom,
    Left,
    Right,
}

impl ForceRatioBar {
    fn translation(self) -> Vec3 {
        match self {
            ForceRatioBar::Top => Vec3::new(0., DESIRED_HEIGHT * 0.5 + RATIO_BAR_SIZE * 0.5, 1.),
            ForceRatioBar::Bottom => {
                Vec3::new(0., DESIRED_HEIGHT * -0.5 - RATIO_BAR_SIZE * 0.5, 1.)
            }
            ForceRatioBar::Left => Vec3::new(DESIRED_WIDTH * -0.5 - RATIO_BAR_SIZE * 0.5, 0., 1.),
            ForceRatioBar::Right => Vec3::new(DESIRED_WIDTH * 0.5 + RATIO_BAR_SIZE * 0.5, 0., 1.),
        }
    }
}

fn force_ratio_setup(mut commands: Commands) {
    for side in [
        ForceRatioBar::Top,
        ForceRatioBar::Bottom,
        ForceRatioBar::Left,
        ForceRatioBar::Right,
    ] {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(RATIO_BAR_SIZE)),
                    color: Color::BLACK,
                    ..Default::default()
                },
                visibility: Visibility::Visible,
                transform: Transform::from_translation(side.translation()),
                ..Default::default()
            },
            Persistent,
            side,
        ));
    }
}

fn force_ratio_update(
    mut transform_query: Query<&mut Transform>,
    camera_query: Query<Entity, With<Camera>>,
    bar_query: Query<(Entity, &ForceRatioBar)>,
    window_query: Query<&Window>,
) {
    if let Ok(window) = window_query.get_single() {
        for camera_entity in camera_query.iter() {
            if let Ok(mut camera_transform) = transform_query.get_mut(camera_entity) {
                let ratio = window.width() / window.height();
                let mut desired_width = DESIRED_WIDTH;
                let mut desired_height = DESIRED_HEIGHT;
                let desired_ratio = desired_width / desired_height;
                if ratio > desired_ratio {
                    desired_width *= ratio / desired_ratio;
                } else {
                    desired_height *= desired_ratio / ratio;
                }
                camera_transform.scale.x = desired_width / window.width();
                camera_transform.scale.y = desired_height / window.height();
            }
        }
    }
    for (bar_entity, bar) in bar_query.iter() {
        if let Ok(mut bar_transform) = transform_query.get_mut(bar_entity) {
            bar_transform.translation = bar.translation();
        }
    }
}
