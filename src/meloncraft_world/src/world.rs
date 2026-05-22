//! Module for struct [`World`].

use std::collections::HashMap;
use bevy::math::IVec2;
use meloncraft_chunk::Chunk;

/// A struct representing the world, which is a collection of chunks.
///
/// ## Representation
/// These chunks are stored as a hashmap, where the key is the chunks position in the world.
/// See [`World::chunks`] for more details on how the chunks are stored.
///
/// ### Chunk position
/// Chunk positions in a [`World`] are calculated from a block position as follows:
/// ```
/// chunk_x = floor(block_x / 16)
/// chunk_z = floor(block_z / 16)
/// ```
///
/// ## state
/// The world is not state by itself. You can use the `meloncraft_world_manager` crate if you want a
/// ready-made world state storage.
#[derive(Default)]
pub struct World {
    /// The chunks in the world, stored as a hashmap where the key is the chunk's position in the world.
    ///
    /// The chunk position is a Bevy [`IVec2`], which is an `(i32, i32)` representing the chunk's
    /// coordinates in the chunk grid. For example, the chunk containing the block at block
    /// coordinates `(100, 50, -100)` would be at chunk coordinates `(6, -7)`, since each chunk
    /// is 16 blocks wide and chunk coordinates are calculated by dividing block coordinates by
    /// 16 and flooring the result.
    ///
    /// The chunk data itself is stored in the value of the hashmap, as a [`Chunk`] struct. See the
    /// `meloncraft_chunk` crate for more information about the `Chunk` struct and how to work with
    /// it.
    ///
    /// Please access this field through the methods provided by the `World` struct, e.g.
    /// [`World::insert_chunk`], [`World::get_chunk`].
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
