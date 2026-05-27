//! Module for bundle struct [`BossbarBundle`].

use bevy::ecs::bundle::Bundle;
use meloncraft_entity::health::current::CurrentHealth;
use meloncraft_nbt::NbtString;
use meloncraft_text::NbtText;

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

impl Default for BossbarBundle {
    fn default() -> Self {
        return BossbarBundle {
            marker: BossbarMarker,
            health: CurrentHealth(1.0),
            title: BossbarTitle(NbtText::Plain(NbtString("Bossbar".to_owned()))),
            color: BossbarColor::Red,
            division: BossbarDivision::DividedInto6,
            darkens_sky: BossbarDarkensSky(false),
            is_dragon: BossbarIsDragon(false),
            creates_fog: BossbarCreatesFog(false),
        };
    }
}

impl BossbarBundle {
    #[must_use]
    pub fn with_title(mut self, title: BossbarTitle) -> BossbarBundle {
        self.title = title;
        return self;
    }

    #[must_use]
    pub const fn with_health(mut self, health: CurrentHealth) -> BossbarBundle {
        self.health = health;
        return self;
    }

    #[must_use]
    pub const fn with_color(mut self, color: BossbarColor) -> BossbarBundle {
        self.color = color;
        return self;
    }
}
