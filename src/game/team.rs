use crate::DamageFlags;

#[derive(Copy, Clone, PartialEq, Eq)]
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
}
