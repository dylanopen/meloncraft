use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadBrainCoralWallFan {
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

impl BlockState for DeadBrainCoralWallFan {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 14994; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 14995; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 14996; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 14993; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 14997; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 14998; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 14999; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 15000; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14994 {
            return Some(DeadBrainCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 14995 {
            return Some(DeadBrainCoralWallFan {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 14996 {
            return Some(DeadBrainCoralWallFan {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 14993 {
            return Some(DeadBrainCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 14997 {
            return Some(DeadBrainCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 14998 {
            return Some(DeadBrainCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 14999 {
            return Some(DeadBrainCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 15000 {
            return Some(DeadBrainCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

