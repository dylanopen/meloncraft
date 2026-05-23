//! Handler systems for message [`SetBlock`].

use bevy::ecs::entity::{Entity};
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::math::UVec3;
use meloncraft_block::set::SetBlock;
use meloncraft_entity::position::EntityPosition;
use meloncraft_packets::ClientboundBlockUpdate;
use meloncraft_player::{ClientViewDistance, GameProfile};
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

pub fn send_set_blocks(
    mut set_block_mr: MessageReader<SetBlock>,
    players: Query<(Entity, &ClientViewDistance, &EntityPosition), With<GameProfile>>,
    mut block_update_pw: MessageWriter<ClientboundBlockUpdate>,
) {
    for set_block in set_block_mr.read() {
        for (player_entity, player_view_distance, player_position) in players {
            #[expect(clippy::as_conversions, clippy::cast_possible_truncation, reason = "Unsure how else to compare a float with an i32")]
            if (set_block.block_location.x - player_position.location.x as i32).abs() / 16 > i32::from(player_view_distance.0) + 1
            || (set_block.block_location.y - player_position.location.y as i32).abs() / 16 > i32::from(player_view_distance.0) + 1 {
                continue; //
            }
            block_update_pw.write(ClientboundBlockUpdate {
                client: player_entity,
                block_location: set_block.block_location,
                new_block: set_block.new_block,
            });
        }
    }
}

