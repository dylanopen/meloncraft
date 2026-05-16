use crate::biome::Biome;
use crate::block::Block;

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
