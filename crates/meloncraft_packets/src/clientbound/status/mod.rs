mod status_response;
pub use status_response::StatusResponse;

mod pong;
pub use pong::Pong;

pub fn register_packets(app: &mut bevy::app::App) {
    use crate::clientbound_messenger::fwd;
    use bevy::app::PostUpdate;

    app.add_message::<Pong>();
    app.add_systems(PostUpdate, fwd::<Pong>);

    app.add_message::<StatusResponse>();
    app.add_systems(PostUpdate, fwd::<StatusResponse>);
}
