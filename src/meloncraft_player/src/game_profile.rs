//! Module for structs [`GameProfile`] and [`GameProfileProperties`].

use crate::Uuid;
use bevy::prelude::Component;

/// Component storing a player's game profile, including their:
/// - UUID ([`GameProfile::uuid`])
/// - Username ([`GameProfile::username`])
///
/// This is used to store information **identifying** the player. It should be attached to a player
/// entity as soon as you identify their username and UUID.
#[derive(Component, Debug, Clone)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct GameProfileProperties {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}
