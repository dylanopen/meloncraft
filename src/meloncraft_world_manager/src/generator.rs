//! Systems for world generation and creation.

use bevy::ecs::system::Commands;
use meloncraft_world::world::World;

use crate::marker::Overworld;

/// Spawns a new entity with the components:
/// - [`World`], a completely empty world with no chunks
/// - [`Overworld`], a no-field marker struct to indicate this is the main overworld.
pub fn generate_empty_overworld(
    mut commands: Commands,
) {
    commands.spawn((
        World::default(),
        Overworld,
    ));
}
