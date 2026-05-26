use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::Changed;
use bevy::ecs::system::Query;
use meloncraft_packets::ClientboundSetExperience;
use meloncraft_player::experience::Experience;

pub fn send_player_experience(
    player_q: Query<(Entity, &Experience), Changed<Experience>>,
    mut set_experience_pw: MessageWriter<ClientboundSetExperience>,
) {
    for (entity, experience) in player_q {
        set_experience_pw.write(ClientboundSetExperience { client: entity, total: *experience, level: None, bar: None });
    }
}
