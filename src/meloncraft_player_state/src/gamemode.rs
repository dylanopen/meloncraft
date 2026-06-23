//! Systems for managing a player's [`GameMode`].

use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::Changed;
use bevy::ecs::system::Query;
use meloncraft_core::GameMode;
use meloncraft_core::game_event::GameEventType;
use meloncraft_packets::ClientboundGameEvent;
use meloncraft_player::client_action::{ClientPlayerAction, UpdateClientPlayerAction};

/// Sends an `UpdateClientPlayerAction` packet to all players on the server, whenever a player's
/// gamemode changes, to indicate they should update the gamemode of that player in their
/// client-side state.
pub fn send_gamemode_info_update(
    changed_gamemode_q: Query<(Entity, &GameMode), Changed<GameMode>>,
    mut update_client_player_action_mw: MessageWriter<UpdateClientPlayerAction>,
) {
    for (entity, gamemode) in changed_gamemode_q {
        update_client_player_action_mw.write(UpdateClientPlayerAction {
            player: entity,
            action: ClientPlayerAction::UpdateGameMode(u8::from(*gamemode).into()),
        });
    }
}

/// Sends a game event *only to the player(s) who changed gamemodes*, to indicate they should switch
/// to the new gamemode component.
pub fn send_gamemode_game_event(
    changed_gamemode_q: Query<(Entity, &GameMode), Changed<GameMode>>,
    mut game_event_pw: MessageWriter<ClientboundGameEvent>,
) {
    for (entity, gamemode) in changed_gamemode_q {
        game_event_pw.write(ClientboundGameEvent {
            client: entity,
            event: GameEventType::ChangeGameMode(*gamemode),
        });
    }
}
