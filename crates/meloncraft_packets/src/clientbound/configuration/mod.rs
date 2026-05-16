pub mod add_resource_pack;
pub use add_resource_pack::ClientboundAddResourcePack;

pub mod clear_dialog;
pub use clear_dialog::ClientboundClearDialog;

pub mod code_of_conduct;
pub use code_of_conduct::ClientboundCodeOfConduct;

pub mod cookie_request;
pub use cookie_request::ClientboundCookieRequest;

pub mod custom_report_details;
pub use custom_report_details::ClientboundCustomReportDetails;

pub mod disconnect;
pub use disconnect::ClientboundConfigurationDisconnect;

pub mod finish_configuration;
pub use finish_configuration::ClientboundFinishConfiguration;

pub mod keep_alive;
pub use keep_alive::ClientboundKeepAlive;

pub mod ping;
pub use ping::ClientboundPing;

pub mod registry_data;
pub use registry_data::ClientboundRegistryData;

pub mod remove_resource_pack;
pub use remove_resource_pack::ClientboundRemoveResourcePack;

pub mod reset_chat;
pub use reset_chat::ClientboundResetChat;

pub mod select_known_packs;
pub use select_known_packs::ClientboundSelectKnownPacks;

pub mod server_links;
pub use server_links::ClientboundServerLinks;

pub mod set_feature_flags;
pub use set_feature_flags::ClientboundSetFeatureFlags;

pub mod show_dialog;
pub use show_dialog::ClientboundShowDialog;

pub mod store_cookie;
pub use store_cookie::ClientboundStoreCookie;

pub mod transfer;
pub use transfer::ClientboundTransfer;

pub mod update_tags;
pub use update_tags::ClientboundUpdateTags;

pub fn register_clientbound_configuration_packets(app: &mut bevy::app::App) {
    use crate::clientbound_messenger::fwd;
    use bevy::app::PostUpdate;

    app.add_message::<ClientboundAddResourcePack>();
    app.add_systems(PostUpdate, fwd::<ClientboundAddResourcePack>);

    app.add_message::<ClientboundClearDialog>();
    app.add_systems(PostUpdate, fwd::<ClientboundClearDialog>);

    app.add_message::<ClientboundCodeOfConduct>();
    app.add_systems(PostUpdate, fwd::<ClientboundCodeOfConduct>);

    app.add_message::<ClientboundCookieRequest>();
    app.add_systems(PostUpdate, fwd::<ClientboundCookieRequest>);

    app.add_message::<ClientboundCustomReportDetails>();
    app.add_systems(PostUpdate, fwd::<ClientboundCustomReportDetails>);

    app.add_message::<ClientboundConfigurationDisconnect>();
    app.add_systems(PostUpdate, fwd::<ClientboundConfigurationDisconnect>);

    app.add_message::<ClientboundFinishConfiguration>();
    app.add_systems(PostUpdate, fwd::<ClientboundFinishConfiguration>);

    app.add_message::<ClientboundKeepAlive>();
    app.add_systems(PostUpdate, fwd::<ClientboundKeepAlive>);

    app.add_message::<ClientboundPing>();
    app.add_systems(PostUpdate, fwd::<ClientboundPing>);

    app.add_message::<ClientboundRegistryData>();
    app.add_systems(PostUpdate, fwd::<ClientboundRegistryData>);

    app.add_message::<ClientboundRemoveResourcePack>();
    app.add_systems(PostUpdate, fwd::<ClientboundRemoveResourcePack>);

    app.add_message::<ClientboundResetChat>();
    app.add_systems(PostUpdate, fwd::<ClientboundResetChat>);

    app.add_message::<ClientboundSelectKnownPacks>();
    app.add_systems(PostUpdate, fwd::<ClientboundSelectKnownPacks>);

    app.add_message::<ClientboundServerLinks>();
    app.add_systems(PostUpdate, fwd::<ClientboundServerLinks>);

    app.add_message::<ClientboundSetFeatureFlags>();
    app.add_systems(PostUpdate, fwd::<ClientboundSetFeatureFlags>);

    app.add_message::<ClientboundShowDialog>();
    app.add_systems(PostUpdate, fwd::<ClientboundShowDialog>);

    app.add_message::<ClientboundStoreCookie>();
    app.add_systems(PostUpdate, fwd::<ClientboundStoreCookie>);

    app.add_message::<ClientboundTransfer>();
    app.add_systems(PostUpdate, fwd::<ClientboundTransfer>);

    app.add_message::<ClientboundUpdateTags>();
    app.add_systems(PostUpdate, fwd::<ClientboundUpdateTags>);
}
