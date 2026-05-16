use std::collections::HashMap;
use meloncraft_chunk::Chunk;

#[derive(Default)]
pub struct World {
    chunks: HashMap<(i32, i32), Chunk>,
}

impl World {
    pub fn insert_chunk(&mut self, chunk_x: i32, chunk_z: i32, chunk: Chunk) {
        self.chunks.insert((chunk_x, chunk_z), chunk);
    }

    #[must_use]
    pub fn get_chunk(&self, chunk_x: i32, chunk_z: i32) -> Option<&Chunk> {
        return self.chunks.get(&(chunk_x, chunk_z));
    }

    #[must_use]
    pub fn get_chunk_mut(&mut self, chunk_x: i32, chunk_z: i32) -> Option<&mut Chunk> {
        return self.chunks.get_mut(&(chunk_x, chunk_z));
    }

    #[must_use]
    pub fn has_chunk(&self, chunk_x: i32, chunk_z: i32) -> bool {
        return self.chunks.contains_key(&(chunk_x, chunk_z));
    }

    #[must_use]
    pub const fn get_chunks(&self) -> &HashMap<(i32, i32), Chunk> {
        return &self.chunks;
    }

    #[must_use]
    pub const fn get_chunks_mut(&mut self) -> &mut HashMap<(i32, i32), Chunk> {
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
