mod client_information;
pub use client_information::ClientInformation;

mod cookie_response;
pub use cookie_response::CookieResponse;

mod finish_configuration;
pub use finish_configuration::FinishConfiguration;

mod keep_alive;
pub use keep_alive::KeepAlive;

mod pong;
pub use pong::Pong;

mod resource_pack_response;
pub use resource_pack_response::ResourcePackResponse;

mod select_known_packs;
pub use select_known_packs::SelectKnownPacks;

pub fn register_packets(app: &mut bevy::app::App) {
    use crate::serverbound_messenger::fwd;
    use bevy::app::PreUpdate;

    app.add_message::<ClientInformation>();
    app.add_systems(PreUpdate, fwd::<ClientInformation>);

    app.add_message::<CookieResponse>();
    app.add_systems(PreUpdate, fwd::<CookieResponse>);

    app.add_message::<FinishConfiguration>();
    app.add_systems(PreUpdate, fwd::<FinishConfiguration>);

    app.add_message::<KeepAlive>();
    app.add_systems(PreUpdate, fwd::<KeepAlive>);

    app.add_message::<Pong>();
    app.add_systems(PreUpdate, fwd::<Pong>);

    app.add_message::<ResourcePackResponse>();
    app.add_systems(PreUpdate, fwd::<ResourcePackResponse>);

    app.add_message::<SelectKnownPacks>();
    app.add_systems(PreUpdate, fwd::<SelectKnownPacks>);
}
