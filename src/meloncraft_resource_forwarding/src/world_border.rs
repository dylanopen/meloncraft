//! Packet forwarders for world border resources.

use bevy::ecs::change_detection::DetectChanges as _;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::{Added, With};
use bevy::ecs::system::{Query, Res};
use meloncraft_packets::ClientboundSetBorderCenter;
use meloncraft_player::PlayerMarker;
use meloncraft_server_info::world_border::WorldBorderCenter;

pub fn send_world_border_center_on_join(
    new_player_q: Query<Entity, Added<PlayerMarker>>,
    world_spawn: Res<WorldBorderCenter>,
    mut set_border_center_pw: MessageWriter<ClientboundSetBorderCenter>,
) {
    for client in new_player_q {
        set_border_center_pw.write(ClientboundSetBorderCenter {
            client,
            center: world_spawn.clone(),
        });
    }
}

pub fn send_world_border_center_on_change(
    player_q: Query<Entity, With<PlayerMarker>>,
    world_border_center: Res<WorldBorderCenter>,
    mut set_border_center_pw: MessageWriter<ClientboundSetBorderCenter>,
) {
    if !world_border_center.is_changed() { return; }
    for client in player_q {
        set_border_center_pw.write(ClientboundSetBorderCenter {
            client,
            center: world_border_center.clone(),
        });
    }
}

