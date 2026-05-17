pub mod handshaken;

use bevy::app::Plugin;
use bevy::app::App;

pub struct MeloncraftHandshakingPlugin;

impl Plugin for MeloncraftHandshakingPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<handshaken::StatusHandshaken>();
        app.add_message::<handshaken::LoginHandshaken>();
        app.add_message::<handshaken::TransferHandshaken>();
    }
}
