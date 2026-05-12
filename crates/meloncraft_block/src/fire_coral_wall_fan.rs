use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FireCoralWallFan {
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

impl BlockState for FireCoralWallFan {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 15050; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 15049; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 15056; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 15053; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 15051; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 15054; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 15052; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 15055; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15050 {
            return Some(FireCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15049 {
            return Some(FireCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 15056 {
            return Some(FireCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 15053 {
            return Some(FireCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 15051 {
            return Some(FireCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 15054 {
            return Some(FireCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 15052 {
            return Some(FireCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15055 {
            return Some(FireCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

