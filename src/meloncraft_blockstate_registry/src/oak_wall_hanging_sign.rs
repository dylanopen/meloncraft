use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakWallHangingSign {
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

impl BlockState for OakWallHangingSign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#facing == Facing::West {
            return 6479;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::West {
            return 6478;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::South {
            return 6476;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::North {
            return 6474;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::East {
            return 6480;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::North {
            return 6475;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::South {
            return 6477;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == false {
            return 6481;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6479 {
            return Some(OakWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 6478 {
            return Some(OakWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 6476 {
            return Some(OakWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 6474 {
            return Some(OakWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 6480 {
            return Some(OakWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 6475 {
            return Some(OakWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6477 {
            return Some(OakWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6481 {
            return Some(OakWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
