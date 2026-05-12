use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrainCoralWallFan {
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

impl BlockState for BrainCoralWallFan {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 15038; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 15036; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 15037; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 15040; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 15039; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 15033; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 15034; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 15035; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15038 {
            return Some(BrainCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15036 {
            return Some(BrainCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15037 {
            return Some(BrainCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 15040 {
            return Some(BrainCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15039 {
            return Some(BrainCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15033 {
            return Some(BrainCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 15034 {
            return Some(BrainCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15035 {
            return Some(BrainCoralWallFan {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

