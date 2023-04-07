use crate::DepthLayer;

pub const DEPTH_BATTLEFIELD_BACKGROUND: DepthLayer = DepthLayer::Back(0.);
pub const DEPTH_BATTLE_SPLASH: DepthLayer = DepthLayer::Front(0.1);

pub const DEPTH_BLOOD_FX: DepthLayer = DepthLayer::Foreground(0.);
pub const DEPTH_PROJECTILE: DepthLayer = DepthLayer::Foreground(0.1);
