use bevy::ecs::message::{MessageReader, MessageWriter};
use meloncraft_chat::sent::PlayerSentChatMessage;
use meloncraft_packets::ServerboundChat;

pub fn fwd_player_sent(
    mut chat_pr: MessageReader<ServerboundChat>,
    mut player_sent_chat_mw: MessageWriter<PlayerSentChatMessage>,
) {
    for chat in chat_pr.read() {
        player_sent_chat_mw.write(PlayerSentChatMessage {
            sender: chat.client,
            timestamp: chat.timestamp,
            message: chat.message.clone(),
        });
    }
}
