use bevy::prelude::Component;
use crate::position::EntityPosition;

#[derive(Debug, Clone, Component)]
pub struct LastEntityPosition(pub EntityPosition);