use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BubbleCoralWallFan {
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

impl BlockState for BubbleCoralWallFan {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 15044; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 15046; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 15048; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 15041; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 15045; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 15043; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 15047; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 15042; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15044 {
            return Some(BubbleCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15046 {
            return Some(BubbleCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 15048 {
            return Some(BubbleCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15041 {
            return Some(BubbleCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 15045 {
            return Some(BubbleCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 15043 {
            return Some(BubbleCoralWallFan {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15047 {
            return Some(BubbleCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15042 {
            return Some(BubbleCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

