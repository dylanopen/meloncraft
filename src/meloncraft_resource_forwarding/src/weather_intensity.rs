use bevy::ecs::change_detection::DetectChanges as _;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::{Added, With};
use bevy::ecs::system::{Query, Res};
use meloncraft_core::game_event::GameEventType;
use meloncraft_packets::ClientboundGameEvent;
use meloncraft_player::PlayerMarker;
use meloncraft_server_info::weather_intensity::{RainIntensity, ThunderIntensity};

pub fn send_rain_on_join(
    player_q: Query<Entity, Added<PlayerMarker>>,
    rain_intensity: Res<RainIntensity>,
    mut game_event_pw: MessageWriter<ClientboundGameEvent>,
) {
    for player in player_q {
        if rain_intensity.0.0 == 0.0 {
            game_event_pw.write(ClientboundGameEvent {
                client: player,
                event: GameEventType::EndRaining,
            });
        } else {
            game_event_pw.write(ClientboundGameEvent {
                client: player,
                event: GameEventType::BeginRaining,
            });
        }
        game_event_pw.write(ClientboundGameEvent {
            client: player,
            event: GameEventType::RainLevelChange(rain_intensity.0.clone()),
        });
    }
}

pub fn send_rain_on_change(
    player_q: Query<Entity, With<PlayerMarker>>,
    rain_intensity: Res<RainIntensity>,
    mut game_event_pw: MessageWriter<ClientboundGameEvent>,
) {
    if !rain_intensity.is_changed() {
        return;
    }
    for player in player_q {
        if rain_intensity.0.0 == 0.0 {
            game_event_pw.write(ClientboundGameEvent {
                client: player,
                event: GameEventType::EndRaining,
            });
        } else {
            game_event_pw.write(ClientboundGameEvent {
                client: player,
                event: GameEventType::BeginRaining,
            });
        }
        game_event_pw.write(ClientboundGameEvent {
            client: player,
            event: GameEventType::RainLevelChange(rain_intensity.0.clone()),
        });
    }
}

pub fn send_thunder_on_join(
    player_q: Query<Entity, Added<PlayerMarker>>,
    thunder_intensity: Res<ThunderIntensity>,
    mut game_event_pw: MessageWriter<ClientboundGameEvent>,
) {
    for player in player_q {
        game_event_pw.write(ClientboundGameEvent {
            client: player,
            event: GameEventType::ThunderLevelChange(thunder_intensity.0.clone()),
        });
    }
}

pub fn send_thunder_on_change(
    player_q: Query<Entity, With<PlayerMarker>>,
    thunder_intensity: Res<ThunderIntensity>,
    mut game_event_pw: MessageWriter<ClientboundGameEvent>,
) {
    if !thunder_intensity.is_changed() {
        return;
    }
    for player in player_q {
        game_event_pw.write(ClientboundGameEvent {
            client: player,
            event: GameEventType::ThunderLevelChange(thunder_intensity.0.clone()),
        });
    }
}
