use bevy::app::{App, Update};
use bevy::MinimalPlugins;
use bevy::prelude::MessageReader;
use meloncraft::handshaking::MeloncraftHandshakingPlugin;
use meloncraft::network::MeloncraftNetworkPlugin;
use meloncraft::incoming_packet_generators::MeloncraftPacketGeneratorsPlugin;
use meloncraft::packets::incoming::handshaking::Intention;
use meloncraft::packets::incoming::status::StatusRequest;
use meloncraft::packets::MeloncraftPacketsPlugin;

pub fn main() {

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(MeloncraftNetworkPlugin);
    app.add_plugins(MeloncraftPacketsPlugin);
    app.add_plugins(MeloncraftPacketGeneratorsPlugin);
    app.add_plugins(MeloncraftHandshakingPlugin);

    app.add_systems(Update, respond_to_handshake);
    app.add_systems(Update, respond_to_status_request);

    app.run();
}

fn respond_to_handshake(
    mut mr: MessageReader<Intention>,
) {
    for msg in mr.read() {
        println!("Handshake received: {:?}", msg);
    }
}

fn respond_to_status_request(
    mut mr: MessageReader<StatusRequest>,
) {
    for msg in mr.read() {
        dbg!(msg);
    }
}
