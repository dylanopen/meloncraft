//! Packet forwarders for world border resources.

use bevy::ecs::change_detection::DetectChanges as _;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::{Added, With};
use bevy::ecs::system::{Query, Res};
use meloncraft_packets::{
    ClientboundSetBorderCenter, ClientboundSetBorderSize, ClientboundSetBorderWarningDelay,
    ClientboundSetBorderWarningDistance,
};
use meloncraft_player::PlayerMarker;
use meloncraft_server_info::world_border::{
    WorldBorderCenter, WorldBorderDiameter, WorldBorderWarningDelay, WorldBorderWarningDistance,
};

pub fn send_world_border_center_on_join(
    new_player_q: Query<Entity, Added<PlayerMarker>>,
    world_border_center: Res<WorldBorderCenter>,
    mut set_border_center_pw: MessageWriter<ClientboundSetBorderCenter>,
) {
    for client in new_player_q {
        set_border_center_pw.write(ClientboundSetBorderCenter {
            client,
            center: world_border_center.clone(),
        });
    }
}

pub fn send_world_border_center_on_change(
    player_q: Query<Entity, With<PlayerMarker>>,
    world_border_center: Res<WorldBorderCenter>,
    mut set_border_center_pw: MessageWriter<ClientboundSetBorderCenter>,
) {
    if !world_border_center.is_changed() {
        return;
    }
    for client in player_q {
        set_border_center_pw.write(ClientboundSetBorderCenter {
            client,
            center: world_border_center.clone(),
        });
    }
}

pub fn send_world_border_diameter_on_join(
    new_player_q: Query<Entity, Added<PlayerMarker>>,
    world_border_diameter: Res<WorldBorderDiameter>,
    mut set_border_size_pw: MessageWriter<ClientboundSetBorderSize>,
) {
    for client in new_player_q {
        set_border_size_pw.write(ClientboundSetBorderSize {
            client,
            diameter: world_border_diameter.clone(),
        });
    }
}

pub fn send_world_border_diameter_on_change(
    player_q: Query<Entity, With<PlayerMarker>>,
    world_border_diameter: Res<WorldBorderDiameter>,
    mut set_border_size_pw: MessageWriter<ClientboundSetBorderSize>,
) {
    if !world_border_diameter.is_changed() {
        return;
    }
    for client in player_q {
        set_border_size_pw.write(ClientboundSetBorderSize {
            client,
            diameter: world_border_diameter.clone(),
        });
    }
}

pub fn send_world_border_warning_delay_on_join(
    new_player_q: Query<Entity, Added<PlayerMarker>>,
    world_border_warning_delay: Res<WorldBorderWarningDelay>,
    mut set_border_warning_delay_pw: MessageWriter<ClientboundSetBorderWarningDelay>,
) {
    for client in new_player_q {
        set_border_warning_delay_pw.write(ClientboundSetBorderWarningDelay {
            client,
            warning_delay: world_border_warning_delay.clone(),
        });
    }
}

pub fn send_world_border_warning_delay_on_change(
    player_q: Query<Entity, With<PlayerMarker>>,
    world_border_warning_delay: Res<WorldBorderWarningDelay>,
    mut set_border_warning_delay_pw: MessageWriter<ClientboundSetBorderWarningDelay>,
) {
    if !world_border_warning_delay.is_changed() {
        return;
    }
    for client in player_q {
        set_border_warning_delay_pw.write(ClientboundSetBorderWarningDelay {
            client,
            warning_delay: world_border_warning_delay.clone(),
        });
    }
}

pub fn send_world_border_warning_distance_on_join(
    new_player_q: Query<Entity, Added<PlayerMarker>>,
    world_border_warning_distance: Res<WorldBorderWarningDistance>,
    mut set_border_warning_distance_pw: MessageWriter<ClientboundSetBorderWarningDistance>,
) {
    for client in new_player_q {
        set_border_warning_distance_pw.write(ClientboundSetBorderWarningDistance {
            client,
            warning_distance: world_border_warning_distance.clone(),
        });
    }
}

pub fn send_world_border_warning_distance_on_change(
    player_q: Query<Entity, With<PlayerMarker>>,
    world_border_warning_distance: Res<WorldBorderWarningDistance>,
    mut set_border_warning_distance_pw: MessageWriter<ClientboundSetBorderWarningDistance>,
) {
    if !world_border_warning_distance.is_changed() {
        return;
    }
    for client in player_q {
        set_border_warning_distance_pw.write(ClientboundSetBorderWarningDistance {
            client,
            warning_distance: world_border_warning_distance.clone(),
        });
    }
}
