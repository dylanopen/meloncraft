use bevy::ecs::bundle::Bundle;
use meloncraft_core::GameMode;
use meloncraft_entity::health::current::CurrentHealth;
use meloncraft_entity::health::food::{FoodHealth, FoodSaturation};
use meloncraft_entity::position::EntityPosition;

use crate::experience::Experience;
use crate::marker::LoadedPlayer;
use crate::{CanFly, CanInstantBreak, FlySpeed, FovModifier, Invulnerable, IsFlying};

/// A bundle to represent a player who has just joined *and loaded into* the world.
/// The components of this bundle is added to the existing client's entity, a new entity is not
/// made.
///
/// ## Example
/// Listen for players which have just joined (just had a [`PlayerMarker`] added)
/// ```rust
/// fn player_joined(
///     mut commands: Commands,
///     player_q: Query<Entity, Added<PlayerMarker>>,
///     world_q: Query<Entity, With<Overworld>>,
/// ) {
///     let world = world_q.single().unwrap();
///     for player in player_q {
///         commands
///             .entity(player)
///             .insert(LoadedPlayerBundle::new(EntityPosition {
///                 location: DVec3::new(0.0, 0.0, 0.0),
///                 world,
///                 flags: EntityPositionFlags::default(),
///             }));
///     }
/// }
/// ```
#[derive(Bundle)]
pub struct LoadedPlayerBundle {
    pub loaded_player: LoadedPlayer,
    pub entity_position: EntityPosition,
    pub is_flying: IsFlying,
    pub can_fly: CanFly,
    pub invulnerable: Invulnerable,
    pub can_instant_break: CanInstantBreak,
    pub fly_speed: FlySpeed,
    pub fov_modifier: FovModifier,
    pub experience: Experience,
    pub current_health: CurrentHealth,
    pub food_health: FoodHealth,
    pub food_saturation: FoodSaturation,
    pub gamemode: GameMode,
}

impl LoadedPlayerBundle {
    #[must_use]
    pub const fn new(position: EntityPosition) -> LoadedPlayerBundle {
        return LoadedPlayerBundle {
            loaded_player: LoadedPlayer,
            entity_position: position,
            is_flying: IsFlying(false),
            can_fly: CanFly(false),
            invulnerable: Invulnerable(false),
            can_instant_break: CanInstantBreak(false),
            fly_speed: FlySpeed(0.05),
            fov_modifier: FovModifier(0.1),
            experience: Experience(0),
            current_health: CurrentHealth(20.0),
            food_health: FoodHealth(20),
            food_saturation: FoodSaturation(5.0),
            gamemode: GameMode::Creative,
        };
    }

    #[must_use]
    pub const fn with_fly_speed(mut self, fly_speed: f32) -> LoadedPlayerBundle {
        self.fly_speed = FlySpeed(fly_speed);
        return self;
    }

    #[must_use]
    pub const fn with_fov_modifier(mut self, fov_modifier: f32) -> LoadedPlayerBundle {
        self.fov_modifier = FovModifier(fov_modifier);
        return self;
    }

    #[must_use]
    pub const fn with_can_fly(mut self, can_fly: bool) -> LoadedPlayerBundle {
        self.can_fly = CanFly(can_fly);
        return self;
    }

    #[must_use]
    pub const fn with_is_flying(mut self, is_flying: bool) -> LoadedPlayerBundle {
        self.is_flying = IsFlying(is_flying);
        return self;
    }

    #[must_use]
    pub const fn with_invulnerable(mut self, invulnerable: bool) -> LoadedPlayerBundle {
        self.invulnerable = Invulnerable(invulnerable);
        return self;
    }

    #[must_use]
    pub const fn with_can_instant_break(mut self, can_instant_break: bool) -> LoadedPlayerBundle {
        self.can_instant_break = CanInstantBreak(can_instant_break);
        return self;
    }

    #[must_use]
    pub const fn with_experience(mut self, experience: i32) -> LoadedPlayerBundle {
        self.experience = Experience(experience);
        return self;
    }

    #[must_use]
    pub const fn with_entity_position(
        mut self,
        entity_position: EntityPosition,
    ) -> LoadedPlayerBundle {
        self.entity_position = entity_position;
        return self;
    }
}
