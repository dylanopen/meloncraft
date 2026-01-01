use bevy::app::{App, Update};
use bevy::MinimalPlugins;
use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft::handshaking::MeloncraftHandshakingPlugin;
use meloncraft::network::MeloncraftNetworkPlugin;
use meloncraft::packet_messengers::MeloncraftPacketGeneratorsPlugin;
use meloncraft::packets::incoming::handshaking::Intention;
use meloncraft::packets::incoming::status::StatusRequest;
use meloncraft::packets::MeloncraftPacketsPlugin;
use meloncraft::packets::outgoing::status::StatusResponse;

pub fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(MeloncraftNetworkPlugin);
    app.add_plugins(MeloncraftPacketsPlugin);
    app.add_plugins(MeloncraftPacketGeneratorsPlugin);
    app.add_plugins(MeloncraftHandshakingPlugin);

    app.add_systems(Update, respond_to_packet);
    app.add_systems(Update, respond_to_status_request);
    // app.add_systems(Update, say_hi);

    app.run();
}

fn say_hi() {
    println!("hello there!");
}

fn respond_to_packet(
    mut mr: MessageReader<Intention>,
    mut mw: MessageWriter<StatusResponse>,
) {
    for msg in mr.read() {
        println!("Handshake received: {:?}", msg);

        mw.write(StatusResponse {
            client: msg.client,
            description: "Meloncraft".to_owned(),
            enforces_secure_chat: false,
            max_players: 1000,
            online_players: 678,
            version_name: "Meloncraft Ultimatum".to_owned(),
            version_protocol: 773,
        });
    }
}

fn respond_to_status_request(
    mut mr: MessageReader<StatusRequest>,
    mut mw: MessageWriter<StatusResponse>,
) {
    for msg in mr.read() {
        mw.write(StatusResponse {
            client: msg.client,
            description: "Meloncraft".to_owned(),
            enforces_secure_chat: false,
            max_players: 1000,
            online_players: 678,
            version_name: "Meloncraft Ultimatum".to_owned(),
            version_protocol: 773,
        });
        dbg!();
    }
}
