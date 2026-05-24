use bevy::app::{App, Plugin, Update};
use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::schedule::IntoScheduleConfigs as _;
use bevy::ecs::system::{Commands, Query};
use bevy::math::{DVec3, IVec2};
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::Identifier;
use meloncraft_packets::{ClientboundGameEvent, ClientboundPlayLogin, ClientboundSetCenterChunk, ClientboundSynchronizePlayerPosition};
use meloncraft_packets::ServerboundAcknowledgeFinishConfiguration;
use meloncraft_player::PlayerMarker;
use meloncraft_protocol_types::PrefixedArray;
use meloncraft_world::messages::ChunkRequest;

pub struct MeloncraftInitPlayPlugin;

impl Plugin for MeloncraftInitPlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            send_chunks,
            add_player_marker,
            game_event_player_info_update,
            sync_position, // reverse order so that the login packet is sent before the position sync
            play_login,    // otherwise bevy might read the messages the wrong way, so send position then login
        ).chain());
    }
}

fn play_login(
    mut ack_finish_config_pr: MessageReader<ServerboundAcknowledgeFinishConfiguration>,
    mut login_pw: MessageWriter<ClientboundPlayLogin>,
    mut client_connection_q: Query<(&mut ClientConnection, Entity)>,
) {
    for ack_packet in ack_finish_config_pr.read() {
        let mut client_connection = client_connection_q.get_mut(ack_packet.client).unwrap().0;
        client_connection.state = ConnectionState::Play;
        login_pw.write(ClientboundPlayLogin {
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
    mut login_play_pr: MessageReader<ClientboundPlayLogin>,
    mut synchronize_player_position_pw: MessageWriter<ClientboundSynchronizePlayerPosition>,
) {
    for login_packet in login_play_pr.read() {
        synchronize_player_position_pw.write(ClientboundSynchronizePlayerPosition {
            client: login_packet.client,
            position: DVec3::new(42.0, 64.0, -42.0),
            velocity: DVec3::ZERO,
            yaw: 0.0,
            pitch: 0.0,
            teleport_id: 0,
        });
    }
}

fn add_player_marker(
    mut commands: Commands,
    mut login_play_pr: MessageReader<ClientboundPlayLogin>,
) {
    for login_packet in login_play_pr.read() {
        commands.get_entity(login_packet.client).unwrap().insert(PlayerMarker);
    }
}

fn game_event_player_info_update(
    mut login_play_pr: MessageReader<ClientboundPlayLogin>,
    mut game_event_pw: MessageWriter<ClientboundGameEvent>,
) {
    for login_packet in login_play_pr.read() {
        game_event_pw.write(ClientboundGameEvent {
            client: login_packet.client,
            event: meloncraft_protocol_types::GameEventType::WaitForChunks,
        });
    }
}

fn send_chunks(
    mut game_event_pr: MessageReader<ClientboundGameEvent>,
    mut chunk_request_mw: MessageWriter<ChunkRequest>,
    mut set_center_chunk_pw: MessageWriter<ClientboundSetCenterChunk>,
) {
    for packet in game_event_pr.read() {
        set_center_chunk_pw.write(ClientboundSetCenterChunk {
            client: packet.client,
            chunk_pos: IVec2::new(0, 0),
        });

        for z in -10..=10 {
            for x in -10..=10 {
                chunk_request_mw.write(ChunkRequest {
                    client: packet.client,
                    chunk_pos: IVec2::new(x, z),
                });
            }
        }
    }
}
