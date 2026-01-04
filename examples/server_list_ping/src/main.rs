use bevy::MinimalPlugins;
use bevy::app::{App, ScheduleRunnerPlugin};
use bevy::app::{PluginGroup, Update};
use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft::handshaking::MeloncraftHandshakingPlugin;
use meloncraft::network::MeloncraftNetworkPlugin;
use meloncraft::packets::MeloncraftPacketsPlugin;
use meloncraft::packets::clientbound::login::Disconnect;
use meloncraft::packets::incoming::login::LoginStart;
use meloncraft::protocol_types::JsonText;
use meloncraft::server_list::MeloncraftServerListPlugin;
use meloncraft::server_list::max_players::MaxPlayers;
use meloncraft::server_list::motd::Motd;
use meloncraft::server_list::online_players::OnlinePlayers;
use std::time::Duration;

pub fn main() {
    let mut app = App::new();

    app.insert_resource(Motd("Meloncraft server list example".to_owned()));
    app.insert_resource(OnlinePlayers(17));
    app.insert_resource(MaxPlayers(32));

    app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(50))));
    app.add_plugins(MeloncraftNetworkPlugin);
    app.add_plugins(MeloncraftPacketsPlugin);
    app.add_plugins(MeloncraftHandshakingPlugin);
    app.add_plugins(MeloncraftServerListPlugin);

    app.add_systems(Update, login_listener);

    app.run();
}

fn login_listener(mut pr: MessageReader<LoginStart>, mut pw: MessageWriter<Disconnect>) {
    for packet in pr.read() {
        println!(
            "{} tried to log-in! This example only adds serverlist functionality.",
            packet.name
        );
        pw.write(Disconnect {
            client: packet.client,
            reason: JsonText {
                data:
                    "{text: \"This example only supports server list pings - no joining yet ;)\"}"
                        .to_owned(),
            },
        });
    }
}
