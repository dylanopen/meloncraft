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

mod block_update;
pub use block_update::ClientboundBlockUpdate;

mod change_difficulty;
pub use change_difficulty::ClientboundChangeDifficulty;

mod set_default_spawn_position;
pub use set_default_spawn_position::ClientboundSetDefaultSpawnPosition;

mod set_ticking_state;
pub use set_ticking_state::ClientboundSetTickingState;

mod set_border_center;
pub use set_border_center::ClientboundSetBorderCenter;

mod set_border_size;
pub use set_border_size::ClientboundSetBorderSize;

mod set_border_warning_delay;
pub use set_border_warning_delay::ClientboundSetBorderWarningDelay;

mod set_border_warning_distance;
pub use set_border_warning_distance::ClientboundSetBorderWarningDistance;

mod server_data;
pub use server_data::ClientboundServerData;

mod set_health;
pub use set_health::ClientboundSetHealth;

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

    app.add_message::<ClientboundBlockUpdate>();
    app.add_systems(PostUpdate, fwd::<ClientboundBlockUpdate>);

    app.add_message::<ClientboundChangeDifficulty>();
    app.add_systems(PostUpdate, fwd::<ClientboundChangeDifficulty>);

    app.add_message::<ClientboundSetDefaultSpawnPosition>();
    app.add_systems(PostUpdate, fwd::<ClientboundSetDefaultSpawnPosition>);

    app.add_message::<ClientboundSetTickingState>();
    app.add_systems(PostUpdate, fwd::<ClientboundSetTickingState>);

    app.add_message::<ClientboundSetBorderCenter>();
    app.add_systems(PostUpdate, fwd::<ClientboundSetBorderCenter>);

    app.add_message::<ClientboundSetBorderSize>();
    app.add_systems(PostUpdate, fwd::<ClientboundSetBorderSize>);

    app.add_message::<ClientboundSetBorderWarningDelay>();
    app.add_systems(PostUpdate, fwd::<ClientboundSetBorderWarningDelay>);

    app.add_message::<ClientboundSetBorderWarningDistance>();
    app.add_systems(PostUpdate, fwd::<ClientboundSetBorderWarningDistance>);

    app.add_message::<ClientboundServerData>();
    app.add_systems(PostUpdate, fwd::<ClientboundServerData>);

    app.add_message::<ClientboundSetHealth>();
    app.add_systems(PostUpdate, fwd::<ClientboundSetHealth>);
}
