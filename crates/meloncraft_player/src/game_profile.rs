use crate::Uuid;
use bevy::prelude::Component;

#[derive(Component, Debug, Clone)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub username: String,
}

pub struct GameProfileProperties {
    pub _name: String,
    pub _value: String,
    pub _signature: Option<String>,
}
