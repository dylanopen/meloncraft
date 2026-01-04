mod handshake;
pub use handshake::Intention;

pub fn register_packets(app: &mut bevy::app::App) {
    use crate::serverbound_messenger::fwd;
    use bevy::app::PreUpdate;

    app.add_message::<Intention>();
    app.add_systems(PreUpdate, fwd::<Intention>);
}
