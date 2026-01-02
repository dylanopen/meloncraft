use bevy::ecs::component::Component;

#[derive(Component, Debug)]
pub struct Uuid(pub u128);
