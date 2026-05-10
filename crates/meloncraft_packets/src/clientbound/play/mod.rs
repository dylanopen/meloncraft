mod login;
pub use login::Login;

mod synchronize_player_position;
pub use synchronize_player_position::SynchronizePlayerPosition;

pub fn register_packets(app: &mut bevy::app::App) {
    use crate::clientbound_messenger::fwd;
    use bevy::app::PostUpdate;

    app.add_message::<Login>();
    app.add_systems(PostUpdate, fwd::<Login>);

    app.add_message::<SynchronizePlayerPosition>();
    app.add_systems(PostUpdate, fwd::<SynchronizePlayerPosition>);
}
