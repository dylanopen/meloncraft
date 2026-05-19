//! Module for struct [`Block`].

/// Represents a block in the game world.
/// This doesn't store the location or anything else outside of the block's state. It simply stores
/// its current state, represented by an `i32` value.
///
/// The location or any extra data about a block should be stored by the `Chunk` struct, or whatever
/// struct contains this [`Block`].
///
/// ## Packet usage
/// Packets should use the [`Block::state_id`] field, which is a unique ID as defined in the internal
/// Minecraft block state registry.
/// You can get a specific block state ID by adding the external `meloncraft_blockstate_registry`
/// crate. Due to the heaviness and slow compilation time of that crate, it is not used internally
/// by Meloncraft, but you are welcome to use it in your own code to make working with block states
/// easier.
///
/// ## Equality
/// Two `Block`s are considered equal if they have the same `state_id`. This means that two `Block`s
/// with the same `state_id` but different locations or other properties are still considered equal.
/// This is because the `Block` struct is only meant to represent the block's state, not its
/// location or other properties.
///
/// ## Constraints
/// - The `state_id` field must be a valid block state ID as defined in the internal Minecraft block
///   state registry. Using an invalid `state_id` **will** likely cause the vanilla client to
///   *CRASH*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block {
    pub state_id: i32,
}

impl Block {
    #[must_use]
    pub const fn new(state_id: i32) -> Self {
        return Block { state_id };
    }
}
