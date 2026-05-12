use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaWallHangingSign {
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

impl BlockState for AcaciaWallHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 6503; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 6500; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 6498; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 6501; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 6502; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 6499; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 6504; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 6505; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6503 {
            return Some(AcaciaWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 6500 {
            return Some(AcaciaWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 6498 {
            return Some(AcaciaWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 6501 {
            return Some(AcaciaWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6502 {
            return Some(AcaciaWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 6499 {
            return Some(AcaciaWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 6504 {
            return Some(AcaciaWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 6505 {
            return Some(AcaciaWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

