use std::collections::HashMap;
use bevy::math::IVec2;
use meloncraft_chunk::Chunk;

#[derive(Default)]
pub struct World {
    chunks: HashMap<IVec2, Chunk>,
}

impl World {
    pub fn insert_chunk(&mut self, chunk_pos: IVec2, chunk: Chunk) {
        self.chunks.insert(chunk_pos, chunk);
    }

    #[must_use]
    pub fn get_chunk(&self, chunk_pos: &IVec2) -> Option<&Chunk> {
        return self.chunks.get(chunk_pos);
    }

    #[must_use]
    pub fn get_chunk_mut(&mut self, chunk_pos: &IVec2) -> Option<&mut Chunk> {
        return self.chunks.get_mut(chunk_pos);
    }

    #[must_use]
    pub fn has_chunk(&self, chunk_pos: &IVec2) -> bool {
        return self.chunks.contains_key(chunk_pos);
    }

    #[must_use]
    pub const fn get_chunks(&self) -> &HashMap<IVec2, Chunk> {
        return &self.chunks;
    }

    #[must_use]
    pub const fn get_chunks_mut(&mut self) -> &mut HashMap<IVec2, Chunk> {
        return &mut self.chunks;
    }

    #[must_use]
    pub fn get_chunk_count(&self) -> usize {
        return self.chunks.len();
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        return self.chunks.is_empty();
    }
}
