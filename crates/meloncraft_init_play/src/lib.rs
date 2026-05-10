use bevy::app::{App, Plugin, Update};
use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::schedule::IntoScheduleConfigs as _;
use bevy::ecs::system::{Query, ResMut};
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::Identifier;
use meloncraft_packets::clientbound;
use meloncraft_packets::serverbound::configuration::AcknowledgeFinishConfiguration;
use meloncraft_protocol_types::PrefixedArray;

pub struct MeloncraftInitPlayPlugin;

impl Plugin for MeloncraftInitPlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            sync_position,
            play_login,
        ).chain()); // reverse order so that the login packet is sent before the position sync
                    // otherwise bevy might read the messages the wrong way, so send position then login
    }
}

fn play_login(
    mut ack_finish_config_pr: MessageReader<AcknowledgeFinishConfiguration>,
    mut login_pw: MessageWriter<clientbound::play::Login>,
    mut client_connection_q: Query<(&mut ClientConnection, Entity)>,
) {
    for ack_packet in ack_finish_config_pr.read() {
        let mut client_connection = client_connection_q.get_mut(ack_packet.client).unwrap().0;
        client_connection.state = ConnectionState::Play;
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

fn sync_position(
    mut login_play_pr: MessageReader<clientbound::play::Login>,
    mut synchronize_player_position_pw: MessageWriter<clientbound::play::SynchronizePlayerPosition>,
) {
    for login_packet in login_play_pr.read() {
        synchronize_player_position_pw.write(clientbound::play::SynchronizePlayerPosition {
            client: login_packet.client,
            x: 42.0,
            y: 64.0,
            z: -42.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            teleport_id: 0,
        });
    }
}

