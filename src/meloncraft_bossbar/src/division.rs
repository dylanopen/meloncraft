//! Module for component enum [`BossbarDivision`].

use bevy::ecs::component::Component;

/// Component enum representing the number of divisions a bossbar can be.
/// *There are set values to this, see the variants*.
///
/// This basically determines how many 'notches' (separators) are in the bossbar.
#[derive(Component, Debug, Clone)]
pub enum BossbarDivision {
    NoDivision,
    DividedInto6,
    DividedInto10,
    DividedInto12,
    DividedInto20,
}

impl TryFrom<i32> for BossbarDivision {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(BossbarDivision::NoDivision),
            1 => Ok(BossbarDivision::DividedInto6),
            2 => Ok(BossbarDivision::DividedInto10),
            3 => Ok(BossbarDivision::DividedInto12),
            4 => Ok(BossbarDivision::DividedInto20),
            _ => Err(()),
        };
    }
}

impl From<BossbarDivision> for i32 {
    fn from(value: BossbarDivision) -> Self {
        return match value {
            BossbarDivision::NoDivision => 0,
            BossbarDivision::DividedInto6 => 1,
            BossbarDivision::DividedInto10 => 2,
            BossbarDivision::DividedInto12 => 3,
            BossbarDivision::DividedInto20 => 4,
        };
    }
}
