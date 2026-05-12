use crate::biome::Biome;
use crate::block::Block;

#[derive(Debug, Clone)]
pub struct ChunkBlockSection {
    pub block_count: i16,
    pub blocks: [Block; 4096],
    pub biomes: [Biome; 64],
}

impl ChunkBlockSection {
    pub fn new(blocks: [Block; 4096], biomes: [Biome; 64]) -> Self {
        let block_count = blocks.iter().filter(|block| block.state_id != 0).count() as i16; // sum non-air
        Self { block_count, blocks, biomes }
    }
}