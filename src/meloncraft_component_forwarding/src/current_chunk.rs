use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::Changed;
use bevy::ecs::system::Query;
use bevy::math::IVec2;
use meloncraft_entity::position::current_chunk::CurrentChunk;
use meloncraft_packets::ClientboundSetCenterChunk;
use meloncraft_player::PlayerMarker;

pub fn send_player_current_chunk(
    current_chunk_q: Query<(Entity, &CurrentChunk, &PlayerMarker), Changed<CurrentChunk>>,
    mut set_center_chunk_pw: MessageWriter<ClientboundSetCenterChunk>,
) {
    for (entity, current_chunk, _) in current_chunk_q {
        set_center_chunk_pw.write(ClientboundSetCenterChunk {
            client: entity,
            chunk_pos: IVec2::new(current_chunk.location.x, current_chunk.location.z),
        });
    }
}
