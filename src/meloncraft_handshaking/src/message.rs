use bevy::ecs::message::Message;

#[derive(Message, Clone, Debug)]
pub struct StatusHandshakeReceived;