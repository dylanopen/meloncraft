use bevy::app::App;

mod confirm_teleportation;
pub use confirm_teleportation::ServerboundConfirmTeleportation;

mod query_block_entity_tag;
pub use query_block_entity_tag::ServerboundQueryBlockEntityTag;

mod bundle_item_selected;
pub use bundle_item_selected::ServerboundBundleItemSelected;

mod change_difficulty;
pub use change_difficulty::ServerboundChangeDifficulty;

mod change_gamemode;
pub use change_gamemode::ServerboundChangeGamemode;

mod acknowledge_chat;
pub use acknowledge_chat::ServerboundAcknowledgeChat;

mod set_player_position;
pub use set_player_position::ServerboundSetPlayerPosition;

mod player_action;
pub use player_action::ServerboundPlayerAction;

mod chat_command;
pub use chat_command::ServerboundChatCommand;

pub fn register_serverbound_play_packets(app: &mut App) {
    use crate::serverbound_messenger::fwd;
    use bevy::app::PreUpdate;

    app.add_message::<ServerboundConfirmTeleportation>();
    app.add_systems(PreUpdate, fwd::<ServerboundConfirmTeleportation>);

    app.add_message::<ServerboundQueryBlockEntityTag>();
    app.add_systems(PreUpdate, fwd::<ServerboundQueryBlockEntityTag>);

    app.add_message::<ServerboundBundleItemSelected>();
    app.add_systems(PreUpdate, fwd::<ServerboundBundleItemSelected>);

    app.add_message::<ServerboundChangeDifficulty>();
    app.add_systems(PreUpdate, fwd::<ServerboundChangeDifficulty>);

    app.add_message::<ServerboundChangeGamemode>();
    app.add_systems(PreUpdate, fwd::<ServerboundChangeGamemode>);

    app.add_message::<ServerboundAcknowledgeChat>();
    app.add_systems(PreUpdate, fwd::<ServerboundAcknowledgeChat>);

    app.add_message::<ServerboundSetPlayerPosition>();
    app.add_systems(PreUpdate, fwd::<ServerboundSetPlayerPosition>);

    app.add_message::<ServerboundPlayerAction>();
    app.add_systems(PreUpdate, fwd::<ServerboundPlayerAction>);

    app.add_message::<ServerboundChatCommand>();
    app.add_systems(PreUpdate, fwd::<ServerboundChatCommand>);
}
