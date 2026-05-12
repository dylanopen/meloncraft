use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadFireCoralWallFan {
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

impl BlockState for DeadFireCoralWallFan {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 15011; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 15010; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 15009; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 15012; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 15015; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 15016; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 15014; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 15013; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15011 {
            return Some(DeadFireCoralWallFan {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15010 {
            return Some(DeadFireCoralWallFan {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15009 {
            return Some(DeadFireCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 15012 {
            return Some(DeadFireCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15015 {
            return Some(DeadFireCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 15016 {
            return Some(DeadFireCoralWallFan {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 15014 {
            return Some(DeadFireCoralWallFan {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15013 {
            return Some(DeadFireCoralWallFan {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

