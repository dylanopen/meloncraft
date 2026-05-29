//! Packet forwarders for world border resources.

use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::{Added, Changed, With};
use bevy::ecs::system::Query;
use meloncraft_entity::position::EntityPosition;
use meloncraft_packets::{
    ClientboundSetBorderCenter, ClientboundSetBorderSize, ClientboundSetBorderWarningDelay,
    ClientboundSetBorderWarningDistance,
};
use meloncraft_player::PlayerMarker;
use meloncraft_player::marker::LoadedPlayer;
use meloncraft_server_info::world_border::{
    WorldBorderCenter, WorldBorderDiameter, WorldBorderWarningDelay, WorldBorderWarningDistance,
};
pub fn send_world_border_center_on_join(
    new_player_q: Query<(Entity, &EntityPosition), Added<LoadedPlayer>>,
    world_border_center_q: Query<&WorldBorderCenter>,
    mut set_border_center_pw: MessageWriter<ClientboundSetBorderCenter>,
) {
    for (client, player_position) in new_player_q {
        let world_border_center = world_border_center_q.get(player_position.world).unwrap();
        set_border_center_pw.write(ClientboundSetBorderCenter {
            client,
            center: world_border_center.clone(),
        });
    }
}

pub fn send_world_border_center_on_change(
    player_q: Query<(Entity, &EntityPosition), With<PlayerMarker>>,
    world_border_center_q: Query<(Entity, &WorldBorderCenter), Changed<WorldBorderCenter>>,
    mut set_border_center_pw: MessageWriter<ClientboundSetBorderCenter>,
) {
    for (border_world, new_border) in world_border_center_q {
        for (client, player_position) in player_q {
            if border_world != player_position.world {
                continue;
            }
            set_border_center_pw.write(ClientboundSetBorderCenter {
                client,
                center: new_border.clone(),
            });
        }
    }
}

pub fn send_world_border_diameter_on_join(
    new_player_q: Query<(Entity, &EntityPosition), Added<LoadedPlayer>>,
    world_border_diameter_q: Query<&WorldBorderDiameter>,
    mut set_border_center_pw: MessageWriter<ClientboundSetBorderSize>,
) {
    for (client, player_position) in new_player_q {
        let world_border_diameter = world_border_diameter_q.get(player_position.world).unwrap();
        set_border_center_pw.write(ClientboundSetBorderSize {
            client,
            diameter: world_border_diameter.clone(),
        });
    }
}

pub fn send_world_border_diameter_on_change(
    player_q: Query<(Entity, &EntityPosition), With<PlayerMarker>>,
    world_border_diameter_q: Query<(Entity, &WorldBorderDiameter), Changed<WorldBorderDiameter>>,
    mut set_border_diameter_pw: MessageWriter<ClientboundSetBorderSize>,
) {
    for (border_world, new_border) in world_border_diameter_q {
        for (client, player_position) in player_q {
            if border_world != player_position.world {
                continue;
            }
            set_border_diameter_pw.write(ClientboundSetBorderSize {
                client,
                diameter: new_border.clone(),
            });
        }
    }
}

pub fn send_world_border_warning_delay_on_join(
    new_player_q: Query<(Entity, &EntityPosition), Added<LoadedPlayer>>,
    world_border_warning_delay_q: Query<&WorldBorderWarningDelay>,
    mut set_border_warning_delay_pw: MessageWriter<ClientboundSetBorderWarningDelay>,
) {
    for (client, player_position) in new_player_q {
        let world_border_warning_delay = world_border_warning_delay_q
            .get(player_position.world)
            .unwrap();
        set_border_warning_delay_pw.write(ClientboundSetBorderWarningDelay {
            client,
            warning_delay: world_border_warning_delay.clone(),
        });
    }
}

pub fn send_world_border_warning_delay_on_change(
    player_q: Query<(Entity, &EntityPosition), With<PlayerMarker>>,
    world_border_warning_delay_q: Query<
        (Entity, &WorldBorderWarningDelay),
        Changed<WorldBorderWarningDelay>,
    >,
    mut set_border_warning_delay_pw: MessageWriter<ClientboundSetBorderWarningDelay>,
) {
    for (border_world, new_border) in world_border_warning_delay_q {
        for (client, player_position) in player_q {
            if border_world != player_position.world {
                continue;
            }
            set_border_warning_delay_pw.write(ClientboundSetBorderWarningDelay {
                client,
                warning_delay: new_border.clone(),
            });
        }
    }
}

pub fn send_world_border_warning_distance_on_join(
    new_player_q: Query<(Entity, &EntityPosition), Added<LoadedPlayer>>,
    world_border_warning_distance_q: Query<&WorldBorderWarningDistance>,
    mut set_border_warning_distance_pw: MessageWriter<ClientboundSetBorderWarningDistance>,
) {
    for (client, player_position) in new_player_q {
        let world_border_warning_distance = world_border_warning_distance_q
            .get(player_position.world)
            .unwrap();
        set_border_warning_distance_pw.write(ClientboundSetBorderWarningDistance {
            client,
            warning_distance: world_border_warning_distance.clone(),
        });
    }
}

pub fn send_world_border_warning_distance_on_change(
    player_q: Query<(Entity, &EntityPosition), With<PlayerMarker>>,
    world_border_warning_distance_q: Query<
        (Entity, &WorldBorderWarningDistance),
        Changed<WorldBorderWarningDistance>,
    >,
    mut set_border_warning_distance_pw: MessageWriter<ClientboundSetBorderWarningDistance>,
) {
    for (border_world, new_border) in world_border_warning_distance_q {
        for (client, player_position) in player_q {
            if border_world != player_position.world {
                continue;
            }
            set_border_warning_distance_pw.write(ClientboundSetBorderWarningDistance {
                client,
                warning_distance: new_border.clone(),
            });
        }
    }
}
