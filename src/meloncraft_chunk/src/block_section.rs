//! Module for [`ChunkBlockSection`] struct.

use crate::biome::Biome;
use meloncraft_block::block::Block;

/// Represents a single, immutable, 16x16x16 cube of blocks and biomes.
///
/// This struct should be a 'final product'. You probably shouldn't store it anywhere, instead, you
/// should use another struct for storage and eventually convert to chunksections when you need to
/// talk to the client.
///
/// ## Packet usage
/// [`ChunkBlockSection`]s are the main way chunk data is sent to clients over the network. Most
/// chunk wrapping structs will provide methods for converting to \[a list of\]
/// [`ChunkBlockSection`]\[s\].
///
/// ## Mutation
/// - You **should not manually mutate** the fields in this struct. They are intended to be
///   read-only.
/// - Mutation will mean that the `block_count` becomes out of date unless you manually update it
///   when changing a block.
#[derive(Debug, Clone)]
pub struct ChunkBlockSection {
    /// The number of non-air blocks in the [`ChunkBlockSection`].
    /// This is automatically calculated when calling [`ChunkBlockSection::new`].
    /// See [`sum_non_air_blocks`] for more information on how it is calculated.
    ///
    /// ## Packet usage
    /// The block count is required in some clientbound packets, so it is calculated and stored
    /// here.
    pub block_count: i16,

    /// A set-length array of [`Block`]s in the chunk.
    /// As a chunk section is always `16x16x16=4096` blocks, that is the length of the array and it
    /// cannot be resized.
    /// This is stored when calling [`ChunkBlockSection::new`].
    pub blocks: [Block; 4096],

    /// A set-length array of [`Biome`]s in the chunk.
    /// Each biome entry in this field takes up a 4x4x4 cube area. For example, the biome at index
    /// `2` is the biome strecthing from x=8-11, y=0-3 and z=0-3.
    /// As a chunk section is always `16x16x16` blocks, each section has `4x4x4=64` biome entries.
    /// This is stored when calling [`ChunkBlockSection::new`].
    pub biomes: [Biome; 64],
}

impl ChunkBlockSection {
    /// Create a new [`ChunkBlockSection`] with the provided `blocks` and `biomes`.
    ///
    /// This function calculates the `block_count` (see [`ChunkBlockSection::block_count`])
    /// automatically.
    ///
    /// ## Parameters
    /// - `blocks`: an array of [`Block`]s, length `4096`, one for each block in the 16x16x16 cube
    ///   chunk section. *Their IDs are not verified to be valid.*
    /// - `biomes`: an array of [`Biome`]s, length `64`, one for each 4x4x4 region in the 16x16x16 cubic
    ///   chunk section. *Their IDs are also not verified to be valid.*
    ///
    /// Again, please note: there is no verification that the [`Block`]s or [`Biome`]s passed in
    /// have valid state IDs. You should check this yourself.
    ///
    /// ## Returns
    /// - A new [`ChunkBlockSection`] with the provided `blocks` and `biomes`, and an autocalculated
    ///   `block_count`.
    #[must_use]
    pub fn new(blocks: [Block; 4096], biomes: [Biome; 64]) -> Self {
        let block_count = sum_non_air_blocks(&blocks);
        return ChunkBlockSection {
            block_count,
            blocks,
            biomes,
        };
    }
}

/// Calculates the total number of non-air blocks in an array of 4096 [`Block`]s.
///
/// Currently, this calculates the sum of every single block in the array, with the only exception
/// being air. This may change as more research is done into the purpose of this value in the
/// protocol.
///
/// ## Parameters
/// - `blocks`: a *reference* to the array of blocks to sum. Must be length 4096.
///
/// ## Returns
/// - An `i16` (short) representing the total number of non-air blocks in the chunk section array.
///
/// ## Panics
/// - The only unwrap is in converting the non-air count from a `usize` to an `i32`. By type check,
///   this is at most 4096, so **panics should be impossible**.
#[must_use]
pub fn sum_non_air_blocks(blocks: &[Block; 4096]) -> i16 {
    return blocks
        .iter()
        .filter(|block| return block.state_id != 0)
        .count()
        .try_into()
        .unwrap();
}
