use bevy::prelude::*;

#[derive(Copy, Clone, Default)]
pub enum CollisionShape {
    #[default]
    None,
    Rect {
        offset: Vec2,
        size: Vec2,
    },
}

impl CollisionShape {
    pub fn at(&self, translation: Vec2) -> TranslatedCollisionShape {
        TranslatedCollisionShape {
            translation,
            shape: *self,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TranslatedCollisionShape {
    pub translation: Vec2,
    pub shape: CollisionShape,
}

impl TranslatedCollisionShape {
    pub fn overlaps(&self, other: TranslatedCollisionShape) -> bool {
        match self.shape {
            CollisionShape::None => false,
            CollisionShape::Rect { offset, size } => match other.shape {
                CollisionShape::None => false,
                CollisionShape::Rect {
                    offset: other_offset,
                    size: other_size,
                } => {
                    (self.translation.x + offset.x) - size.x * 0.5
                        <= (other.translation.x + other_offset.x) + other_size.x * 0.5
                        && (self.translation.x + offset.x) + size.x * 0.5
                            >= (other.translation.x + other_offset.x) - other_size.x * 0.5
                        && (self.translation.y + offset.y) - size.y * 0.5
                            <= (other.translation.y + other_offset.y) + other_size.y * 0.5
                        && (self.translation.y + offset.y) + size.y * 0.5
                            >= (other.translation.y + other_offset.y) - other_size.y * 0.5
                }
            },
        }
    }
}
