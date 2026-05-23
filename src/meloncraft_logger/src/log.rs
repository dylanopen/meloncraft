use bevy::ecs::message::Message;

use crate::level::LogLevel;


#[derive(Message)]
pub struct Log {
    pub level: LogLevel,
    pub source: String,
    pub message: String,
}
