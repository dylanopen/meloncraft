use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakWallHangingSign {
    pub r#facing: Facing,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for DarkOakWallHangingSign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#facing == Facing::South { return 6525; }
        if self.r#waterlogged == true && self.r#facing == Facing::West { return 6526; }
        if self.r#waterlogged == true && self.r#facing == Facing::North { return 6522; }
        if self.r#facing == Facing::South && self.r#waterlogged == true { return 6524; }
        if self.r#waterlogged == false && self.r#facing == Facing::West { return 6527; }
        if self.r#facing == Facing::North && self.r#waterlogged == false { return 6523; }
        if self.r#waterlogged == false && self.r#facing == Facing::East { return 6529; }
        if self.r#waterlogged == true && self.r#facing == Facing::East { return 6528; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6525 {
            return Some(DarkOakWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6526 {
            return Some(DarkOakWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 6522 {
            return Some(DarkOakWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 6524 {
            return Some(DarkOakWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 6527 {
            return Some(DarkOakWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 6523 {
            return Some(DarkOakWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 6529 {
            return Some(DarkOakWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 6528 {
            return Some(DarkOakWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

