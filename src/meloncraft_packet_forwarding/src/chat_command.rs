use bevy::ecs::message::{MessageReader, MessageWriter};
use meloncraft_command::raw::RawCommand;
use meloncraft_packets::ServerboundChatCommand;

pub fn fwd_raw_command(
    mut chat_command_pr: MessageReader<ServerboundChatCommand>,
    mut raw_command_mw: MessageWriter<RawCommand>,
) {
    for chat_command in chat_command_pr.read() {
        let words: Vec<String> = chat_command.command.split_whitespace().map(|s| return s.to_owned()).collect();
        let Some(name) = words.first() else { continue };
        let Some(args) = words.get(1..) else { continue };
        let args = args.to_vec();
        raw_command_mw.write(RawCommand {
            executor: chat_command.client,
            name: name.to_owned(),
            args,
        });
    }
}
