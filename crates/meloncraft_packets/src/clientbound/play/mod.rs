mod login;
pub use login::Login;

pub fn register_packets(app: &mut bevy::app::App) {
    use crate::clientbound_messenger::fwd;
    use bevy::app::PostUpdate;

    app.add_message::<Login>();
    app.add_systems(PostUpdate, fwd::<Login>);
}
