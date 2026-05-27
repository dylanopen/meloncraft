//! Module for component struct [`BossbarTitle`].

use bevy::ecs::component::Component;
use meloncraft_text::NbtText;

/// The title component of the bossbar entity, as [`NbtText`].
/// This should be formattable.
#[derive(Component, Debug, Clone)]
pub struct BossbarTitle(pub NbtText);
