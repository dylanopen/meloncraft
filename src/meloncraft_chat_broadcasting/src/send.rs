use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Query;
use meloncraft_chat::send::SendChatMessage;
use meloncraft_chat::sent::PlayerSentChatMessage;
use meloncraft_logger::warnlog;
use meloncraft_nbt::NbtString;
use meloncraft_player::GameProfile;
use meloncraft_text::NbtText;

pub fn send_player_chat(
    mut player_sent_chat_mr: MessageReader<PlayerSentChatMessage>,
    mut send_chat_mw: MessageWriter<SendChatMessage>,
    game_profile_q: Query<(Entity, &GameProfile)>,
) {
    let clients: Vec<Entity> = game_profile_q
        .iter()
        .map(|profile| return profile.0)
        .collect();
    for chat in player_sent_chat_mr.read() {
        let Ok(sender) = game_profile_q.get(chat.sender) else {
            warnlog!(
                "Received PlayerSentChatMessage from entity {:?} without a GameProfile. Skipping message.",
                chat.sender
            );
            continue;
        };
        let username = sender.1.username.clone();
        let message = chat.message.clone();
        let output = format!("<{username}> {message}");
        send_chat_mw.write(SendChatMessage {
            receivers: clients.clone(),
            message: NbtText::Plain(NbtString(output)),
        });
    }
}

