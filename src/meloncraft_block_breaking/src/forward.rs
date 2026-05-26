//! Module for managing the chunk data in block breaking events.

use bevy::ecs::message::{MessageReader, MessageWriter};
use meloncraft_block::block::Block;
use meloncraft_block::broken::BlockBroken;
use meloncraft_block::set::SetBlock;

pub fn set_broken_blocks(
    mut block_broken_mr: MessageReader<BlockBroken>,
    mut set_block_mw: MessageWriter<SetBlock>,
) {
    for block_broken in block_broken_mr.read() {
        set_block_mw.write(SetBlock {
            block_location: block_broken.block_location,
            new_block: Block::new(0),
        });
    }
}
