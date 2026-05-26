use bevy::prelude::{MessageReader, MessageWriter, Res};
use meloncraft_packets::ClientboundStatusResponse;
use meloncraft_packets::ServerboundStatusRequest;
use meloncraft_server_info::max_players::MaxPlayers;
use meloncraft_server_info::motd::Motd;
use meloncraft_server_info::online_players::OnlinePlayers;

pub fn respond_to_status_request(
    mut status_pr: MessageReader<ServerboundStatusRequest>,
    mut status_pw: MessageWriter<ClientboundStatusResponse>,
    motd: Res<Motd>,
    max_players: Res<MaxPlayers>,
    online_players: Res<OnlinePlayers>,
) {
    for msg in status_pr.read() {
        status_pw.write(ClientboundStatusResponse {
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
