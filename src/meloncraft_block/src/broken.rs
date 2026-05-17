use bevy::ecs::message::Message;

#[derive(Message, Debug, Clone)]
pub struct BlockBroken {
    pub block_x: i32,
    pub block_y: i32,
    pub block_z: i32,
    pub sequence: i32,
}
