use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Query;
use meloncraft_packets::ClientboundPlayerInfoUpdate;
use meloncraft_player::GameProfile;
use meloncraft_player::client_action::UpdateClientPlayerAction;

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

