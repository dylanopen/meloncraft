use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaWallHangingSign {
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

impl BlockState for AcaciaWallHangingSign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#facing == Facing::North {
            return 6499;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::West {
            return 6503;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == true {
            return 6502;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == false {
            return 6505;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::South {
            return 6501;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::East {
            return 6504;
        }
        if self.r#facing == Facing::South && self.r#waterlogged == true {
            return 6500;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == true {
            return 6498;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6499 {
            return Some(AcaciaWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6503 {
            return Some(AcaciaWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 6502 {
            return Some(AcaciaWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 6505 {
            return Some(AcaciaWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 6501 {
            return Some(AcaciaWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6504 {
            return Some(AcaciaWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 6500 {
            return Some(AcaciaWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 6498 {
            return Some(AcaciaWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
