mod encryption_response;
pub use encryption_response::ServerboundEncryptionResponse;

mod login_acknowledged;
pub use login_acknowledged::ServerboundLoginAcknowledged;

mod login_start;
pub use login_start::ServerboundLoginStart;

pub fn register_serverbound_login_packets(app: &mut bevy::app::App) {
    use crate::serverbound_messenger::fwd;
    use bevy::app::PreUpdate;

    app.add_message::<ServerboundLoginStart>();
    app.add_systems(PreUpdate, fwd::<ServerboundLoginStart>);

    app.add_message::<ServerboundEncryptionResponse>();
    app.add_systems(PreUpdate, fwd::<ServerboundEncryptionResponse>);

    app.add_message::<ServerboundLoginAcknowledged>();
    app.add_systems(PreUpdate, fwd::<ServerboundLoginAcknowledged>);
}
