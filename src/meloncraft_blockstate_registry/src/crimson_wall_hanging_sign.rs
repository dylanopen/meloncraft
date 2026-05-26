use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonWallHangingSign {
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

impl BlockState for CrimsonWallHangingSign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#facing == Facing::East {
            return 6552;
        }
        if self.r#facing == Facing::South && self.r#waterlogged == false {
            return 6549;
        }
        if self.r#facing == Facing::South && self.r#waterlogged == true {
            return 6548;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::North {
            return 6546;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == false {
            return 6547;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == false {
            return 6553;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::West {
            return 6551;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == true {
            return 6550;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6552 {
            return Some(CrimsonWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 6549 {
            return Some(CrimsonWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 6548 {
            return Some(CrimsonWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 6546 {
            return Some(CrimsonWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 6547 {
            return Some(CrimsonWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 6553 {
            return Some(CrimsonWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 6551 {
            return Some(CrimsonWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 6550 {
            return Some(CrimsonWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
