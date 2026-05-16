mod confirm_teleportation;
pub use confirm_teleportation::ServerboundConfirmTeleportation;

mod query_block_entity_tag;
pub use query_block_entity_tag::ServerboundQueryBlockEntityTag;

mod bundle_item_selected;
pub use bundle_item_selected::ServerboundBundleItemSelected;

mod change_difficulty;
pub use change_difficulty::ServerboundChangeDifficulty;

use bevy::app::App;

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
}
