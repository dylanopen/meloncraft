use bevy::ecs::component::Component;

#[derive(Component, Debug, Clone)]
pub struct Uuid(pub u128);
