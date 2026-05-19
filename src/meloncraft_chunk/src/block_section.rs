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

    pub block_count: i16,

    pub blocks: [Block; 4096],

    pub biomes: [Biome; 64],
}

impl ChunkBlockSection {

    #[must_use]
    pub fn new(blocks: [Block; 4096], biomes: [Biome; 64]) -> Self {
        let block_count = sum_non_air_blocks(&blocks);
        return ChunkBlockSection { block_count, blocks, biomes }
    }
}

#[must_use]
pub fn sum_non_air_blocks(blocks: &[Block; 4096]) -> i16 {
    return blocks.iter().filter(|block| return block.state_id != 0).count().try_into().unwrap();
}
