use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft_packets::incoming::status::Ping;
use meloncraft_packets::outgoing::status::Pong;

pub fn respond_to_ping_request(mut ping_pr: MessageReader<Ping>, mut pong_pw: MessageWriter<Pong>) {
    for packet in ping_pr.read() {
        pong_pw.write(Pong {
            client: packet.client,
            timestamp: packet.timestamp,
        });
    }
}
