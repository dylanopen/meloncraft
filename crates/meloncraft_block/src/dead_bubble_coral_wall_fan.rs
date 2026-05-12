use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadBubbleCoralWallFan {
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

impl BlockState for DeadBubbleCoralWallFan {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 15007; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 15003; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 15005; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 15006; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 15004; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 15002; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 15008; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 15001; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15007 {
            return Some(DeadBubbleCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15003 {
            return Some(DeadBubbleCoralWallFan {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15005 {
            return Some(DeadBubbleCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 15006 {
            return Some(DeadBubbleCoralWallFan {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 15004 {
            return Some(DeadBubbleCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15002 {
            return Some(DeadBubbleCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15008 {
            return Some(DeadBubbleCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15001 {
            return Some(DeadBubbleCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

