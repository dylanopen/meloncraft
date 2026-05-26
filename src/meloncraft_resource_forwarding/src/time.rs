use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::query::{Added, With};
use bevy::ecs::system::{Query, Res};
use meloncraft_packets::ClientboundSetTime;
use meloncraft_player::PlayerMarker;
use meloncraft_server_info::time::{DayTime, DaylightCycle, OpenTime, TimeChanged};

pub fn send_set_time_on_join(
    player_q: Query<Entity, Added<PlayerMarker>>,
    day_time: Res<DayTime>,
    open_time: Res<OpenTime>,
    daylight_cycle: Res<DaylightCycle>,
    mut set_time_pw: MessageWriter<ClientboundSetTime>,
) {
    for player in player_q {
        set_time_pw.write(ClientboundSetTime { client: player, day_time: *day_time, open_time: *open_time, daylight_cycle: *daylight_cycle });
    }
}

pub fn send_set_time_on_change(
    player_q: Query<Entity, With<PlayerMarker>>,
    day_time: Res<DayTime>,
    open_time: Res<OpenTime>,
    daylight_cycle: Res<DaylightCycle>,
    mut time_changed_mr: MessageReader<TimeChanged>,
    mut set_time_pw: MessageWriter<ClientboundSetTime>,
) {
    for _ in time_changed_mr.read() {
        for player in player_q {
            set_time_pw.write(ClientboundSetTime { client: player, day_time: *day_time, open_time: *open_time, daylight_cycle: *daylight_cycle });
        }
    }
}

