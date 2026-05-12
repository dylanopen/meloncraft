use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceWallHangingSign {
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

impl BlockState for SpruceWallHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 6488; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 6486; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 6489; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 6484; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 6482; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 6485; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 6487; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 6483; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6488 {
            return Some(SpruceWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 6486 {
            return Some(SpruceWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 6489 {
            return Some(SpruceWallHangingSign {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 6484 {
            return Some(SpruceWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 6482 {
            return Some(SpruceWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 6485 {
            return Some(SpruceWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 6487 {
            return Some(SpruceWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 6483 {
            return Some(SpruceWallHangingSign {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

