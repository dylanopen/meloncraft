//! Module for enum component [`MainHand`].

use bevy::prelude::Component;

/// A player component storing the player's preference as to whether they use their left or right
/// hand as the main hand.
///
/// Clients can choose which arm they want to use primarily, either left, or right (default).
///
/// This is likely used when forwarding a swing arm packet to all other players, and possibly
/// others, such as broadcasting the hand a player holds the mainhand item in.
///
/// See the individual variants for the protocol IDs of each hand:
/// - [`MainHand::Left`]
/// - [`MainHand::Right`]
#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(i32)]
pub enum MainHand {

    /// Indicates that a player's main hand/arm is their *left* hand/arm.
    /// This is the hand they swing and hold mainhand items in. See [`MainHand`] for more info.
    /// This is not the default option. The default main hand in Minecraft is [`MainHand::Right`].
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

