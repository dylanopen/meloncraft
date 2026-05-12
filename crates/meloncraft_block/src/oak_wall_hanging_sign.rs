use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakWallHangingSign {
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

impl BlockState for OakWallHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 6475; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 6478; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 6474; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 6476; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 6479; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 6481; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 6477; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 6480; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6475 {
            return Some(OakWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6478 {
            return Some(OakWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 6474 {
            return Some(OakWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 6476 {
            return Some(OakWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 6479 {
            return Some(OakWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 6481 {
            return Some(OakWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 6477 {
            return Some(OakWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6480 {
            return Some(OakWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

