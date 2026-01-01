use bevy::MinimalPlugins;
use bevy::app::PluginGroup;
use bevy::app::{App, ScheduleRunnerPlugin, Update};
use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft::handshaking::MeloncraftHandshakingPlugin;
use meloncraft::network::MeloncraftNetworkPlugin;
use meloncraft::network::packet::IncomingNetworkPacketReceived;
use meloncraft::packet_messengers::MeloncraftPacketGeneratorsPlugin;
use meloncraft::packets::MeloncraftPacketsPlugin;
use meloncraft::packets::incoming::handshaking::Intention;
use meloncraft::packets::incoming::status::StatusRequest;
use meloncraft::packets::outgoing::status::StatusResponse;
use std::time::Duration;

pub fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(500))));
    app.add_plugins(MeloncraftNetworkPlugin);
    app.add_plugins(MeloncraftPacketsPlugin);
    app.add_plugins(MeloncraftPacketGeneratorsPlugin);
    app.add_plugins(MeloncraftHandshakingPlugin);

    app.add_systems(Update, respond_to_status_request);
    app.add_systems(Update, respond_to_packet);
    app.add_systems(Update, respond_to_intention);
    app.add_systems(Update, say_hi);

    app.run();
}

fn respond_to_intention(mut mr: MessageReader<Intention>) {
    for msg in mr.read() {
        dbg!(msg);
    }
}

fn respond_to_packet(mut mr: MessageReader<IncomingNetworkPacketReceived>) {
    for msg in mr.read() {
        // dbg!(&msg.packet);
    }
}

fn say_hi() {}

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
            version_name: "Meloncraft".to_owned(),
            version_protocol: 773,
        });
    }
}
