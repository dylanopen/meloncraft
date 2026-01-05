use bevy::prelude::MessageReader;
use meloncraft_packets::serverbound::login::LoginAcknowledged;

pub fn login_acknowledged_listener(mut login_acknowledged_pr: MessageReader<LoginAcknowledged>) {
    for _packet in login_acknowledged_pr.read() {}
}
