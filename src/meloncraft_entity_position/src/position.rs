use bevy::ecs::component::Component;
use crate::EntityPositionFlags;

#[derive(Component, Debug, Clone)]
pub struct EntityPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub flags: EntityPositionFlags,
}