use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadTubeCoralWallFan {
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

impl BlockState for DeadTubeCoralWallFan {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 14985; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 14986; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 14987; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 14989; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 14990; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 14992; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 14991; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 14988; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14985 {
            return Some(DeadTubeCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 14986 {
            return Some(DeadTubeCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 14987 {
            return Some(DeadTubeCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14989 {
            return Some(DeadTubeCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 14990 {
            return Some(DeadTubeCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 14992 {
            return Some(DeadTubeCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 14991 {
            return Some(DeadTubeCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 14988 {
            return Some(DeadTubeCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

