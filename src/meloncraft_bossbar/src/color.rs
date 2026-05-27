//! Module for component enum [`BossbarColor`].

use bevy::ecs::component::Component;

/// Component to add to a bossbar entity, representing its **color**.
/// This can only be one of 7 variants: see the values below.
#[derive(Component, Debug, Clone)]
pub enum BossbarColor {
    Pink,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

impl TryFrom<i32> for BossbarColor {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(BossbarColor::Pink),
            1 => Ok(BossbarColor::Blue),
            2 => Ok(BossbarColor::Red),
            3 => Ok(BossbarColor::Green),
            4 => Ok(BossbarColor::Yellow),
            5 => Ok(BossbarColor::Purple),
            6 => Ok(BossbarColor::White),
            _ => Err(()),
        };
    }
}

impl From<BossbarColor> for i32 {
    fn from(value: BossbarColor) -> Self {
        return match value {
            BossbarColor::Pink => 0,
            BossbarColor::Blue => 1,
            BossbarColor::Red => 2,
            BossbarColor::Green => 3,
            BossbarColor::Yellow => 4,
            BossbarColor::Purple => 5,
            BossbarColor::White => 6,
        };
    }
}
