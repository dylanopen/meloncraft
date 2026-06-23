use bevy::app::App;

mod login;
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

mod player_abilities;
pub use player_abilities::ClientboundPlayerAbilities;

mod set_time;
pub use set_time::ClientboundSetTime;

mod set_experience;
pub use set_experience::ClientboundSetExperience;

mod system_chat;
pub use system_chat::ClientboundSystemChat;

mod set_title_text;
pub use set_title_text::ClientboundSetTitleText;

mod boss_event;
pub use boss_event::BossEventAction;
pub use boss_event::ClientboundBossEvent;

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

    app.add_message::<ClientboundPlayerAbilities>();
    app.add_systems(PostUpdate, fwd::<ClientboundPlayerAbilities>);

    app.add_message::<ClientboundSetTime>();
    app.add_systems(PostUpdate, fwd::<ClientboundSetTime>);

    app.add_message::<ClientboundSetExperience>();
    app.add_systems(PostUpdate, fwd::<ClientboundSetExperience>);

    app.add_message::<ClientboundSystemChat>();
    app.add_systems(PostUpdate, fwd::<ClientboundSystemChat>);

    app.add_message::<ClientboundSetTitleText>();
    app.add_systems(PostUpdate, fwd::<ClientboundSetTitleText>);

    app.add_message::<ClientboundBossEvent>();
    app.add_systems(PostUpdate, fwd::<ClientboundBossEvent>);
}
