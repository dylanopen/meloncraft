use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft_client::intention_type::IntentionType;
use meloncraft_handshaking::handshaken::{LoginHandshaken, StatusHandshaken, TransferHandshaken};
use meloncraft_packets::ServerboundIntention;

pub fn fwd_handshaken(
    mut handshake_pr: MessageReader<ServerboundIntention>,
    mut status_received_mw: MessageWriter<StatusHandshaken>,
    mut login_received_mw: MessageWriter<LoginHandshaken>,
    mut transfer_received_mw: MessageWriter<TransferHandshaken>,
) {
    for packet in handshake_pr.read() {
        match packet.next_state {
            IntentionType::Status => {
                status_received_mw.write(StatusHandshaken {
                    player: packet.client,
                });
            }
            IntentionType::Login => {
                login_received_mw.write(LoginHandshaken {
                    player: packet.client,
                });
            }
            IntentionType::Transfer => {
                transfer_received_mw.write(TransferHandshaken {
                    player: packet.client,
                });
            }
        }
    }
}
