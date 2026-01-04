mod status_request;
pub use status_request::StatusRequest;

mod ping;
pub use ping::Ping;

pub fn register_packets(app: &mut bevy::app::App) {
    use crate::serverbound_messenger::fwd;
    use bevy::app::PreUpdate;

    app.add_message::<StatusRequest>();
    app.add_systems(PreUpdate, fwd::<StatusRequest>);

    app.add_message::<Ping>();
    app.add_systems(PreUpdate, fwd::<Ping>);
}
