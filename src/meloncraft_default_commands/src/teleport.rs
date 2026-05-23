//! Module for `/teleport` and `/tp` command.
//! See [`cmd_teleport`].

use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Query;
use bevy::math::DVec3;
use meloncraft_command::raw::RawCommand;
use meloncraft_entity::position::EntityPosition;
use meloncraft_entity::position::flags::EntityPositionFlags;
use meloncraft_entity::position::teleport::TeleportEntity;
use meloncraft_player::GameProfile;

pub fn cmd_teleport(
    mut raw_command_mr: MessageReader<RawCommand>,
    player_profile_q: Query<(Entity, &GameProfile)>,
    mut teleport_entity_mw: MessageWriter<TeleportEntity>,
) {
    for raw_command in raw_command_mr.read() {
        if raw_command.name != "teleport" && raw_command.name != "tp" {
            return; // we don't care about other commands
        }
        if raw_command.args.len() != 4 {
            print_usage(raw_command.executor);
        }
        // Obviously these unwraps aren't going to stay (the ones for parsing).. They are purely temporary.
        let player_name = raw_command.args.first().unwrap();
        let new_x = raw_command.args.get(1).unwrap().parse().unwrap();
        let new_y = raw_command.args.get(2).unwrap().parse().unwrap();
        let new_z = raw_command.args.get(3).unwrap().parse().unwrap();

        let entity = get_player_entity_by_name(player_name, player_profile_q).unwrap();
        let new_position = EntityPosition {
            location: DVec3::new(new_x, new_y, new_z),
            flags: EntityPositionFlags { on_ground: false, pushing_against_wall: false },
        };

        teleport_entity_mw.write(TeleportEntity {
            entity,
            new_position,
        });
    }
}

fn get_player_entity_by_name(
    player_name: &str,
    player_profile_q: Query<(Entity, &GameProfile)>,
) -> Option<Entity> {
    for (entity, profile) in player_profile_q.iter() {
        if profile.username == player_name {
            return Some(entity);
        }
    }
    return None;
}

#[expect(clippy::todo, reason = "Will soon be replaced.")]
pub fn print_usage(_receiver: Entity) {
    todo!("Send client chat message explaining the /tp command");
}
