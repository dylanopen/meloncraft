use bevy::ecs::message::{MessageReader, MessageWriter};
use meloncraft_chat::send::SendChatMessage;
use meloncraft_chat::sent::PlayerSentChatMessage;
use meloncraft_packets::{ClientboundSystemChat, ServerboundChat};

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

pub fn fwd_send_chat(
    mut send_chat_mr: MessageReader<SendChatMessage>,
    mut system_chat_pw: MessageWriter<ClientboundSystemChat>,
) {
    for send_chat in send_chat_mr.read() {
        for receiver in send_chat.receivers.clone() {
            system_chat_pw.write(ClientboundSystemChat {
                client: receiver,
                message: send_chat.message.clone(),
                overlay: false,
            });
        }
    }
}
