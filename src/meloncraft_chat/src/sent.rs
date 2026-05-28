use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;

#[derive(Message, Debug, Clone)]
pub struct PlayerSentChatMessage {
    pub sender: Entity,
    pub timestamp: u64,
    pub message: String,
}
