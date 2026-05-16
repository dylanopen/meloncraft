use bevy::app::{App, Plugin, Update};
use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::schedule::IntoScheduleConfigs as _;
use bevy::ecs::system::Query;
use meloncraft_block::BlockState;
use meloncraft_block::dirt::Dirt;
use meloncraft_chunk::block::Block;
use meloncraft_chunk::Chunk;
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::Identifier;
use meloncraft_packets::{ClientboundGameEvent, ClientboundPlayLogin, ClientboundPlayerInfoUpdate, ClientboundSetCenterChunk, ClientboundSynchronizePlayerPosition};
use meloncraft_packets::ServerboundAcknowledgeFinishConfiguration;
use meloncraft_player::GameProfile;
use meloncraft_protocol_types::{AddPlayerAction, PlayerAction, PrefixedArray};
use meloncraft_world::messages::SendChunk;

pub struct MeloncraftInitPlayPlugin;

impl Plugin for MeloncraftInitPlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            send_chunks,
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

fn game_event_player_info_update(
    mut login_play_pr: MessageReader<ClientboundPlayLogin>,
    mut player_info_update_pw: MessageWriter<ClientboundPlayerInfoUpdate>,
    mut game_event_pw: MessageWriter<ClientboundGameEvent>,
    client_profile_q: Query<&GameProfile>
) {
    for login_packet in login_play_pr.read() {
        let profile = client_profile_q.get(login_packet.client).unwrap();
        player_info_update_pw.write(ClientboundPlayerInfoUpdate {
            client: login_packet.client,
            action_mask: 0x01,
            players: vec![
                (
                    profile.uuid.clone(),
                    vec![
                        PlayerAction::AddPlayer(AddPlayerAction {
                            name: profile.username.clone(),
                            game_profile_properties: vec![],
                        }),
                    ],
                ),
            ],
        });
        game_event_pw.write(ClientboundGameEvent {
            client: login_packet.client,
            event: meloncraft_protocol_types::GameEventType::WaitForChunks,
        });
    }
}

fn send_chunks(
    mut game_event_pr: MessageReader<ClientboundGameEvent>,
    mut send_chunk_mw: MessageWriter<SendChunk>,
    mut set_center_chunk_pw: MessageWriter<ClientboundSetCenterChunk>,
) {
    for packet in game_event_pr.read() {
        set_center_chunk_pw.write(ClientboundSetCenterChunk {
            client: packet.client,
            x: 0,
            z: 0,
        });

        for z in -10..=10 {
            for x in -10..=10 {
                let chunk = Chunk::new(vec![Block::new(Dirt{}.to_id()); 16*16*384]);
                send_chunk_mw.write(SendChunk {
                    client: packet.client,
                    chunk_x: x,
                    chunk_z: z,
                    chunk,
                });
            }
        }
    }
}
