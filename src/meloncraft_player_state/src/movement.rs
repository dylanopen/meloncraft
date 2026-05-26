use bevy::math::DVec3;
use bevy::prelude::{MessageReader, MessageWriter, Query, With};
use meloncraft_entity::position::EntityPosition;
use meloncraft_entity::position::moved::EntityMoved;
use meloncraft_entity::position::teleport::TeleportEntity;
use meloncraft_packets::{ClientboundSynchronizePlayerPosition, ServerboundSetPlayerPosition};
use meloncraft_player::GameProfile;

/// System for forwarding player movement packets as [`EntityMoved`] messages.
///
/// This system listens for [`ServerboundSetPlayerPosition`] packets, retrieves the player's old
/// position from the ECS query, and writes an [`EntityMoved`] message with the old and
/// new positions.
///
/// If the player does not have an existing position (i.e., they are new or their position has not
/// been tracked yet), the system will use the new position from the packet as the old position in
/// the [`EntityMoved`] message (i.e. [`EntityMoved::new_position`] will equal [`EntityMoved::old_position`]).
pub fn fwd_player_moved(
    mut set_position_pr: MessageReader<ServerboundSetPlayerPosition>,
    mut player_moved_mw: MessageWriter<EntityMoved>,
    existing_positions: Query<&EntityPosition, With<GameProfile>>,
) {
    for packet in set_position_pr.read() {
        let old_position = existing_positions.get(packet.client).map_or_else(
            |_| return packet.position.clone(),
            |old_position| return old_position.clone(),
        );
        player_moved_mw.write(EntityMoved {
            entity: packet.client,
            old_position,
            new_position: packet.position.clone(),
        });
    }
}

pub fn fwd_player_teleport(
    mut teleport_entity_mr: MessageReader<TeleportEntity>,
    game_profile_q: Query<&GameProfile>,
    mut synchronize_player_position_pw: MessageWriter<ClientboundSynchronizePlayerPosition>,
) {
    for teleport_entity in teleport_entity_mr.read() {
        if game_profile_q.get(teleport_entity.entity).is_err() {
            continue; // the entity isn't a player, we don't teleport them in this crate.
        }
        synchronize_player_position_pw.write(ClientboundSynchronizePlayerPosition {
            client: teleport_entity.entity,
            position: teleport_entity.new_position.location,
            velocity: DVec3::ZERO,
            teleport_id: 0, // TODO. As for now, I don't think the ID matters.
            pitch: 90.0,
            yaw: 90.0,
        });
    }

    // TODO: We should ignore any packets the client sends asking to update their position, in
    // between sending this packet and receiving acknowledgement from the client.
    // Otherwise we may store incorrect values.
}
