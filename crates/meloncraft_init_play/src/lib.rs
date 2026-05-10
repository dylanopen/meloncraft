use bevy::app::{App, Plugin, Update};
use bevy::ecs::message::{MessageReader, MessageWriter};
use meloncraft_core::Identifier;
use meloncraft_packets::clientbound;
use meloncraft_packets::serverbound::configuration::AcknowledgeFinishConfiguration;
use meloncraft_protocol_types::PrefixedArray;

pub struct MeloncraftInitPlayPlugin;

impl Plugin for MeloncraftInitPlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, play_login);
    }
}

fn play_login(
    mut ack_finish_config_pr: MessageReader<AcknowledgeFinishConfiguration>,
    mut login_pw: MessageWriter<clientbound::play::Login>,
) {
    for ack_packet in ack_finish_config_pr.read() {
        login_pw.write(clientbound::play::Login {
            client: ack_packet.client,
            entity_id: 1,
            is_hardcore: false,
            dimension_names: PrefixedArray(vec![
                Identifier("minecraft:overworld".to_string()),
                Identifier("minecraft:the_nether".to_string()),
                Identifier("minecraft:the_end".to_string()),
            ]),
            max_players: 20,
            view_distance: 12,
            simulation_distance: 12,
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: 0,
            dimension_name: Identifier("minecraft:overworld".to_string()),
            hashed_seed: 0x323A_7B9C_4D5E_6F01, // just set to a random seed hash for now
            game_mode: 0,
            previous_game_mode: 1,
            is_debug: false,
            is_flat: false,
            has_death_location: false,
            death_dimension_name: None,
            death_location: None,
            portal_cooldown: 10,
            sea_level: 50,
            enforces_secure_chat: false,
        });
    }
}

