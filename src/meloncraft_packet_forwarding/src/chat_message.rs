use bevy::ecs::message::{MessageReader, MessageWriter};
use meloncraft_chat::send::{ClearTitles, SendChatMessage, SendTitleMessage};
use meloncraft_chat::sent::PlayerSentChatMessage;
use meloncraft_chat::title::TitlePosition;
use meloncraft_packets::{ClientboundClearTitles, ClientboundSetActionbarText, ClientboundSetSubtitleText, ClientboundSetTitleAnimationTimes, ClientboundSetTitleText, ClientboundSystemChat, ServerboundChat};

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

pub fn fwd_send_title(
    mut send_title_mr: MessageReader<SendTitleMessage>,
    mut set_title_animation_times_pw: MessageWriter<ClientboundSetTitleAnimationTimes>,
    mut set_title_text_pw: MessageWriter<ClientboundSetTitleText>,
    mut set_subtitle_text_pw: MessageWriter<ClientboundSetSubtitleText>,
    mut set_actionbar_text_pw: MessageWriter<ClientboundSetActionbarText>,
) {
    for send_title in send_title_mr.read() {
        for receiver in send_title.receivers.clone() {
            if let Some(times) = &send_title.times {
                set_title_animation_times_pw.write(ClientboundSetTitleAnimationTimes { client: receiver, fade_in_ticks: times.fade_in_ticks, stay_ticks: times.stay_ticks, fade_out_ticks: times.fade_out_ticks });
            }

            match send_title.position {
                TitlePosition::Title => {
                    set_title_text_pw.write(ClientboundSetTitleText {
                        client: receiver,
                        title: send_title.message.clone(),
                    });
                },
                TitlePosition::Subtitle => {
                    set_subtitle_text_pw.write(ClientboundSetSubtitleText {
                        client: receiver,
                        title: send_title.message.clone(),
                    });
                },
                TitlePosition::Actionbar => {
                    set_actionbar_text_pw.write(ClientboundSetActionbarText {
                        client: receiver,
                        title: send_title.message.clone(),
                    });
                },
            }
        }
    }
}

pub fn fwd_clear_titles(
    mut clear_title_mr: MessageReader<ClearTitles>,
    mut clear_titles_pw: MessageWriter<ClientboundClearTitles>,
) {
    for send_title in clear_title_mr.read() {
        for receiver in send_title.receivers.clone() {
            // for now, we always 'reset'.
            clear_titles_pw.write(ClientboundClearTitles { client: receiver, reset: true });
        }
    }
}
