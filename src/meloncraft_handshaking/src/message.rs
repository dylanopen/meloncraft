use bevy::ecs::message::Message;

#[derive(Message, Clone, Debug)]
pub struct StatusHandshakeReceived;

#[derive(Message, Clone, Debug)]
pub struct LoginHandshakeReceived;

#[derive(Message, Clone, Debug)]
pub struct TransferHandshakeReceived;