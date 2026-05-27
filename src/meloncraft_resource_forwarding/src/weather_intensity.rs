use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::Added;
use bevy::ecs::system::{Query, Res};
use meloncraft_core::game_event::GameEventType;
use meloncraft_packets::ClientboundGameEvent;
use meloncraft_player::PlayerMarker;
use meloncraft_server_info::weather_intensity::RainIntensity;

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
