use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;
use meloncraft_nbt::NbtTag;

use crate::GameProfileProperties;

/// Message to indicate that a player's state has changed and should be updated, by a
/// `ClientboundPlayerInfoUpdate` packet being sent to *all* clients.
#[derive(Debug, Clone, Message)]
pub struct UpdateClientPlayerAction {

    /// The player whose state has changed and should be updated.
    /// For example, if a player has just joined the game, this would be the Entity representing
    /// that player.
    /// It should be broadcast to *all clients*.
    pub player: Entity,

    /// The [`ClientPlayerAction`] that should be performed on the player, indicating what state has
    /// changed.
    /// For example, if a player has just joined the game, this would be
    /// `ClientPlayerAction::AddPlayer`, and the `AddPlayerAction` would include the player's
    /// username and game profile properties.
    pub action: ClientPlayerAction,
}

#[derive(Debug, Clone)]
pub enum ClientPlayerAction {
    AddPlayer(AddPlayerAction),
    InitializeChat(Option<InitializeChatAction>),
    UpdateGameMode(i32),
    UpdateListed(bool),
    UpdateLatency(i32),
    UpdateDisplayName(Option<NbtTag>),
    UpdateListPriority(i32),
    UpdateHat(bool),
}

impl ClientPlayerAction {
    #[must_use]
    pub const fn mask(&self) -> u8 {
        return match self {
            ClientPlayerAction::AddPlayer(_) => 0x01,
            ClientPlayerAction::InitializeChat(_) => 0x02,
            ClientPlayerAction::UpdateGameMode(_) => 0x04,
            ClientPlayerAction::UpdateListed(_) => 0x08,
            ClientPlayerAction::UpdateLatency(_) => 0x10,
            ClientPlayerAction::UpdateDisplayName(_) => 0x20,
            ClientPlayerAction::UpdateListPriority(_) => 0x40,
            ClientPlayerAction::UpdateHat(_) => 0x80,
        };
    }
}

#[derive(Debug, Clone)]
pub struct AddPlayerAction {
    pub name: String,
    pub game_profile_properties: Vec<GameProfileProperties>,
}

#[derive(Debug, Clone)]
pub struct InitializeChatAction {
    pub session_id: u128,
    pub public_key_expiry_time: i64,
    pub encoded_public_key: Vec<u8>,
    pub public_key_signature: Vec<u8>,
}

