use bevy::ecs::component::Component;

/// Component for marking a world as the primary overworld.
///
/// This is used by the `meloncraft_world_manager` crate to determine which world is the 'main'
/// world to load and save chunks to, among other things.
///
/// You should only have one world with this component in your game.
#[derive(Component)]
pub struct Overworld;

