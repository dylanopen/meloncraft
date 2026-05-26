//! Forwarder for player's abilities, see [`meloncraft_player::ability`].

use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::{Changed, Or};
use bevy::ecs::system::Query;
use meloncraft_packets::ClientboundPlayerAbilities;
use meloncraft_player::{CanFly, CanInstantBreak, FlySpeed, FovModifier, Invulnerable, IsFlying};

type AbilityChanged = Or<(
    Changed<Invulnerable>,
    Changed<IsFlying>,
    Changed<CanFly>,
    Changed<CanInstantBreak>,
    Changed<FlySpeed>,
    Changed<FovModifier>,
)>;

#[expect(clippy::type_complexity, reason = "Having it here in the method signature makes it much more clear what we're querying for.")]
pub fn send_player_abilities(
    player_q: Query<(
        Entity,
        &Invulnerable,
        &IsFlying,
        &CanFly,
        &CanInstantBreak,
        &FlySpeed,
        &FovModifier,
    ), AbilityChanged>,
    mut player_abilities_pw: MessageWriter<ClientboundPlayerAbilities>,
) {
    for (client, invulnerable, is_flying, can_fly, can_instant_break, fly_speed, fov_modifier) in player_q {
        player_abilities_pw.write(ClientboundPlayerAbilities { client, invulnerable: *invulnerable, is_flying: *is_flying, allow_flying: *can_fly, instant_break: *can_instant_break, fly_speed: *fly_speed, fov_modifier: *fov_modifier });
    }
}
