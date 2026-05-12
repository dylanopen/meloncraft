use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TubeCoralWallFan {
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

impl BlockState for TubeCoralWallFan {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 15026; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 15031; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 15025; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 15030; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 15029; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 15027; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 15032; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 15028; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15026 {
            return Some(TubeCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15031 {
            return Some(TubeCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 15025 {
            return Some(TubeCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 15030 {
            return Some(TubeCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15029 {
            return Some(TubeCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 15027 {
            return Some(TubeCoralWallFan {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15032 {
            return Some(TubeCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15028 {
            return Some(TubeCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

