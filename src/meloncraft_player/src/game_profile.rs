//! Module for structs [`GameProfile`] and [`GameProfileProperties`].

use crate::Uuid;
use bevy::prelude::Component;

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
