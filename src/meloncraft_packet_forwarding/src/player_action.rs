use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft_block::broken::BlockBroken;
use meloncraft_packets::ServerboundPlayerAction;
use meloncraft_player::action_status::PlayerActionStatus;

pub fn fwd_block_broken(
    mut player_action_pr: MessageReader<ServerboundPlayerAction>,
    mut block_broken_mw: MessageWriter<BlockBroken>,
) {
    for player_action in player_action_pr.read() {
        if player_action.status != PlayerActionStatus::StartedDigging { continue; } // ignore any actions which aren't starting digging
        block_broken_mw.write(BlockBroken {
            block_location: player_action.block_location.0, 
        });
    }
}