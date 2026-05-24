use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::query::Added;
use bevy::ecs::system::Query;
use meloncraft_packets::ClientboundPlayerInfoUpdate;
use meloncraft_player::{GameProfile, PlayerMarker};
use meloncraft_player::client_action::{AddPlayerAction, ClientPlayerAction, UpdateClientPlayerAction};

pub fn send_client_player_action(
    mut update_client_player_action_mr: MessageReader<UpdateClientPlayerAction>,
    mut player_info_update_pw: MessageWriter<ClientboundPlayerInfoUpdate>,
    player_q: Query<(Entity, &GameProfile)>,
) {
    for action in update_client_player_action_mr.read() {
        for (client, _) in player_q {
            let uuid = player_q.get(action.player).unwrap().1.uuid.clone();
            player_info_update_pw.write(ClientboundPlayerInfoUpdate {
                action_mask: action.action.mask(),
                client,
                players: vec![(uuid.clone(), vec![action.action.clone()])],
            });
        }
    }
}

pub fn send_add_player(
    added_player_q: Query<(Entity, &GameProfile), Added<PlayerMarker>>,
    mut update_client_player_action_mw: MessageWriter<UpdateClientPlayerAction>,
) {
    for added_player in added_player_q {
        update_client_player_action_mw.write(UpdateClientPlayerAction {
            player: added_player.0,
            action: ClientPlayerAction::AddPlayer(AddPlayerAction {
                name: added_player.1.username.clone(),
                game_profile_properties: Vec::new(),
            })
        });
        update_client_player_action_mw.write(UpdateClientPlayerAction {
            player: added_player.0,
            action: ClientPlayerAction::UpdateListed(true),
        });
    };
}

