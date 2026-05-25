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

impl WorldSpawn {
    /// Create a new instance of [`WorldSpawn`] with the given `location`.
    #[must_use]
    pub const fn new(location: DVec3) -> WorldSpawn{
        return WorldSpawn {
            location,
        };
    }
}

