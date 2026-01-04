mod disconnect;
pub use disconnect::Disconnect;

mod encryption_request;
pub use encryption_request::EncryptionRequest;

mod login_success;
pub use login_success::LoginSuccess;

mod set_compression;
pub use set_compression::SetCompression;

pub fn register_packets(app: &mut bevy::app::App) {
    use crate::clientbound_messenger::fwd;
    use bevy::app::PostUpdate;

    app.add_message::<Disconnect>();
    app.add_systems(PostUpdate, fwd::<Disconnect>);

    app.add_message::<EncryptionRequest>();
    app.add_systems(PostUpdate, fwd::<EncryptionRequest>);

    app.add_message::<LoginSuccess>();
    app.add_systems(PostUpdate, fwd::<LoginSuccess>);

    app.add_message::<SetCompression>();
    app.add_systems(PostUpdate, fwd::<SetCompression>);
}
