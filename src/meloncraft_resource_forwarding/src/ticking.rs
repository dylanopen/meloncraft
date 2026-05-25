use bevy::ecs::change_detection::DetectChanges as _;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::{Added, With};
use bevy::ecs::system::{Query, Res};
use meloncraft_packets::ClientboundSetTickingState;
use meloncraft_player::PlayerMarker;
use meloncraft_server_info::tick::{TickRate, TickingFrozen};

pub fn send_ticking_state_on_join(
    new_player_q: Query<Entity, Added<PlayerMarker>>,
    tick_rate: Res<TickRate>,
    ticking_frozen: Res<TickingFrozen>,
    mut set_ticking_state_pw: MessageWriter<ClientboundSetTickingState>
) {
    for client in new_player_q {
        set_ticking_state_pw.write(ClientboundSetTickingState {
            client,
            tick_rate: *tick_rate,
            ticking_frozen: *ticking_frozen,
        });
    }
}

pub fn send_ticking_state_on_change(
    player_q: Query<Entity, With<PlayerMarker>>,
    tick_rate: Res<TickRate>,
    ticking_frozen: Res<TickingFrozen>,
    mut set_ticking_state_pw: MessageWriter<ClientboundSetTickingState>
) {
    if !tick_rate.is_changed() && !ticking_frozen.is_changed() { return; }
    for client in player_q {
        set_ticking_state_pw.write(ClientboundSetTickingState {
            client,
            tick_rate: *tick_rate,
            ticking_frozen: *ticking_frozen,
        });
    }
}

