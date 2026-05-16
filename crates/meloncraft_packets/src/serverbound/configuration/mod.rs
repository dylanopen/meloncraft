mod client_information;
pub use client_information::ServerboundClientInformation;

mod cookie_response;
pub use cookie_response::ServerboundCookieResponse;

mod finish_configuration;
pub use finish_configuration::ServerboundFinishConfiguration;

mod keep_alive;
pub use keep_alive::ServerboundConfigurationKeepAlive;

mod pong;
pub use pong::ServerboundConfigurationPong;

mod resource_pack_response;
pub use resource_pack_response::ServerboundResourcePackResponse;

mod select_known_packs;
pub use select_known_packs::ServerboundSelectKnownPacks;

mod accept_code_of_conduct;
pub use accept_code_of_conduct::ServerboundAcceptCodeOfConduct;

mod acknowledge_finish_configuration;
pub use acknowledge_finish_configuration::ServerboundAcknowledgeFinishConfiguration;

pub fn register_serverbound_configuration_packets(app: &mut bevy::app::App) {
    use crate::serverbound_messenger::fwd;
    use bevy::app::PreUpdate;
    
    app.add_message::<ServerboundClientInformation>();
    app.add_systems(PreUpdate, fwd::<ServerboundClientInformation>);

    app.add_message::<ServerboundCookieResponse>();
    app.add_systems(PreUpdate, fwd::<ServerboundCookieResponse>);

    app.add_message::<ServerboundFinishConfiguration>();
    app.add_systems(PreUpdate, fwd::<ServerboundFinishConfiguration>);

    app.add_message::<ServerboundConfigurationKeepAlive>();
    app.add_systems(PreUpdate, fwd::<ServerboundConfigurationKeepAlive>);

    app.add_message::<ServerboundConfigurationPong>();
    app.add_systems(PreUpdate, fwd::<ServerboundConfigurationPong>);

    app.add_message::<ServerboundResourcePackResponse>();
    app.add_systems(PreUpdate, fwd::<ServerboundResourcePackResponse>);

    app.add_message::<ServerboundSelectKnownPacks>();
    app.add_systems(PreUpdate, fwd::<ServerboundSelectKnownPacks>);

    app.add_message::<ServerboundAcceptCodeOfConduct>();
    app.add_systems(PreUpdate, fwd::<ServerboundAcceptCodeOfConduct>);

    app.add_message::<ServerboundAcknowledgeFinishConfiguration>();
    app.add_systems(PreUpdate, fwd::<ServerboundAcknowledgeFinishConfiguration>);
}
