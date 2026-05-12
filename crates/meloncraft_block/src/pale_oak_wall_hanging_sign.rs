use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakWallHangingSign {
    pub waterlogged: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for PaleOakWallHangingSign {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#waterlogged == true { return 6534; }
        if self.r#facing == Facing::South && self.r#waterlogged == true { return 6532; }
        if self.r#facing == Facing::West && self.r#waterlogged == false { return 6535; }
        if self.r#facing == Facing::North && self.r#waterlogged == true { return 6530; }
        if self.r#facing == Facing::North && self.r#waterlogged == false { return 6531; }
        if self.r#facing == Facing::East && self.r#waterlogged == true { return 6536; }
        if self.r#waterlogged == false && self.r#facing == Facing::South { return 6533; }
        if self.r#waterlogged == false && self.r#facing == Facing::East { return 6537; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6534 {
            return Some(PaleOakWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 6532 {
            return Some(PaleOakWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 6535 {
            return Some(PaleOakWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 6530 {
            return Some(PaleOakWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 6531 {
            return Some(PaleOakWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 6536 {
            return Some(PaleOakWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 6533 {
            return Some(PaleOakWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6537 {
            return Some(PaleOakWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

