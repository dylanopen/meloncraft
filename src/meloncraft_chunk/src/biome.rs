//! Module for struct [`Biome`].

/// Represents a biome in the game world.
/// This doesn't store the location or anything else outside of the biome's state ID. It simply
/// stores its current state, represented by an `i32` value.
///
/// The location or any extra data about a biome should be stored by whatever
/// struct contains this [`Block`], e.g. a chunk-related struct.
///
/// ## Packet usage
/// Packets should use the [`Biome::state_id`] field, which is a unique ID as defined in the
/// Minecraft biome registry that is sent to the client in a `RegistryData` packet.
///
/// ## Equality
/// Two `Biome`s are considered equal if they have the same `state_id`. This means that two `Biome`s
/// with the same `state_id` but different locations or other properties are still considered equal.
/// This is because the `Biome` struct is only meant to represent the biome's state, not its
/// location or other properties.
///
/// ## Constraints
/// - The `state_id` field must be a valid biome state ID as defined in the sent Minecraft biome
///   registry. Using an invalid `state_id` **will** likely cause the vanilla client to *CRASH*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Biome {
    pub state_id: i32,
}

impl Biome {
    #[must_use]
    pub const fn new(state_id: i32) -> Self {
        return Self { state_id };
    }
}
