//! Forwarding for the `ClientboundServerData` packet (play).
//! Sends MOTD and serrver icon.

use bevy::ecs::change_detection::DetectChanges as _;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::{Added, With};
use bevy::ecs::system::{Query, Res};
use meloncraft_packets::ClientboundServerData;
use meloncraft_player::PlayerMarker;
use meloncraft_server_info::motd::Motd;


pub fn send_server_data_on_join(
    new_player_q: Query<Entity, Added<PlayerMarker>>,
    motd: Res<Motd>,
    mut server_data_pw: MessageWriter<ClientboundServerData>,
) {
    for client in new_player_q {
        server_data_pw.write(ClientboundServerData {
            client,
            motd: motd.clone(),
            icon: Vec::new(),
        });
    }
}

pub fn send_server_data_on_change_motd(
    player_q: Query<Entity, With<PlayerMarker>>,
    motd: Res<Motd>,
    mut server_data_pw: MessageWriter<ClientboundServerData>,
) {
    if !motd.is_changed() { return; }
    for client in player_q {
        server_data_pw.write(ClientboundServerData {
            client,
            motd: motd.clone(),
            icon: Vec::new(),
        });
    }
}

