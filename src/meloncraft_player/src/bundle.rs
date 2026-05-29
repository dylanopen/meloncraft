use bevy::ecs::bundle::Bundle;
use meloncraft_entity::position::EntityPosition;

use crate::experience::Experience;
use crate::marker::LoadedPlayer;
use crate::{CanFly, CanInstantBreak, FlySpeed, FovModifier, Invulnerable, IsFlying};

#[derive(Bundle)]
pub struct LoadedPlayerBundle {
    pub is_flying: IsFlying,
    pub can_fly: CanFly,
    pub invulnerable: Invulnerable,
    pub can_instant_break: CanInstantBreak,
    pub fly_speed: FlySpeed,
    pub fov_modifier: FovModifier,
    pub experience: Experience,
    pub loaded_player: LoadedPlayer,
    pub entity_position: EntityPosition,
}
