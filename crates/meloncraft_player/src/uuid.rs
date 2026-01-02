use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Uuid(pub u128);
