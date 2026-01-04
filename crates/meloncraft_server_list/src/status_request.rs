use crate::max_players::MaxPlayers;
use crate::motd::Motd;
use crate::online_players::OnlinePlayers;
use bevy::prelude::{MessageReader, MessageWriter, Res};
use meloncraft_packets::clientbound::status::StatusResponse;
use meloncraft_packets::incoming::status::StatusRequest;

pub fn respond_to_status_request(
    mut status_pr: MessageReader<StatusRequest>,
    mut status_pw: MessageWriter<StatusResponse>,
    motd: Res<Motd>,
    max_players: Res<MaxPlayers>,
    online_players: Res<OnlinePlayers>,
) {
    for msg in status_pr.read() {
        status_pw.write(StatusResponse {
            client: msg.client,
            description: motd.0.clone(),
            enforces_secure_chat: false,
            max_players: max_players.0,
            online_players: online_players.0,
            version_name: "Meloncraft".to_owned(),
            version_protocol: 773,
        });
    }
}
