use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_player::GameProfile;

#[derive(Message, Debug, Clone)]
pub struct OfflineLoginStarted {
    pub client: Entity,
    pub profile: GameProfile,
}