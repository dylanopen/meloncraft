//! Systems for managing a player's [`GameMode`].

use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::{Added, Changed};
use bevy::ecs::system::{Commands, Query};
use meloncraft_core::GameMode;
use meloncraft_core::game_event::GameEventType;
use meloncraft_packets::ClientboundGameEvent;
use meloncraft_player::PlayerMarker;
use meloncraft_player::client_action::{ClientPlayerAction, UpdateClientPlayerAction};

/// Inserts a [`GameMode`] component with the default value of `Creative` for each player that has
/// just loaded in.
pub fn insert_gamemode(
    mut commands: Commands,
    added_player_q: Query<Entity, Added<PlayerMarker>>,
) {
    for added_player in added_player_q {
        commands.get_entity(added_player).unwrap().insert(GameMode::Creative);
    }
}

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

