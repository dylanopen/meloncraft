//! Module for struct [`Chunk`].

use crate::biome::Biome;
use crate::block_section::ChunkBlockSection;
use bevy::math::IVec3;
use meloncraft_block::block::Block;

/// A chunk of blocks, which is 16 blocks in width and length, and a variable height (384 blocks in
/// the vanilla overworld).
///
/// This is the main struct for storing blocks in the world. It contains a `Vec<Block>` of all the
/// blocks in the chunk, as well as methods for getting and setting blocks at specific locations,
/// and converting the chunk into the [`ChunkBlockSection`] format used in the chunk data packets.
///
/// See the individual methods for more information.
///
/// This struct cannot be manually constructed. Please use the [`Chunk::new`] function, then the
/// provided methods to interact with the blocks in the chunk.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Chunk {
    /// A `Vec<Block>` of all the blocks in the chunk. 
    ///
    /// ## Block order
    /// The blocks are stored in a specific order, as follows:
    /// - The blocks are stored in **XZY** order, which means that the X coordinate changes the
    ///   fastest, followed by Z, and then Y changing the slowest. You should view the source code
    ///   for the [`Chunk::get_index`] function for more information on how the block locations are
    ///   mapped to the indices in this `Vec<Block>`.
    ///
    /// Library users do not need to interact with this field directly, instead, functions for
    /// safely getting and setting blocks at specific locations are provided, which handle the
    /// indexing and bounds checking for you.
    ///
    /// You can also convert the chunk into the [`ChunkBlockSection`] format used in the chunk data
    /// packets using the [`Chunk::to_chunk_sections`] method, which also handles the indexing and
    /// formatting for you.
    ///
    /// ## Constraints
    /// - The length of this `Vec<Block>` must be a multiple of `16*16*16` (4096), as each chunk
    ///   section is 16x16x16 blocks.
    /// - The height of the chunk is determined by the length of this `Vec<Block>`, divided by `4096`.
    ///   For details, see [`Chunk::get_height`].
    /// - All blocks in the chunk must be valid block states, meaning that their `state_id` field
    ///   must be a valid block state ID as defined in the internal registries.
    /// - The chunk blocks must be stored in the correct order, as defined above.
    blocks: Vec<Block>,
}

impl Chunk {
    /// Creates a new `Chunk` with the specified blocks.
    ///
    /// ## Parameters
    /// - `blocks`: A `Vec<Block>` of all the blocks in the chunk.
    ///
    /// ## Returns
    /// A new [`Chunk`] containing the passed blocks.
    ///
    /// ## `blocks` constraints
    /// - The length of this `Vec<Block>` must be a multiple of `16*16*16` (4096), as each chunk
    ///   section is 16x16x16 blocks.
    /// - The height of the chunk is determined by the length of this `Vec<Block>`, divided by
    ///   `4096`. For details, see [`Chunk::get_height`].
    /// - All [`Block`]s in the chunk must be valid block states, meaning that their `state_id` field
    ///   must be a valid block state ID as defined in the internal registries.
    /// - The chunk blocks must be stored in the correct order.
    ///
    /// ## Block ordering
    /// The blocks are stored in **XZY** order, which means that the X coordinate changes the
    /// fastest, followed by Z, and then Y changing the slowest. You should view the source code
    /// for the [`Chunk::get_index`] function for more information on how the block locations are
    /// mapped to the indices in this `Vec<Block>`.
    ///
    /// ## Example
    /// ```rust
    /// use meloncraft_block::block::Block;
    /// use meloncraft_chunk::chunk::Chunk;
    /// use bevy::math::IVec3;
    /// let blocks = vec![Block::new(1); 16*16*16*4]; // 4 chunk sections of stone blocks
    /// let chunk = Chunk::new(blocks);
    /// assert_eq!(chunk.get_height(), 4);
    /// assert_eq!(chunk.get_block(IVec3::new(0, 0, 0)).unwrap().state_id, 1);
    /// assert_eq!(chunk.get_block(IVec3::new(15, 0, 8)).unwrap().state_id, 1);
    /// ```
    #[must_use]
    pub const fn new(blocks: Vec<Block>) -> Self {
        return Chunk {
            blocks
        };
    }

    /// Get the height, in **number of chunk sections**, of the chunk.
    ///
    /// The number of chunk *sections* represented by the chunk is equal to
    /// the number of blocks, divided by `4096` (16*16*16).
    /// This is because a chunk section is 16 blocks in width and length, and 16 blocks in height,
    /// for a total of 4096 blocks per chunk section.
    ///
    /// This method will return a value 16 times smaller than the [`Chunk::get_height_in_blocks`]
    /// method, which returns the height in blocks (a value 16x larger).
    ///
    /// Examples:
    /// - A chunk with 4096 blocks (1 chunk section) will return `1`.
    /// - A chunk with 8192 blocks (2 chunk sections) will return `2`.
    /// - If the value 6 is returned, that means the chunk has 6 chunk sections, which is 96 blocks
    ///   in height (6 * 16).
    #[must_use]
    pub const fn get_height_in_chunks(&self) -> usize {
        return self.blocks.len() / (16*16*16);
    }

    /// Get the height of the chunk in **number of blocks upwards**.
    /// 
    /// In the default vanilla overworld, this is 384 blocks.
    ///
    /// The number of chunk sections is 16 times this value (as each [`ChunkBlockSection`] is made
    /// up of 16 layers of blocks. You should use the [`Chunk::get_height_in_chunks`] for this
    /// purpose, though.
    ///
    /// This method will return a value 16 times larger than the [`Chunk::get_height_in_chunks`]
    /// method, which returns the height in chunk sections (each chunk section is 16 blocks).
    #[must_use]
    pub const fn get_height_in_blocks(&self) -> usize {
        return self.blocks.len() / (16*16);
    }

    /// Get **a reference** to the [`Block`] at the specified `location` in the chunk.
    /// If you want to edit the block, please use [`Chunk::get_block_mut`] or [`Chunk::set_block`]
    /// instead.
    ///
    /// ## Parameters
    /// - `location`: An [`IVec3`] representing the location of the block in the chunk.
    /// 
    /// ## Returns
    /// - `Some(&Block)` if the `location` is valid and within the bounds of the chunk.
    /// - [`None`] if the `location` is out of bounds, meaning that any of the coordinates are outside
    ///   the ranges of `0..16` for X and Z, or <code>0..[`Chunk::get_height_in_blocks`]</code> for Y.
    #[must_use]
    pub fn get_block(&self, location: IVec3) -> Option<&Block> {
        let index = Chunk::get_index(location);
        return self.blocks.get(index);
    }

    /// Get ** a mutable reference** to the [`Block`], at the specified `location` in the chunk.
    /// If you don't need to edit this block, consider using the [`Chunk::get_block`] method instead
    /// for an *immutable* reference.
    /// To replace the block without checking the old block, you can use [`Chunk::set_block`].
    ///
    /// ## Parameters
    /// - `location`: An [`IVec3`] representing the location of the block in the chunk.
    ///
    /// ## Returns
    /// - `Some(&mut Block)` if the `location` is valid and within the bounds of the chunk.
    /// - [`None`] if the `location` is out of bounds, meaning that any of the coordinates are outside
    ///   the ranges of `0..16` for X and Z, or <code>0..[`Chunk::get_height_in_blocks`]</code> for Y.
    #[must_use]
    pub fn get_block_mut(&mut self, location: IVec3) -> Option<&mut Block> {
        let index = Chunk::get_index(location);
        return self.blocks.get_mut(index);
    }

    /// Get a list of all the blocks in the chunk.
    ///
    /// ## Block order
    /// The blocks are stored in a specific order, as follows:
    /// - The blocks are stored in **XZY** order, which means that the X coordinate changes the
    ///   fastest, followed by Z, and then Y changing the slowest. You should view the source code
    ///   for the [`Chunk::get_index`] function for more information on how the block locations are
    ///   mapped to the indices in this `Vec<Block>`.
    /// - It is recommended to just use [`Chunk::get_index`] instead of doing the calculations
    ///   yourself.
    ///
    /// ## Returns
    /// - A *reference* to a [`Vec`] of [`Block`]s.
    /// - Currently, this field just returns an immutable reference to the [`Chunk::blocks`] private
    ///   field.
    ///
    /// ## Restrictions
    /// While you are free to call this method, it is recommended to instead use the other provided
    /// methods for interacting with chunks and use this as a 'last resort'.
    #[must_use]
    pub const fn get_blocks(&self) -> &Vec<Block> {
        return &self.blocks;
    }

    /// Replace the block in this chunk at the specified `location` with the new `block` argument.
    ///
    /// This will fully replace the [`Block`] object internally, not just copy fields.
    ///
    /// ## Parameters
    /// - `&mut self`: the [`Chunk`] to operate on.
    /// - `location`: the position of the block, as a Bevy [`IVec3`]
    /// - `block`: the new [`Block`] to put at that position.
    ///
    /// ## Returns
    /// - `()` if the block was set sucessfully.
    /// - `None` if the block could not be set, for example, because the `location` was out of
    ///   bounds for the chunk.
    pub fn set_block(&mut self, location: IVec3, block: Block) -> Option<()> {
        let index = Chunk::get_index(location); 
        let b = self.blocks.get_mut(index)?;
        *b = block;
        return Some(());
    }

    /// Check whether a chunk contains only air blocks.
    ///
    /// ## Returns
    /// - `true` if the chunk has only air blocks and no other type of block
    /// - `false` if the chunk has other type(s) of blocks, and not just air.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        return self.blocks.iter().all(|b| return b.state_id == 0); // 0 is air, I'm not importing blockstate_registry to tell us that.
    }

    /// Convert the [`Chunk`] to a [`Vec`] of [`ChunkBlockSection`]s.
    /// [`ChunkBlockSection`]s store a 16x16x16 cube of blocks, and each chunk can be represented by
    /// a collection of these chunk sections.
    ///
    /// The number of items this returns is the same as the number of chunk sections returned by
    /// [`Chunk::get_height_in_chunks`].
    ///
    /// [`ChunkBlockSection`]s are the main way chunks are represented in the protocol, so this
    /// function will probably be called at some point if you are sending these chunks over the
    /// network.
    #[must_use]
    pub fn to_chunk_sections(&self) -> Vec<ChunkBlockSection> {
        let mut sections = Vec::new();
        let biomes = [Biome::new(40); 64]; // temporarily set all biomes to plains
        #[expect(clippy::indexing_slicing, reason = "Bounds are already manually checked, by iterating only over self.get_height()")]
        for i in 0..self.get_height_in_chunks() {
            let mut section_blocks = [Block::new(0); 4096];
            section_blocks.copy_from_slice(&self.blocks[i*16*16*16..(i+1)*16*16*16]);
            let section = ChunkBlockSection::new(section_blocks, biomes);
            sections.push(section);
        }
        return sections;
    }

    pub fn get_index(location: IVec3) -> usize {
        return usize::try_from(location.y * 16*16 + location.z * 16 + location.x).unwrap();
    }
}
