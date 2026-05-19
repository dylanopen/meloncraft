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
#[derive(Debug, Clone, Eq, PartialEq)]
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
        return self.blocks.len() / (16*16*16);
    }

    #[must_use]
    pub fn get_block(&self, location: IVec3) -> Option<&Block> {
        let index = Chunk::get_index(location);
        return self.blocks.get(index);
    }

    #[must_use]
    pub fn get_block_mut(&mut self, location: IVec3) -> Option<&mut Block> {
        let index = Chunk::get_index(location);
        return self.blocks.get_mut(index);
    }

    #[must_use]
    pub const fn get_blocks(&self) -> &Vec<Block> {
        return &self.blocks;
    }

    pub fn set_block(&mut self, location: IVec3, block: Block) -> Option<()> {
        let index = Chunk::get_index(location); 
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
            section_blocks.copy_from_slice(&self.blocks[i*16*16*16..(i+1)*16*16*16]);
            let section = ChunkBlockSection::new(section_blocks, biomes);
            sections.push(section);
        }
        return sections;
    }

    fn get_index(location: IVec3) -> usize {
        return usize::try_from(location.y * 16*16 + location.z * 16 + location.x).unwrap();
    }
}
