//! Module for enum [`BlockFaceType`].

/// The face of a block, often that is being interacted with.
///
/// This is defined in absolute directions, e.g. [`BlockFaceType::North`] is always the north face
/// of a block, it doesn't depend on the client's position or anything like that.
///
/// ## Packet usage
/// Clients send this when breaking or placing blocks, among other interactions.
/// They are usually represented as a `u8` in packets. See the individual variants for the value
/// associated with each face.
#[derive(Debug, Clone)]
pub enum BlockFaceType {

    /// - The bottom face of a block.
    /// - Opposite to the [`BlockFaceType::Top`] face.
    /// - This is the closest face to the negative Y direction - closest to the void at the bottom
    ///   of the world.
    /// - **Offset: `-Y`.**
    /// - **ID in packets: `0`**.
    Bottom,

    /// - The top face of a block.
    /// - Opposite to the [`BlockFaceType::Bottom`] face.
    /// - This is the closest face to the positive Y direction - closest to the sky at the top of the world.
    /// - **Offset: `+Y`.**
    /// - **ID in packets: `1`**.
    Top,

    /// - The north face of a block.
    /// - Opposite to the [`BlockFaceType::South`] face.
    /// - This is the closest face to the negative Z direction.
    /// - **Offset: `-Z`.**
    /// - **ID in packets: `2`**.
    North,

    South,

    West,

    East,
}

impl From<BlockFaceType> for u8 {
    fn from(value: BlockFaceType) -> Self {
        return match value {
            BlockFaceType::Bottom => 0,
            BlockFaceType::Top => 1,
            BlockFaceType::North => 2,
            BlockFaceType::South => 3,
            BlockFaceType::West => 4,
            BlockFaceType::East => 5,
        }
    }
}

impl TryFrom<u8> for BlockFaceType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(Self::Bottom),
            1 => Ok(Self::Top),
            2 => Ok(Self::North),
            3 => Ok(Self::South),
            4 => Ok(Self::West),
            5 => Ok(Self::East),
            _ => Err(()),
        }
    }
}
