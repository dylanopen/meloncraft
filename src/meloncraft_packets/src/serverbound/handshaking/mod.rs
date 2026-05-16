mod handshake;

use bevy::app::App;
pub use handshake::ServerboundIntention;

pub fn register_serverbound_handshaking_packets(app: &mut App) {
    use crate::serverbound_messenger::fwd;
    use bevy::app::PreUpdate;

    app.add_message::<ServerboundIntention>();
    app.add_systems(PreUpdate, fwd::<ServerboundIntention>);
}
