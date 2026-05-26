use bevy::app::App;

mod disconnect;
pub use disconnect::ClientboundLoginDisconnect;

mod encryption_request;
pub use encryption_request::ClientboundEncryptionRequest;

mod login_success;
pub use login_success::ClientboundLoginSuccess;

mod set_compression;
pub use set_compression::ClientboundSetCompression;

pub fn register_clientbound_login_packets(app: &mut App) {
    use crate::clientbound_messenger::fwd;
    use bevy::app::PostUpdate;

    app.add_message::<ClientboundLoginDisconnect>();
    app.add_systems(PostUpdate, fwd::<ClientboundLoginDisconnect>);

    app.add_message::<ClientboundEncryptionRequest>();
    app.add_systems(PostUpdate, fwd::<ClientboundEncryptionRequest>);

    app.add_message::<ClientboundLoginSuccess>();
    app.add_systems(PostUpdate, fwd::<ClientboundLoginSuccess>);

    app.add_message::<ClientboundSetCompression>();
    app.add_systems(PostUpdate, fwd::<ClientboundSetCompression>);
}
