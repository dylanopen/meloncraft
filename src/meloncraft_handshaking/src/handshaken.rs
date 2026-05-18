use bevy::ecs::message::Message;
use bevy::prelude::Entity;

#[derive(Message, Clone, Debug)]
pub struct StatusHandshaken {
    pub player: Entity,
}

#[derive(Message, Clone, Debug)]
pub struct LoginHandshaken {
    pub player: Entity,
}

#[derive(Message, Clone, Debug)]
pub struct TransferHandshaken {
    pub player: Entity,
}