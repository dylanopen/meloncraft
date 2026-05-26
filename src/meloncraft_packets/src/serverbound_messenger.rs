use crate::ServerboundPacket;
use crate::network_messages::ServerboundNetworkPacketReceived;
use bevy::prelude::{Message, MessageReader, MessageWriter};
use core::fmt::Debug;

pub fn fwd<T: Message + ServerboundPacket + Debug>(
    mut all_packets: MessageReader<ServerboundNetworkPacketReceived>,
    mut packet_writer: MessageWriter<T>,
) {
    for network_packet in all_packets.read() {
        if T::validate(&network_packet.packet).is_err() {
            continue;
        }
        if let Some(packet) = T::deserialize(network_packet.packet.clone()) {
            // TODO: avoid cloning this
            packet_writer.write(packet);
        }
    }
}
