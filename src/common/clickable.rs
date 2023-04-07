use bevy::prelude::*;

use crate::{CollisionShape, Cursor};

pub struct ClickablePlugin;

impl Plugin for ClickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(clickable_update);
    }
}

#[derive(Component, Default)]
pub struct Clickable {
    pub shape: CollisionShape,
    pub use_global: bool,

    pub disabled: bool,

    pub hovered: bool,
    pub last_hovered: bool,

    pub clicked: bool,
    pub last_clicked: bool,

    pub confirmed: bool,
}

impl Clickable {
    pub fn new(shape: CollisionShape) -> Self {
        Self {
            shape,
            ..Default::default()
        }
    }

    pub fn just_hovered(&self) -> bool {
        return self.hovered && !self.last_hovered;
    }

    pub fn just_clicked(&self) -> bool {
        return self.clicked && !self.last_clicked;
    }

    pub fn just_released(&self) -> bool {
        return !self.clicked && self.last_clicked;
    }
}

fn clickable_update(
    mut query: Query<(&mut Clickable, &GlobalTransform)>,
    cursor: Res<Cursor>,
    input: Res<Input<MouseButton>>,
) {
    for (mut clickable, transform) in query.iter_mut() {
        let (scale, _, _) = transform.to_scale_rotation_translation();
        let shape = match clickable.shape {
            CollisionShape::None => CollisionShape::None,
            CollisionShape::Point { .. } => CollisionShape::None,
            CollisionShape::Rect { offset, size } => CollisionShape::Rect {
                offset,
                size: size * scale.truncate(),
            },
        };
        clickable.last_hovered = clickable.hovered;
        clickable.last_clicked = clickable.clicked;
        clickable.confirmed = false;
        clickable.hovered = shape
            .at(transform.translation().truncate())
            .overlaps(CollisionShape::Point { offset: Vec2::ZERO }.at(cursor.position));
        if clickable.hovered && input.just_pressed(MouseButton::Left) {
            clickable.clicked = true;
        }
        if clickable.clicked && input.just_released(MouseButton::Left) {
            clickable.clicked = false;
            if clickable.hovered {
                clickable.confirmed = true;
            }
        }
    }
}
