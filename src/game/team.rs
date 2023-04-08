use enum_map::Enum;
use strum_macros::EnumIter;

use crate::DamageFlags;

#[derive(Copy, Clone, PartialEq, Eq, EnumIter, Hash, Enum)]
pub enum Team {
    Friendly,
    Enemy,
}

impl Team {
    pub fn hit_flags(&self) -> DamageFlags {
        match self {
            Self::Friendly => DamageFlags::FRIENDLY,
            Self::Enemy => DamageFlags::ENEMY,
        }
    }

    pub fn hurt_flags(&self) -> DamageFlags {
        match self {
            Self::Friendly => DamageFlags::ENEMY,
            Self::Enemy => DamageFlags::FRIENDLY,
        }
    }

    pub fn move_direction(&self) -> f32 {
        match self {
            Self::Friendly => 1.,
            Self::Enemy => -1.,
        }
    }

    pub fn opposite_team(&self) -> Team {
        match self {
            Self::Friendly => Self::Enemy,
            Self::Enemy => Self::Friendly,
        }
    }
}
