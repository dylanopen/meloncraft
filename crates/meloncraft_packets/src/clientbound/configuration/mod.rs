pub mod add_resource_pack;
pub use add_resource_pack::AddResourcePack;

pub mod clear_dialog;
pub use clear_dialog::ClearDialog;

pub mod code_of_conduct;
pub use code_of_conduct::CodeOfConduct;

pub mod cookie_request;
pub use cookie_request::CookieRequest;

pub mod custom_report_details;
pub use custom_report_details::CustomReportDetails;

pub mod disconnect;
pub use disconnect::Disconnect;

pub mod finish_configuration;
pub use finish_configuration::FinishConfiguration;

pub mod keep_alive;
pub use keep_alive::KeepAlive;

pub mod ping;
pub use ping::Ping;

pub mod registry_data;
pub use registry_data::RegistryData;

pub mod remove_resource_pack;
pub use remove_resource_pack::RemoveResourcePack;

pub mod reset_chat;
pub use reset_chat::ResetChat;

pub mod select_known_packs;
pub use select_known_packs::SelectKnownPacks;

pub mod server_links;
pub use server_links::ServerLinks;

pub mod set_feature_flags;
pub use set_feature_flags::SetFeatureFlags;

pub mod show_dialog;
pub use show_dialog::ShowDialog;

pub mod store_cookie;
pub use store_cookie::StoreCookie;

pub mod transfer;
pub use transfer::Transfer;

pub mod update_tags;
pub use update_tags::UpdateTags;

pub fn register_packets(app: &mut bevy::app::App) {
    use crate::clientbound_messenger::fwd;
    use bevy::app::PostUpdate;

    app.add_message::<AddResourcePack>();
    app.add_systems(PostUpdate, fwd::<AddResourcePack>);

    app.add_message::<ClearDialog>();
    app.add_systems(PostUpdate, fwd::<ClearDialog>);

    app.add_message::<CodeOfConduct>();
    app.add_systems(PostUpdate, fwd::<CodeOfConduct>);

    app.add_message::<CookieRequest>();
    app.add_systems(PostUpdate, fwd::<CookieRequest>);

    app.add_message::<CustomReportDetails>();
    app.add_systems(PostUpdate, fwd::<CustomReportDetails>);

    app.add_message::<Disconnect>();
    app.add_systems(PostUpdate, fwd::<Disconnect>);

    app.add_message::<FinishConfiguration>();
    app.add_systems(PostUpdate, fwd::<FinishConfiguration>);

    app.add_message::<KeepAlive>();
    app.add_systems(PostUpdate, fwd::<KeepAlive>);

    app.add_message::<Ping>();
    app.add_systems(PostUpdate, fwd::<Ping>);

    app.add_message::<RegistryData>();
    app.add_systems(PostUpdate, fwd::<RegistryData>);

    app.add_message::<RemoveResourcePack>();
    app.add_systems(PostUpdate, fwd::<RemoveResourcePack>);

    app.add_message::<ResetChat>();
    app.add_systems(PostUpdate, fwd::<ResetChat>);

    app.add_message::<SelectKnownPacks>();
    app.add_systems(PostUpdate, fwd::<SelectKnownPacks>);

    app.add_message::<ServerLinks>();
    app.add_systems(PostUpdate, fwd::<ServerLinks>);

    app.add_message::<SetFeatureFlags>();
    app.add_systems(PostUpdate, fwd::<SetFeatureFlags>);

    app.add_message::<ShowDialog>();
    app.add_systems(PostUpdate, fwd::<ShowDialog>);

    app.add_message::<StoreCookie>();
    app.add_systems(PostUpdate, fwd::<StoreCookie>);

    app.add_message::<Transfer>();
    app.add_systems(PostUpdate, fwd::<Transfer>);

    app.add_message::<UpdateTags>();
    app.add_systems(PostUpdate, fwd::<UpdateTags>);
}
