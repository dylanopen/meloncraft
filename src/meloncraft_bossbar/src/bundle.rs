//! Module for bundle struct [`BossbarBundle`].

use bevy::ecs::bundle::Bundle;
use meloncraft_entity::health::current::CurrentHealth;

use crate::marker::BossbarMarker;

#[derive(Bundle, Debug, Clone)]
pub struct BossbarBundle {
    pub marker: BossbarMarker,
    pub health: CurrentHealth,
}
