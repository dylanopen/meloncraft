mod encryption_response;
pub use encryption_response::EncryptionResponse;

mod login_acknowledged;
pub use login_acknowledged::LoginAcknowledged;

mod login_start;
pub use login_start::LoginStart;

pub fn register_packets(app: &mut bevy::app::App) {
    use crate::serverbound_messenger::fwd;
    use bevy::app::PreUpdate;

    app.add_message::<LoginStart>();
    app.add_systems(PreUpdate, fwd::<LoginStart>);

    app.add_message::<EncryptionResponse>();
    app.add_systems(PreUpdate, fwd::<EncryptionResponse>);

    app.add_message::<LoginAcknowledged>();
    app.add_systems(PreUpdate, fwd::<LoginAcknowledged>);
}
