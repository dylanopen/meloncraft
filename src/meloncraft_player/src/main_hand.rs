//! Module for enum component [`MainHand`].

use bevy::prelude::Component;

/// A player component storing the player's preference as to whether they use their left or right
/// hand as the main hand.
///
/// Clients can choose which arm they want to use primarily, either left, or right (default).
///
/// This is likely used when forwarding a swing arm packet to all other players, and possibly
/// others.
///
/// See the individual variants for the protocol IDs of each hand:
/// - [`MainHand::Left`]
/// - [`MainHand::Right`]
#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(i32)]
pub enum MainHand {
    Left,
    Right,
}

impl TryFrom<i32> for MainHand {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(MainHand::Left),
            1 => Ok(MainHand::Right),
            _ => Err(()),
        };
    }
}

impl From<MainHand> for i32 {
    fn from(value: MainHand) -> Self {
        return match value {
            MainHand::Left => 0,
            MainHand::Right => 1,
        }
    }
}

