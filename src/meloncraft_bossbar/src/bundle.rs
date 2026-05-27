//! Module for bundle struct [`BossbarBundle`].

use bevy::ecs::bundle::Bundle;
use meloncraft_entity::health::current::CurrentHealth;

use crate::color::BossbarColor;
use crate::division::BossbarDivision;
use crate::flags::{BossbarCreatesFog, BossbarDarkensSky, BossbarIsDragon};
use crate::marker::BossbarMarker;
use crate::title::BossbarTitle;

#[derive(Bundle, Debug, Clone)]
pub struct BossbarBundle {
    pub marker: BossbarMarker,
    pub health: CurrentHealth,
    pub title: BossbarTitle,
    pub color: BossbarColor,
    pub division: BossbarDivision,
    pub darkens_sky: BossbarDarkensSky,
    pub is_dragon: BossbarIsDragon,
    pub creates_fog: BossbarCreatesFog,
}
