mod status_request;

use bevy::app::App;
pub use status_request::ServerboundStatusRequest;

mod ping;
pub use ping::ServerboundStatusPing;

pub fn register_serverbound_status_packets(app: &mut App) {
    use crate::serverbound_messenger::fwd;
    use bevy::app::PreUpdate;

    app.add_message::<ServerboundStatusRequest>();
    app.add_systems(PreUpdate, fwd::<ServerboundStatusRequest>);

    app.add_message::<ServerboundStatusPing>();
    app.add_systems(PreUpdate, fwd::<ServerboundStatusPing>);
}
