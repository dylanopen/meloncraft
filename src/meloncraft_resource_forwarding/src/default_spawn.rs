//! Packet forwarders for [`DefaultSpawn`] resource.

use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::Added;
use bevy::ecs::system::{Query, Res};
use meloncraft_packets::ClientboundSetDefaultSpawnPosition;
use meloncraft_player::PlayerMarker;
use meloncraft_server_info::worldspawn::WorldSpawn;

pub fn send_world_spawn_on_join(
    new_player_q: Query<Entity, Added<PlayerMarker>>,
    world_spawn: Res<WorldSpawn>,
    mut set_default_spawn_position_pw: MessageWriter<ClientboundSetDefaultSpawnPosition>,
) {
    for client in new_player_q {
        set_default_spawn_position_pw.write(ClientboundSetDefaultSpawnPosition {
            client,
            dimension: "minecraft:overworld".to_owned(),
            location: world_spawn.location,
            pitch: 0.0,
            yaw: 0.0,
        });
    }
}
