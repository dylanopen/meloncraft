//! Handler systems for message [`SetBlock`].

use bevy::ecs::message::MessageReader;
use bevy::ecs::system::Query;
use bevy::math::{IVec3, UVec3};
use meloncraft_block::set::SetBlock;
use meloncraft_world::world::World;

pub fn store_set_blocks(
    mut set_block_mr: MessageReader<SetBlock>,
    mut worlds_q: Query<&mut World>,
) {
    #[expect(clippy::expect_used, reason = "Will be replaced by logging soon.")]
    let mut world = worlds_q.single_mut().expect("Expected exactly one world");

    for set_block in set_block_mr.read() {
        let chunk = world.get_chunk_at_mut(&set_block.block_location).unwrap();
        let local_block_pos = UVec3::new(
            set_block.block_location.x.rem_euclid(16).try_into().unwrap(),
            (set_block.block_location.y+64).try_into().unwrap(),
            set_block.block_location.z.rem_euclid(16).try_into().unwrap(),
        );
        chunk.set_block(local_block_pos, set_block.new_block);
    }
}

