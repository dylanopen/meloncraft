use crate::biome::Biome;
use crate::block::Block;
use crate::block_section::ChunkBlockSection;

pub struct Chunk {
    blocks: Vec<Block>,
}

impl Chunk {
    #[must_use]
    pub const fn new(blocks: Vec<Block>) -> Self {
        return Chunk {
            blocks
        };
    }

    #[must_use]
    pub const fn get_height(&self) -> usize {
        return self.blocks.len() / (16*16);
    }

    #[must_use]
    pub fn get_block(&self, x: usize, z: usize, y: usize) -> Option<&Block> {
        let index = y * 16*16 + z * 16 + x;
        return self.blocks.get(index);
    }

    #[must_use]
    pub fn get_block_mut(&mut self, x: usize, z: usize, y: usize) -> Option<&mut Block> {
        let index = y * 16*16 + z * 16 + x;
        return self.blocks.get_mut(index);
    }

    #[must_use]
    pub const fn get_blocks(&self) -> &Vec<Block> {
        return &self.blocks;
    }

    pub fn set_block(&mut self, x: usize, z: usize, y: usize, block: Block) -> Option<()> {
        let index = y * 16*16 + z * 16 + x;
        let b = self.blocks.get_mut(index)?;
        *b = block;
        return Some(());
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        return self.blocks.iter().all(|b| return b.state_id == 0); // 0 is air, I'm not importing block to tell us that.
    }

    #[must_use]
    pub fn to_chunk_sections(&self) -> Vec<ChunkBlockSection> {
        let mut sections = Vec::new();
        let biomes = [Biome::new(40); 64]; // temporarily set all biomes to plains
        #[expect(clippy::indexing_slicing, reason = "Bounds are already manually checked, by iterating only over self.get_height()")]
        for i in 0..self.get_height() {
            let mut section_blocks = [Block::new(0); 4096];
            section_blocks.copy_from_slice(&self.blocks[i*16*16..(i+1)*16*16]);
            let section = ChunkBlockSection::new(section_blocks, biomes);
            sections.push(section);
        }
        return sections;
    }
}
