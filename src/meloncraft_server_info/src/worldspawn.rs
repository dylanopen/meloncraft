//! Module for resource struct [`WorldSpawn`].

use bevy::ecs::resource::Resource;
use bevy::math::DVec3;

/// The global 'world spawn point', where all players who don't have a custom spawnpoint set from a
/// bed will spawn.
#[derive(Resource, Debug, Clone)]
pub struct WorldSpawn {

    /// The entity location of the world spawn.
    pub location: DVec3,
}
