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

    /// The numerical ID that the Minecraft client understands as the block's state.
    /// This ID is unique to every block state type. You can see the generated crate
    /// `meloncraft_blockstate_registry` for more information about the blockstate IDs.
    ///
    /// Use the [`Block::new`] constructor to create a new `Block` with a specific `state_id`.
    /// 
    /// ## Packet usage
    /// Packets should use this field, which is a unique ID as defined in the internal Minecraft
    /// block state registry.
    /// You can get a specific block state ID by adding the external `meloncraft_blockstate_registry`
    /// crate. Due to the heaviness and slow compilation time of that crate, it is not used internally
    /// by Meloncraft, but you are welcome to use it in your own code to make working with block states
    /// easier.
    pub state_id: i32,
}

impl Block {
    #[must_use]
    /// Creates a new [`Block`] with the specified `state_id`.
    /// ## Parameters
    /// - `state_id`: The numerical ID that the Minecraft client understands as the block's state.
    ///   This ID is unique to every block state type. Look at the `meloncraft_blockstate_registry`
    ///   crate for more information about the blockstate IDs.
    /// - This function does not check if the `state_id` is valid, so make sure to use a valid block
    ///   state ID, otherwise the vanilla client may *CRASH* when it receives a packet with an invalid
    ///   block state ID.
    ///
    /// ## Returns
    /// A new `Block` object, wrapping the specified state ID.
    /// *This will be returned even if the block state is invalid*.
    ///
    /// ## Example
    /// ```rust
    /// use meloncraft_block::block::Block;
    /// let stone_block = Block::new(1); // Stone has blockstate ID `1`
    /// let dirt_block = Block::new(2); // Dirt has blockstate ID `2`
    /// assert_eq!(stone_block.state_id, 1);
    /// assert_eq!(dirt_block.state_id, 2);
    /// assert_ne!(stone_block, dirt_block);
    /// ```
    pub const fn new(state_id: i32) -> Self {
        return Block { state_id };
    }
}
