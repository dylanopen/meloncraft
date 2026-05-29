//! Module for struct [`EntityPositionFlags`].

/// Stores extra metadata about the player's position. Some packets require these flags to be sent
/// alongside the player's location.
///
/// ## Flags
/// - [`EntityPositionFlags::on_ground`]
/// - [`EntityPositionFlags::pushing_against_wall`]
///
/// ## Serialization
/// [`From`] is implemented bidirectionally between [`EntityPositionFlags`] and `u8`. It is
/// converted into a bitset of values: see the individual fields for information about how they are
/// serialized and deserialized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityPositionFlags {
    /// `true` if the entity's feet are touching the ground, `false` otherwise.
    /// This is represented by the **least significant bit**, `0b_0000_0001`.
    pub on_ground: bool,

    /// `true` if the entity is touching / moving into a wall, `false` otherwise.
    /// This is represented by the **second to least significant bit**, `0b_0000_0010`.
    pub pushing_against_wall: bool,
}

impl Default for EntityPositionFlags {
    fn default() -> Self {
        // these defaults may change in the future
        return EntityPositionFlags {
            on_ground: true,
            pushing_against_wall: false,
        };
    }
}

impl From<u8> for EntityPositionFlags {
    fn from(value: u8) -> Self {
        return Self {
            on_ground: value & 0b_0000_0001 != 0,
            pushing_against_wall: value & 0b_0000_0010 != 0,
        };
    }
}

impl From<EntityPositionFlags> for u8 {
    fn from(value: EntityPositionFlags) -> Self {
        let mut result = 0;
        if value.on_ground {
            result |= 0b_0000_0001;
        }
        if value.pushing_against_wall {
            result |= 0b_0000_0010;
        }
        return result;
    }
}
