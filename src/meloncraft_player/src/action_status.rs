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

    /// **Protocol ID: `2`**.
    /// A [`PlayerActionStatus`] indicating that a player has *finished* breaking a block. This is
    /// sent when a player successfully breaks a block, by holding down the mouse button
    /// until the block is fully broken.
    /// *It is not sent when a block is instamined.* In that case, only a
    /// [`PlayerActionStatus::StartedDigging`] is sent. You should treat a `StartedDigging` as a
    /// `FinishedDigging` if the block they dug can be instamined, since the player has actually
    /// finished digging the block, as soon as they send the `StartedDigging` packet.
    /// Not sent when a player cancels digging a block. See [`PlayerActionStatus::CancelledDigging`]
    /// for that case.
    FinishedDigging,

    /// **Protocol ID: `3`**.
    /// A [`PlayerActionStatus`] indicating that a player has dropped an entire stack of items from
    /// their inventory. This is sent when a player presses the drop key (default `Q`) while
    /// also holding left control (tbc), in order to drop the entire stack of items they are
    /// currently holding.
    ///
    /// Not sent when dropping a single item. See [`PlayerActionStatus::DropSingleItem`] for that.
    DropItemStack,

    /// **Protocol ID: `4`**.
    /// A [`PlayerActionStatus`] indicating that a player has dropped a single item from their
    /// inventory. This is sent when a player presses the drop key (default `Q`) without holding
    /// left control, in order to drop a single item from the stack of items they are currently
    /// holding.
    ///
    /// Not sent when dropping an entire stack of items. See [`PlayerActionStatus::DropItemStack`].
    DropSingleItem,
    
    /// **Protocol ID: `5`**.
    /// A [`PlayerActionStatus`] indicating that a player has updated the item they are currently
    /// holding. This is sent when a player changes the item they are holding in their hand, by
    /// scrolling the mouse wheel or pressing the number keys to switch hotbar slots.
    /// It doesn't seem like this is when they move around their inventory (tbc) but only when they
    /// switch to a different hotbar slot.
    UpdateHeldItem,

    /// **Protocol ID: `6`**.
    /// A [`PlayerActionStatus`] indicating that a player has swapped the item in their mainhand
    /// with the item in their offhand. This is sent when a player presses the key to swap their
    /// mainhand with their offhand, or potentially (tbc) when they drag an item from their mainhand
    /// to their offhand in their inventory.
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
