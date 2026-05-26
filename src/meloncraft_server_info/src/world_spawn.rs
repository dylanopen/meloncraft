//! Module for resource struct [`WorldSpawn`].

use bevy::ecs::resource::Resource;
use bevy::math::IVec3;

/// The global 'world spawn point', where all players who don't have a custom spawnpoint set from a
/// bed will spawn.
#[derive(Resource, Debug, Clone)]
pub struct WorldSpawn {
    /// The block location of the world spawn, as 3 i32s.
    pub location: IVec3,
}

impl WorldSpawn {
    /// Create a new instance of [`WorldSpawn`] with the given `location`.
    #[must_use]
    pub const fn new(location: IVec3) -> WorldSpawn {
        return WorldSpawn { location };
    }
}
