use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedWallHangingSign {
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

impl BlockState for WarpedWallHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 6557; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 6554; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 6556; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 6555; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 6558; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 6561; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 6560; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 6559; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6557 {
            return Some(WarpedWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6554 {
            return Some(WarpedWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 6556 {
            return Some(WarpedWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 6555 {
            return Some(WarpedWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6558 {
            return Some(WarpedWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 6561 {
            return Some(WarpedWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 6560 {
            return Some(WarpedWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 6559 {
            return Some(WarpedWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

