use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft_packets::ClientboundPong;
use meloncraft_packets::ServerboundStatusPing;

pub fn respond_to_ping_request(
    mut ping_pr: MessageReader<ServerboundStatusPing>,
    mut pong_pw: MessageWriter<ClientboundPong>,
) {
    for packet in ping_pr.read() {
        pong_pw.write(ClientboundPong {
            client: packet.client,
            timestamp: packet.timestamp,
        });
    }
}
