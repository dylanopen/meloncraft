//! Module for struct [`World`].

use bevy::ecs::component::Component;
use bevy::math::{IVec2, IVec3};
use meloncraft_chunk::Chunk;
use std::collections::HashMap;

/// A struct representing the world, which is a collection of chunks.
///
/// ## Representation
/// These chunks are stored as a hashmap, where the key is the chunks position in the world.
/// See [`World::chunks`] for more details on how the chunks are stored.
///
/// ### Chunk position
/// Chunk positions in a [`World`] are calculated from a block position as follows:
/// ```txt
/// chunk_x = floor(block_x / 16)
/// chunk_z = floor(block_z / 16)
/// ```
///
/// ## state
/// The world is not state by itself. You can use the `meloncraft_world_manager` crate if you want a
/// ready-made world state storage.
#[derive(Component, Default)]
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
    /// Inserts a chunk into the world at the specified position.
    ///
    /// ## Replacement
    /// If there is already a chunk at the specified position, it will be replaced by the new chunk,
    /// and the old chunk will be returned as `Some(old_chunk)`.
    ///
    /// ## Insertion
    /// If there is no chunk at the specified position, the new chunk will simply be inserted into
    /// the world, and `None` will be returned.
    ///
    /// ## Parameters
    /// - `&mut self`: The world to insert the chunk into, as a mutable reference to a `World`
    ///   struct.
    /// - `chunk_pos`: The **chunk** position of the chunk to insert at, as an [`IVec2`].
    /// - `chunk`: The chunk to insert, as a [`Chunk`] struct.
    ///
    /// ## Returns
    /// - `Some(old_chunk)`: If there was already a chunk at the specified position, the old chunk
    ///   that was replaced will be returned.
    /// - `None`: If there was no chunk at the insertion position.
    pub fn insert_chunk(&mut self, chunk_pos: IVec2, chunk: Chunk) -> Option<Chunk> {
        return self.chunks.insert(chunk_pos, chunk);
    }

    /// Return a reference to the chunk at the specified position, if it exists, else `None`.
    ///
    /// ## Parameters
    /// - `&self`: The world to get the chunk from, as a reference to a `World` struct.
    /// - `chunk_pos`: The **chunk** position of the chunk to get, as an [`IVec2`].
    ///
    /// ## Returns
    /// - `None` if there is no chunk at the specified position.
    /// - `Some(&chunk)`, a reference to the chunk at the specified position, if it exists.
    #[must_use]
    pub fn get_chunk(&self, chunk_pos: &IVec2) -> Option<&Chunk> {
        return self.chunks.get(chunk_pos);
    }

    /// Return a mutable reference to the chunk at the specified position, if it exists, else
    /// `None`.
    ///
    /// > Mutable variant of [`World::get_chunk`].
    ///
    /// ## Parameters
    /// - `&mut self`: The world to get the chunk from, as a mutable reference to a `World` struct.
    /// - `chunk_pos`: The **chunk** position of the chunk to get, as an [`IVec2`].
    ///
    /// ## Returns
    /// - `None` if there is no chunk at the specified position.
    /// - `Some(&mut chunk)`, a mutable reference to the chunk at the specified position, if it
    ///   exists.
    #[must_use]
    pub fn get_chunk_mut(&mut self, chunk_pos: &IVec2) -> Option<&mut Chunk> {
        return self.chunks.get_mut(chunk_pos);
    }

    /// Return `true` if there is a chunk at the specified *chunk* position, else `false`.
    ///
    /// ## Parameters
    /// - `&self`: The world to check for the chunk, as a reference to a `World` struct.
    /// - `chunk_pos`: The **chunk** position of the chunk to check for, as an [`IVec2`].
    ///
    /// ## Returns
    /// - `true` if there is a chunk at the specified position.
    /// - `false` if there is no chunk at the specified position.
    ///
    /// ## Alternatives
    /// - You can also check for the existence of a chunk by using [`World::get_chunk`] and
    ///   checking if the result is `Some(&chunk)` or `None`.
    #[must_use]
    pub fn has_chunk(&self, chunk_pos: &IVec2) -> bool {
        return self.chunks.contains_key(chunk_pos);
    }

    /// Return a reference to the hashmap of **all** chunks in the world.
    ///
    /// ## Parameters
    /// - `&self`: The world to get the chunks from, as a reference to a `World` struct.
    ///
    /// ## Returns
    /// - A *reference* to the hashmap of chunks in the world, where the key is the chunk's position
    /// - The reference is to the hashmap itself, not the individual [`Chunk`] elements.
    #[must_use]
    pub const fn get_chunks(&self) -> &HashMap<IVec2, Chunk> {
        return &self.chunks;
    }

    /// Return a mutable reference to the hashmap of **all** chunks in the world.
    /// > Mutable variant of [`World::get_chunks`].
    ///
    /// ## Parameters
    /// - `&mut self`: The world to get the chunks from, as a mutable reference to a `World` struct.
    ///
    /// ## Returns
    /// - A *mutable reference* to the hashmap of chunks in the world, where the key is the chunk's position
    /// - The mutable reference is to the hashmap itself, not the individual [`Chunk`] elements.
    #[must_use]
    pub const fn get_chunks_mut(&mut self) -> &mut HashMap<IVec2, Chunk> {
        return &mut self.chunks;
    }

    /// Return the number of chunks currently *stored* in the world.
    ///
    /// This is not necessarily the same as the number of chunks currently *loaded* in the world,
    /// since you probably won't have all chunks loaded at once.
    ///
    /// Let's say, in your world, you have two different 9x9 chunk areas loaded around two different
    /// players, with no chunks loaded anywhere else. In this case, you would have 162 chunks loaded
    /// in the world, since each 9x9 area contains 81 chunks, and you have two of those areas
    /// loaded.
    ///
    /// ## Parameters
    /// - `&self`: The world to get the chunk count from, as a reference to a `World` struct.
    ///
    /// ## Returns
    /// - The number of chunks currently stored in the world, as a `usize`.
    #[must_use]
    pub fn get_chunk_count(&self) -> usize {
        return self.chunks.len();
    }

    /// Return `true` if there are no chunks currently stored in the world, else `false`.
    ///
    /// This is not necessarily the same as whether there are no chunks currently *loaded* in the
    /// world.
    ///
    /// ## Internals
    /// Internally, this checks whether the hashmap [`World::chunks`] is empty or not, since if
    /// there are no chunks stored in the world, then the world is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        return self.chunks.is_empty();
    }

    /// Return the chunk position corresponding to the given block position, as an [`IVec2`].
    ///
    /// ## Parameters
    /// - `block_pos`: The block position to get the chunk position from, as an [`IVec2`].
    ///
    /// ## Returns
    /// - The chunk position (in chunk grid coords) corresponding to the given block position,
    ///   as an [`IVec2`].
    #[must_use]
    pub const fn get_chunk_pos_from_block_pos(block_pos: &IVec3) -> IVec2 {
        return IVec2::new(block_pos.x.div_euclid(16), block_pos.z.div_euclid(16));
    }

    /// Return the actual chunk corresponding to the given block position, if it exists, else `None`.
    ///
    /// ## Parameters
    /// - `&self`: The world to get the chunk from, as a reference to a `World` struct.
    /// - `block_pos`: The block position to get the chunk from, as an [`IVec2`].
    ///
    /// ## Returns
    /// - `None` if there is no chunk at the specified block position.
    /// - `Some(&chunk)`, a reference to the chunk at the specified block position, if it exists.
    #[must_use]
    pub fn get_chunk_at(&self, block_pos: &IVec3) -> Option<&Chunk> {
        let chunk_pos = Self::get_chunk_pos_from_block_pos(block_pos);
        return self.get_chunk(&chunk_pos);
    }

    /// Return a mutable reference to the actual chunk corresponding to the given block position,
    /// if it exists, else `None`.
    ///
    /// > Mutable variant of [`World::get_chunk_at`].
    ///
    /// ## Parameters
    /// - `&mut self`: The world to get the chunk from, as a mutable reference to a `World` struct.
    /// - `block_pos`: The block position to get the chunk from, as an [`IVec2`].
    ///
    /// ## Returns
    /// - `None` if there is no chunk at the specified block position.
    /// - `Some(&mut chunk)`, a mutable reference to the chunk at the specified block position, if it exists.
    #[must_use]
    pub fn get_chunk_at_mut(&mut self, block_pos: &IVec3) -> Option<&mut Chunk> {
        let chunk_pos = Self::get_chunk_pos_from_block_pos(block_pos);
        return self.get_chunk_mut(&chunk_pos);
    }
}
