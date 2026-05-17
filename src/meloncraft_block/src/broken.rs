use bevy::ecs::message::Message;
use bevy::math::IVec3;

#[derive(Message, Debug, Clone)]
pub struct BlockBroken {
    pub block_location: IVec3,
}
