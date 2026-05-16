mod status_response;
pub use status_response::ClientboundStatusResponse;

mod pong;
pub use pong::ClientboundPong;

pub fn register_clientbound_status_packets(app: &mut bevy::app::App) {
    use crate::clientbound_messenger::fwd;
    use bevy::app::PostUpdate;

    app.add_message::<ClientboundPong>();
    app.add_systems(PostUpdate, fwd::<ClientboundPong>);

    app.add_message::<ClientboundStatusResponse>();
    app.add_systems(PostUpdate, fwd::<ClientboundStatusResponse>);
}
