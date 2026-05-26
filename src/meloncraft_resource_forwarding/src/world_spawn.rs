//! Packet forwarders for [`WorldSpawn`] resource.

use bevy::ecs::change_detection::DetectChanges as _;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::{Added, With};
use bevy::ecs::system::{Query, Res};
use meloncraft_packets::ClientboundSetDefaultSpawnPosition;
use meloncraft_player::PlayerMarker;
use meloncraft_server_info::world_spawn::WorldSpawn;

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

pub fn send_world_spawn_on_change(
    player_q: Query<Entity, With<PlayerMarker>>,
    world_spawn: Res<WorldSpawn>,
    mut set_default_spawn_position_pw: MessageWriter<ClientboundSetDefaultSpawnPosition>,
) {
    if !world_spawn.is_changed() {
        return;
    }
    for client in player_q {
        set_default_spawn_position_pw.write(ClientboundSetDefaultSpawnPosition {
            client,
            dimension: "minecraft:overworld".to_owned(),
            location: world_spawn.location,
            pitch: 0.0,
            yaw: 0.0,
        });
    }
}
