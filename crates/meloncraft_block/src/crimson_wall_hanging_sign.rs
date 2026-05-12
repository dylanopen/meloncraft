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
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 6549; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 6550; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 6548; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 6553; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 6552; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 6546; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 6547; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 6551; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6549 {
            return Some(CrimsonWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6550 {
            return Some(CrimsonWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 6548 {
            return Some(CrimsonWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 6553 {
            return Some(CrimsonWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 6552 {
            return Some(CrimsonWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 6546 {
            return Some(CrimsonWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 6547 {
            return Some(CrimsonWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6551 {
            return Some(CrimsonWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

