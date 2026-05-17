mod login;

use bevy::app::App;
pub use login::ClientboundPlayLogin;

mod synchronize_player_position;
pub use synchronize_player_position::ClientboundSynchronizePlayerPosition;

mod player_info_update;
pub use player_info_update::ClientboundPlayerInfoUpdate;

mod game_event;
pub use game_event::ClientboundGameEvent;

mod chunk_data;
pub use chunk_data::ClientboundChunkData;

mod set_center_chunk;
pub use set_center_chunk::ClientboundSetCenterChunk;

mod acknowledge_block_change;
pub use acknowledge_block_change::ClientboundAcknowledgeBlockChange;

pub fn register_clientbound_play_packets(app: &mut App) {
    use crate::clientbound_messenger::fwd;
    use bevy::app::PostUpdate;

    app.add_message::<ClientboundPlayLogin>();
    app.add_systems(PostUpdate, fwd::<ClientboundPlayLogin>);

    app.add_message::<ClientboundSynchronizePlayerPosition>();
    app.add_systems(PostUpdate, fwd::<ClientboundSynchronizePlayerPosition>);

    app.add_message::<ClientboundPlayerInfoUpdate>();
    app.add_systems(PostUpdate, fwd::<ClientboundPlayerInfoUpdate>);

    app.add_message::<ClientboundGameEvent>();
    app.add_systems(PostUpdate, fwd::<ClientboundGameEvent>);

    app.add_message::<ClientboundChunkData>();
    app.add_systems(PostUpdate, fwd::<ClientboundChunkData>);

    app.add_message::<ClientboundSetCenterChunk>();
    app.add_systems(PostUpdate, fwd::<ClientboundSetCenterChunk>);

    app.add_message::<ClientboundAcknowledgeBlockChange>();
    app.add_systems(PostUpdate, fwd::<ClientboundAcknowledgeBlockChange>);
}
