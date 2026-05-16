mod confirm_teleportation;

use bevy::app::App;
pub use confirm_teleportation::ServerboundConfirmTeleportation;

pub fn register_serverbound_play_packets(app: &mut App) {
    use crate::serverbound_messenger::fwd;
    use bevy::app::PreUpdate;

    app.add_message::<ServerboundConfirmTeleportation>();
    app.add_systems(PreUpdate, fwd::<ServerboundConfirmTeleportation>);
}
