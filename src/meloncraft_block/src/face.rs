pub enum BlockFaceType {
    Bottom,
    Top,
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
