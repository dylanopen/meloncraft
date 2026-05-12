use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveWallHangingSign {
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

impl BlockState for MangroveWallHangingSign {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 6540; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 6543; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 6545; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 6542; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 6538; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 6541; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 6539; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 6544; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6540 {
            return Some(MangroveWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 6543 {
            return Some(MangroveWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 6545 {
            return Some(MangroveWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 6542 {
            return Some(MangroveWallHangingSign {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 6538 {
            return Some(MangroveWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 6541 {
            return Some(MangroveWallHangingSign {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 6539 {
            return Some(MangroveWallHangingSign {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6544 {
            return Some(MangroveWallHangingSign {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

