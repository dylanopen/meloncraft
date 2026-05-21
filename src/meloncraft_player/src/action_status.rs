//! Module for enum [`PlayerActionStatus`].

/// An enum representing the various types of player actions that can be sent in a
/// `ServerboundPlayerAction` packet.
/// 
/// This includes actions such as starting or finishing digging, dropping items, updating the held
/// item, and swapping the offhand item, among many other things.
///
/// ## Conversions
/// You can convert this to and from a `u8` or `i32` using the `From` and `TryFrom` traits, which is
/// useful for converting to and from the raw values used in the Minecraft protocol. The mapping of
/// enum variants to `u8` values can be found in the documentation for the individual variants of
/// this struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerActionStatus {

    /// **Protocol ID: `0`**.
    /// A [`PlayerActionStatus`] indicating that a player has *started* to break a block.
    /// This may be the only packet sent for this block, if the block can be instamined. In some
    /// cases, you should probably treat this as a [`PlayerActionStatus::FinishedDigging`], if the
    /// block they dug can be instamined.
    StartedDigging,

    /// **Protocol ID: `1`**.
    /// A [`PlayerActionStatus`] indicating that a player has *cancelled* breaking a block. This is
    /// sent when a player starts to break a block but then stops before the block is fully broken,
    /// e.g. by releasing the mouse button or moving out of range.
    /// This is not sent when a player finishes breaking a block. See [`PlayerActionStatus::StartedDigging`]
    /// and [`PlayerActionStatus::FinishedDigging`] for that case.
    CancelledDigging,
    FinishedDigging,
    DropItemStack,
    DropSingleItem,
    UpdateHeldItem,
    SwapOffhand,
}

impl TryFrom<u8> for PlayerActionStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(Self::StartedDigging),
            1 => Ok(Self::CancelledDigging),
            2 => Ok(Self::FinishedDigging),
            3 => Ok(Self::DropItemStack),
            4 => Ok(Self::DropSingleItem),
            5 => Ok(Self::UpdateHeldItem),
            6 => Ok(Self::SwapOffhand),
            _ => Err(()),
        }
    }
}

impl From<PlayerActionStatus> for u8 {
    fn from(value: PlayerActionStatus) -> Self {
        return match value {
            PlayerActionStatus::StartedDigging => 0,
            PlayerActionStatus::CancelledDigging => 1,
            PlayerActionStatus::FinishedDigging => 2,
            PlayerActionStatus::DropItemStack => 3,
            PlayerActionStatus::DropSingleItem => 4,
            PlayerActionStatus::UpdateHeldItem => 5,
            PlayerActionStatus::SwapOffhand => 6,
        }
    }
}

impl From<PlayerActionStatus> for i32 {
    fn from(value: PlayerActionStatus) -> Self {
        return u8::from(value).into();
    }
}

impl TryFrom<i32> for PlayerActionStatus {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        return Self::try_from(u8::try_from(value).map_err(|_| ())?);
    }
}
