mod confirm_teleportation;
pub use confirm_teleportation::ConfirmTeleportation;

pub fn register_packets(app: &mut bevy::app::App) {
    use crate::serverbound_messenger::fwd;
    use bevy::app::PreUpdate;

    app.add_message::<ConfirmTeleportation>();
    app.add_systems(PreUpdate, fwd::<ConfirmTeleportation>);
}
