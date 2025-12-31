use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft_network::packet::IncomingNetworkPacketReceived;
use meloncraft_packets::incoming;
use meloncraft_packets::IncomingPacket;
use meloncraft_packets::incoming::handshaking::Intention;

pub fn fwd_handshake(
    mut all_packets: MessageReader<IncomingNetworkPacketReceived>,
    mut handshake_mw: MessageWriter<Intention>
) {
    for network_packet in all_packets.read() {
        if let Some(packet) = Intention::from_packet(&network_packet.packet) {
            handshake_mw.write(packet);
        }
    }
}